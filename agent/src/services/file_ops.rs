//! File Operations - upload/download handlers for built-in commands

use crate::protocol::AgentMessage;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::Serialize;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::mpsc::Sender;
use std::time::UNIX_EPOCH;

/// Maximum file size (in bytes) allowed for download — 50 MB.
const MAX_DOWNLOAD_BYTES: u64 = 50 * 1024 * 1024;

/// Maximum decoded payload size (in bytes) allowed for upload — 50 MB.
const MAX_UPLOAD_BYTES: u64 = 50 * 1024 * 1024;

/// Maximum number of entries returned by a single `browse` call.
const MAX_LIST_ENTRIES: usize = 500;

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

    // Estimate decoded size before decoding: base64 expands ~4/3
    let estimated_decoded_len = (parsed.content_base64.len() as u64 * 3) / 4;
    if estimated_decoded_len > MAX_UPLOAD_BYTES {
        let _ = sender.send(fail(
            task_id,
            format!(
                "文件过大，超过 {}MB 上传限制",
                MAX_UPLOAD_BYTES / (1024 * 1024)
            ),
        ));
        return;
    }

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

/// Handle `browse` command: list directory entries and return JSON metadata
pub fn handle_browse(task_id: &str, payload: &str, sender: &Sender<AgentMessage>) {
    #[derive(serde::Deserialize)]
    struct BrowsePayload {
        path: String,
    }

    let parsed: BrowsePayload = match serde_json::from_str(payload) {
        Ok(p) => p,
        Err(e) => {
            let _ = sender.send(fail(task_id, format!("invalid payload: {e}")));
            return;
        }
    };

    let result = browse_dir(&parsed.path);
    let _ = sender.send(match result {
        Ok(entries) => match serde_json::to_string(&entries) {
            Ok(output) => AgentMessage::TaskResult {
                task_id: task_id.to_string(),
                success: true,
                output,
            },
            Err(e) => fail(task_id, format!("serialize failed: {e}")),
        },
        Err(e) => fail(task_id, e),
    });
}

/// Check if a command is a built-in file operation
pub fn is_file_op(command: &str) -> bool {
    matches!(command, "upload" | "download" | "browse")
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
    let metadata = fs::metadata(path).map_err(|e| format!("metadata failed: {e}"))?;
    if metadata.len() > MAX_DOWNLOAD_BYTES {
        return Err(format!(
            "文件过大，超过 {}MB 限制",
            MAX_DOWNLOAD_BYTES / (1024 * 1024)
        ));
    }
    let mut file = fs::File::open(path).map_err(|e| format!("open failed: {e}"))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)
        .map_err(|e| format!("read failed: {e}"))?;
    Ok(buf)
}

fn browse_dir(path: &str) -> Result<Vec<BrowseEntry>, String> {
    let mut entries = Vec::new();
    let dir = fs::read_dir(path).map_err(|e| format!("read_dir failed: {e}"))?;

    for entry in dir {
        if entries.len() >= MAX_LIST_ENTRIES {
            break;
        }
        let entry = entry.map_err(|e| format!("read_dir entry failed: {e}"))?;
        let metadata = entry
            .metadata()
            .map_err(|e| format!("metadata failed: {e}"))?;
        let modified = metadata
            .modified()
            .ok()
            .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
            .map(|duration| duration.as_secs())
            .unwrap_or(0);

        entries.push(BrowseEntry {
            name: entry.file_name().to_string_lossy().into_owned(),
            is_dir: metadata.is_dir(),
            size: if metadata.is_dir() { 0 } else { metadata.len() },
            modified,
        });
    }

    Ok(entries)
}

#[derive(Serialize)]
struct BrowseEntry {
    name: String,
    is_dir: bool,
    size: u64,
    modified: u64,
}

fn fail(task_id: &str, detail: String) -> AgentMessage {
    AgentMessage::TaskResult {
        task_id: task_id.to_string(),
        success: false,
        output: detail,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn browse_is_treated_as_file_op() {
        assert!(is_file_op("browse"));
    }

    #[test]
    fn handle_browse_lists_directory_entries_as_json() {
        let dir = unique_temp_dir("browse-success");
        let subdir = dir.join("nested");
        std::fs::create_dir_all(&subdir).unwrap();
        let file_path = dir.join("note.txt");
        std::fs::write(&file_path, b"hello").unwrap();

        let (sender, receiver) = mpsc::channel();
        let payload = serde_json::json!({ "path": dir.to_string_lossy() }).to_string();

        handle_browse("task-1", &payload, &sender);

        let message = receiver.recv().unwrap();
        let AgentMessage::TaskResult {
            task_id,
            success,
            output,
        } = message
        else {
            panic!("expected task result");
        };

        assert_eq!(task_id, "task-1");
        assert!(success);

        let entries: serde_json::Value = serde_json::from_str(&output).unwrap();
        let entries = entries.as_array().unwrap();
        assert!(entries.iter().any(|entry| {
            entry["name"] == "nested"
                && entry["is_dir"] == true
                && entry["size"] == 0
                && entry["modified"].as_u64().is_some()
        }));
        assert!(entries.iter().any(|entry| {
            entry["name"] == "note.txt"
                && entry["is_dir"] == false
                && entry["size"] == 5
                && entry["modified"].as_u64().is_some()
        }));

        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn handle_browse_reports_invalid_payload() {
        let (sender, receiver) = mpsc::channel();

        handle_browse("task-2", "{not-json}", &sender);

        let AgentMessage::TaskResult {
            task_id,
            success,
            output,
        } = receiver.recv().unwrap()
        else {
            panic!("expected task result");
        };

        assert_eq!(task_id, "task-2");
        assert!(!success);
        assert!(output.contains("invalid payload"));
    }

    fn unique_temp_dir(prefix: &str) -> std::path::PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("hermes-{prefix}-{unique}"));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }
}

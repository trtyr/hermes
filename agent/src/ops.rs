//! Operations - local command execution helpers

use std::process::{Command, Stdio};
use std::time::Duration;

#[derive(Clone)]
pub struct AgentConfig {
    pub agent_id: String,
    pub hostname: String,
    pub username: Option<String>,
    pub pid: u32,
    pub max_output_chars: usize,
    pub max_list_entries: usize,
    pub command_timeout_secs: u64,
}

pub fn execute_operation(
    command: &str,
    payload: Option<&str>,
    cfg: &AgentConfig,
) -> (bool, String) {
    let cmd = match payload {
        Some(p) if !p.trim().is_empty() => format!("{} {}", command.trim(), p.trim()),
        _ => command.trim().to_string(),
    };

    if cmd.is_empty() {
        return (true, String::new());
    }

    exec_cmd(&cmd, None, cfg.command_timeout_secs)
}

pub fn build_operation_command(command: &str, payload: Option<&str>) -> String {
    match payload {
        Some(p) if !p.trim().is_empty() => format!("{} {}", command.trim(), p.trim()),
        _ => command.trim().to_string(),
    }
}

pub fn spawn_operation(
    command: &str,
    payload: Option<&str>,
    cwd: Option<&str>,
) -> std::io::Result<std::process::Child> {
    let command = build_operation_command(command, payload);
    if command.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "empty command",
        ));
    }
    spawn_shell_process(&command, cwd)
}

pub fn execute_shell(command: &str, cwd: Option<&str>, timeout_secs: u64) -> (bool, String) {
    exec_cmd(command, cwd, timeout_secs)
}

pub fn terminate_process(pid: u32) -> bool {
    Command::new("taskkill")
        .args(["/PID", &pid.to_string(), "/T", "/F"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn exec_cmd(command: &str, cwd: Option<&str>, timeout_secs: u64) -> (bool, String) {
    let child = match spawn_shell_process(command, cwd) {
        Ok(c) => c,
        Err(_) => return (false, "exec failed".to_string()),
    };

    let output = match wait_child(child, timeout_secs) {
        ChildResult::Finished(output) => output,
        ChildResult::TimedOut => {
            return (false, format!("command timed out after {timeout_secs}s"));
        }
        ChildResult::Failed => {
            return (false, "exec failed".to_string());
        }
    };

    let code = output.status.code().unwrap_or(-1);
    let out = decode_output(&output.stdout);
    let err = decode_output(&output.stderr);
    let merged = if err.trim().is_empty() {
        out
    } else if out.trim().is_empty() {
        err
    } else {
        format!("{out}\n{err}")
    };
    let trimmed = merged.trim();
    (code == 0, trimmed.to_string())
}

fn windows_shell_command(command: &str) -> String {
    // No chcp prefix — let commands run in native code page (e.g. GBK/936 on Chinese Windows).
    // The decode_output() function handles encoding conversion properly.
    command.to_string()
}

fn spawn_shell_process(command: &str, cwd: Option<&str>) -> std::io::Result<std::process::Child> {
    let mut process = Command::new("cmd");
    let command = windows_shell_command(command);
    process.args(["/C", &command]);
    if let Some(cwd) = cwd {
        process.current_dir(cwd);
    }
    process
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
}

/// Default working directory - hardcoded fallback
pub fn default_cwd() -> String {
    std::env::current_dir()
        .map(|path| path.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "C:\\".to_string())
}

/// Decode raw bytes from a child process output.
/// Tries UTF-8 first; if the bytes are not valid UTF-8,
/// falls back to GBK (code page 936, the ANSI code page on Chinese Windows).
pub fn decode_output(raw: &[u8]) -> String {
    if raw.is_empty() {
        return String::new();
    }
    // Fast path: valid UTF-8
    if let Ok(s) = std::str::from_utf8(raw) {
        return s.to_string();
    }
    // Windows: try system ANSI code page (GBK/936 on Chinese systems)
    let (decoded, _encoding_used, _had_errors) = encoding_rs::GBK.decode(raw);
    decoded.to_string()
}

pub enum ChildResult {
    Finished(std::process::Output),
    TimedOut,
    Failed,
}

/// Wait for a child process with an optional timeout.
/// If timeout_secs is 0, waits indefinitely.
pub fn wait_child(child: std::process::Child, timeout_secs: u64) -> ChildResult {
    if timeout_secs == 0 {
        match child.wait_with_output() {
            Ok(output) => ChildResult::Finished(output),
            Err(_) => ChildResult::Failed,
        }
    } else {
        let pid = child.id();
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let result = child.wait_with_output();
            let _ = tx.send(result);
        });
        match rx.recv_timeout(Duration::from_secs(timeout_secs)) {
            Ok(Ok(output)) => ChildResult::Finished(output),
            Ok(Err(_)) => ChildResult::Failed,
            Err(_) => {
                // Timeout — kill the process
                terminate_process(pid);
                // Drain the result so the thread doesn't panic
                let _ = rx.recv();
                ChildResult::TimedOut
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_shell_command_returns_command_unchanged() {
        assert_eq!(
            windows_shell_command("dir /b"),
            "dir /b"
        );
    }

    #[test]
    fn decode_output_utf8_passthrough() {
        let input = "hello world".as_bytes();
        assert_eq!(decode_output(input), "hello world");
    }

    #[test]
    fn decode_output_empty() {
        assert_eq!(decode_output(&[]), "");
    }
}


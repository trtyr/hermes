//! Operations - local command execution helpers

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
    #[cfg(windows)]
    {
        use windows_sys::Win32::Foundation::CloseHandle;
        use windows_sys::Win32::System::Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE};
        unsafe {
            let handle = OpenProcess(PROCESS_TERMINATE, 0, pid);
            if handle.is_null() {
                return false;
            }
            let result = TerminateProcess(handle, 1);
            CloseHandle(handle);
            result != 0
        }
    }
    #[cfg(not(windows))]
    {
        let _ = pid;
        false
    }
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

fn spawn_shell_process(command: &str, cwd: Option<&str>) -> std::io::Result<std::process::Child> {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        let (program, args) = if command.starts_with('"') {
            if let Some(end) = command[1..].find('"') {
                let prog = &command[1..end + 1];
                let rest = command[end + 2..].trim();
                (prog.to_string(), rest.to_string())
            } else {
                (command.to_string(), String::new())
            }
        } else {
            match command.split_once(' ') {
                Some((prog, rest)) => (prog.to_string(), rest.to_string()),
                None => (command.to_string(), String::new()),
            }
        };
        let mut cmd = std::process::Command::new(&program);
        if !args.is_empty() {
            cmd.raw_arg(&args);
        }
        if let Some(cwd) = cwd {
            cmd.current_dir(cwd);
        }
        cmd.stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
        cmd.spawn()
    }
    #[cfg(not(windows))]
    {
        let mut cmd = std::process::Command::new("sh");
        cmd.arg("-c").arg(command);
        if let Some(cwd) = cwd {
            cmd.current_dir(cwd);
        }
        cmd.stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
    }
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
    fn decode_output_utf8_passthrough() {
        let input = "hello world".as_bytes();
        assert_eq!(decode_output(input), "hello world");
    }

    #[test]
    fn decode_output_empty() {
        assert_eq!(decode_output(&[]), "");
    }
}


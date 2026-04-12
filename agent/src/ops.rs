//! Operations - local command execution helpers

use std::process::{Command, Stdio};

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
    _cfg: &AgentConfig,
) -> (bool, String) {
    let cmd = match payload {
        Some(p) if !p.trim().is_empty() => format!("{} {}", command.trim(), p.trim()),
        _ => command.trim().to_string(),
    };

    if cmd.is_empty() {
        return (true, String::new());
    }

    exec_cmd(&cmd, None)
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

pub fn execute_shell(command: &str, cwd: Option<&str>, _timeout: u64) -> (bool, String) {
    exec_cmd(command, cwd)
}

pub fn terminate_process(pid: u32) -> bool {
    if cfg!(target_os = "windows") {
        Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/T", "/F"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    } else {
        Command::new("kill")
            .args(["-TERM", &pid.to_string()])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }
}

fn exec_cmd(command: &str, cwd: Option<&str>) -> (bool, String) {
    let output = spawn_shell_process(command, cwd).and_then(|process| process.wait_with_output());

    match output {
        Ok(o) => {
            let code = o.status.code().unwrap_or(-1);
            let out = String::from_utf8_lossy(&o.stdout).to_string();
            let err = String::from_utf8_lossy(&o.stderr).to_string();
            let merged = if err.is_empty() {
                out
            } else {
                format!("{out}\n{err}")
            };
            let trimmed = merged.trim();
            (code == 0, trimmed.to_string())
        }
        Err(_) => (false, "exec failed".to_string()),
    }
}

fn spawn_shell_process(command: &str, cwd: Option<&str>) -> std::io::Result<std::process::Child> {
    let mut process = if cfg!(target_os = "windows") {
        let mut process = Command::new("cmd");
        process.args(["/C", command]);
        process
    } else {
        let mut process = Command::new("sh");
        process.args(["-lc", command]);
        process
    };
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
        .unwrap_or_else(|_| {
            if cfg!(target_os = "windows") {
                "C:\\".to_string()
            } else {
                "/".to_string()
            }
        })
}

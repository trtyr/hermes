use std::sync::Arc;

use tokio::sync::RwLock;

use crate::protocol::{AgentTaskStatus, WebEvent};

use super::super::{effects::RuntimePorts, now_ts};
use crate::kernel::state::KernelState;

pub(super) async fn handle_task_result(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    session_id: u64,
    task_id: String,
    success: bool,
    output: String,
) {
    let now = now_ts();
    let mut state = state.write().await;
    if let Some(agent_id) = state.update_last_seen(session_id, now) {
        if let Some(session) = state.session_mut(session_id) {
            effects.persist_agent_online(session.snapshot());
        }
        let mut parent_task = None;
        let mut accepted = false;
        if let Some(task) = state.complete_task(&task_id, success, output.clone(), now) {
            accepted = true;
            parent_task = state.parent_task_snapshot(&task.task_id);
            effects.task_updated(task);
        }
        if let Some(task) = parent_task {
            effects.task_updated(task);
        }
        if accepted {
            effects.publish(&WebEvent::TaskResult {
                task_id,
                agent_id,
                success,
                output,
            });
        }
    }
}

pub(super) async fn handle_task_update(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    session_id: u64,
    task_id: String,
    status: AgentTaskStatus,
    output: Option<String>,
) {
    let now = now_ts();
    let mut state = state.write().await;
    if state.update_last_seen(session_id, now).is_some() {
        if let Some(session) = state.session_mut(session_id) {
            effects.persist_agent_online(session.snapshot());
        }
        let mut parent_task = None;
        let task = match status {
            AgentTaskStatus::Running => state.mark_task_running(&task_id, now),
            AgentTaskStatus::Cancelled => state.mark_task_cancelled(&task_id, output, now),
        };
        if let Some(task) = task {
            parent_task = state.parent_task_snapshot(&task.task_id);
            effects.task_updated(task);
        }
        if let Some(task) = parent_task {
            effects.task_updated(task);
        }
    }
}

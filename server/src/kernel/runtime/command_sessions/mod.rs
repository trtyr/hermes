mod close;
mod execute;
mod open;

use std::sync::Arc;

use tokio::sync::{RwLock, oneshot};

use crate::protocol::{CommandExecutionSnapshot, CommandSessionResult, CommandSessionSnapshot};

use super::effects::RuntimePorts;
use crate::kernel::state::KernelState;

pub(super) async fn open_command_session(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    agent_id: String,
    command_session_id: String,
    created_by: String,
    respond_to: oneshot::Sender<anyhow::Result<CommandSessionSnapshot>>,
) {
    open::open_command_session(
        state,
        effects,
        agent_id,
        command_session_id,
        created_by,
        respond_to,
    )
    .await;
}

pub(super) async fn execute_command_session(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    command_session_id: String,
    command_id: String,
    line: String,
    respond_to: oneshot::Sender<anyhow::Result<CommandSessionResult>>,
) {
    execute::execute_command_session(
        state,
        effects,
        command_session_id,
        command_id,
        line,
        respond_to,
    )
    .await;
}

pub(super) async fn queue_command_session(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    command_session_id: String,
    command_id: String,
    line: String,
    respond_to: oneshot::Sender<anyhow::Result<CommandExecutionSnapshot>>,
) {
    execute::queue_command_session(
        state,
        effects,
        command_session_id,
        command_id,
        line,
        respond_to,
    )
    .await;
}

pub(super) async fn close_command_session(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    command_session_id: String,
    respond_to: oneshot::Sender<anyhow::Result<CommandSessionSnapshot>>,
) {
    close::close_command_session(state, effects, command_session_id, respond_to).await;
}

pub(super) async fn dispatch_next_command_if_idle(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    command_session_id: &str,
) {
    execute::dispatch_next_command_if_idle(state, effects, command_session_id).await;
}

use std::sync::Arc;

use tokio::sync::{RwLock, oneshot};

use crate::protocol::{CommandSessionSnapshot, ServerCommand};

use super::super::{agent_lifecycle::send_server_command_to_agent, effects::RuntimePorts, now_ts};
use crate::kernel::state::KernelState;

pub(super) async fn open_command_session(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    agent_id: String,
    command_session_id: String,
    created_by: String,
    respond_to: oneshot::Sender<anyhow::Result<CommandSessionSnapshot>>,
) {
    let now = now_ts();
    let mut state = state.write().await;
    if state.session_by_agent_id(&agent_id).is_none() {
        let _ = respond_to.send(Err(anyhow::anyhow!(
            "target agent {} is not connected",
            agent_id
        )));
        return;
    }
    state.insert_command_session(
        command_session_id.clone(),
        agent_id.clone(),
        created_by,
        now,
        respond_to,
    );
    if let Err(error) = send_server_command_to_agent(
        &mut state,
        effects,
        &agent_id,
        ServerCommand::OpenCommandSession {
            command_session_id: command_session_id.clone(),
        },
        "command sender closed while opening command session",
    ) {
        state.fail_pending_command_sessions_for_agent(&agent_id, &error.to_string());
    }
}

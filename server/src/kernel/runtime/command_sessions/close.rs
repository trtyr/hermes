use std::sync::Arc;

use tokio::sync::{RwLock, oneshot};

use crate::console;
use crate::protocol::{CommandSessionSnapshot, CommandSessionStatus, ServerCommand, WebEvent};

use super::super::{agent_lifecycle::send_server_command_to_agent, effects::RuntimePorts, now_ts};
use crate::kernel::state::KernelState;

pub(super) async fn close_command_session(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    command_session_id: String,
    respond_to: oneshot::Sender<anyhow::Result<CommandSessionSnapshot>>,
) {
    let now = now_ts();
    let mut state = state.write().await;
    let Some(snapshot) = state.command_session_snapshot(&command_session_id) else {
        let _ = respond_to.send(Err(anyhow::anyhow!(
            "command session {} not found",
            command_session_id
        )));
        return;
    };
    if snapshot.status != CommandSessionStatus::Open {
        let _ = respond_to.send(Ok(snapshot));
        return;
    }
    let queued_command_ids = state.drain_command_session_queue(&command_session_id);
    for command_id in queued_command_ids {
        if let Some(command) = state.cancel_command_execution(
            &command_id,
            "command session closed by operator".to_string(),
            now,
        ) {
            state.fail_pending_command_execute(&command_id, "command session closed by operator");
            effects.publish(&WebEvent::CommandUpdated { command });
        }
    }
    if let Some(active_command_id) = state.clear_active_command_for_session(&command_session_id) {
        if let Some(command) = state.cancel_command_execution(
            &active_command_id,
            "command session closed by operator".to_string(),
            now,
        ) {
            state.fail_pending_command_execute(
                &active_command_id,
                "command session closed by operator",
            );
            effects.publish(&WebEvent::CommandUpdated { command });
        }
    }
    if state.session_by_agent_id(&snapshot.agent_id).is_none() {
        if let Some(closed) = state.close_command_session(&command_session_id, now) {
            console::command_session_closed(&closed.command_session_id, &closed.agent_id);
            effects.publish(&WebEvent::CommandSessionClosed {
                command_session_id: closed.command_session_id.clone(),
                agent_id: closed.agent_id.clone(),
            });
            let _ = respond_to.send(Ok(closed));
        }
        return;
    }
    let agent_id = snapshot.agent_id.clone();
    state.register_pending_close_command_session(command_session_id.clone(), respond_to);
    if let Err(error) = send_server_command_to_agent(
        &mut state,
        effects,
        &agent_id,
        ServerCommand::CloseCommandSession {
            command_session_id: command_session_id.clone(),
        },
        "command sender closed while closing command session",
    ) {
        state.fail_pending_command_sessions_for_agent(&agent_id, &error.to_string());
    }
}

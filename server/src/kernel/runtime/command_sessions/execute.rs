use std::sync::Arc;

use tokio::sync::{RwLock, oneshot};

use crate::protocol::{
    CommandExecutionSnapshot, CommandSessionResult, CommandSessionStatus, ServerCommand, WebEvent,
};

use super::super::{agent_lifecycle::send_server_command_to_agent, effects::RuntimePorts, now_ts};
use crate::kernel::state::KernelState;

pub(super) async fn execute_command_session(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    command_session_id: String,
    command_id: String,
    line: String,
    respond_to: oneshot::Sender<anyhow::Result<CommandSessionResult>>,
) {
    let now = now_ts();
    let mut state = state.write().await;
    if let Err(error) = validate_command_session_ready(&state, &command_session_id) {
        let _ = respond_to.send(Err(error));
        return;
    }
    state.register_pending_command_execute(command_id.clone(), respond_to);
    match enqueue_command(&mut state, &command_session_id, &command_id, line, now) {
        Ok(command) => {
            effects.publish(&WebEvent::CommandUpdated { command });
            if let Err(error) =
                dispatch_next_command_if_idle_locked(&mut state, effects, &command_session_id)
            {
                state.fail_pending_command_execute(&command_id, &error.to_string());
            }
        }
        Err(error) => {
            state.fail_pending_command_execute(&command_id, &error.to_string());
        }
    }
}

pub(super) async fn queue_command_session(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    command_session_id: String,
    command_id: String,
    line: String,
    respond_to: oneshot::Sender<anyhow::Result<CommandExecutionSnapshot>>,
) {
    let now = now_ts();
    let mut state = state.write().await;
    match enqueue_command(&mut state, &command_session_id, &command_id, line, now) {
        Ok(command) => {
            effects.publish(&WebEvent::CommandUpdated {
                command: command.clone(),
            });
            match dispatch_next_command_if_idle_locked(&mut state, effects, &command_session_id) {
                Ok(_) => {
                    let _ = respond_to.send(Ok(command));
                }
                Err(error) => {
                    let _ = respond_to.send(Err(error));
                }
            }
        }
        Err(error) => {
            let _ = respond_to.send(Err(error));
        }
    }
}

pub(super) async fn dispatch_next_command_if_idle(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    command_session_id: &str,
) {
    let mut state = state.write().await;
    let _ = dispatch_next_command_if_idle_locked(&mut state, effects, command_session_id);
}

fn enqueue_command(
    state: &mut KernelState,
    command_session_id: &str,
    command_id: &str,
    line: String,
    queued_at: u64,
) -> anyhow::Result<CommandExecutionSnapshot> {
    validate_command_session_ready(state, command_session_id)?;
    let Some(command) =
        state.queue_command_execution(command_id.to_string(), command_session_id, line, queued_at)
    else {
        return Err(anyhow::anyhow!("failed to queue command {}", command_id));
    };
    Ok(command)
}

fn validate_command_session_ready(
    state: &KernelState,
    command_session_id: &str,
) -> anyhow::Result<()> {
    let Some(snapshot) = state.command_session_snapshot(command_session_id) else {
        return Err(anyhow::anyhow!(
            "command session {} not found",
            command_session_id
        ));
    };
    if snapshot.status != CommandSessionStatus::Open {
        return Err(anyhow::anyhow!(
            "command session {} is not open",
            command_session_id
        ));
    }
    if state.session_by_agent_id(&snapshot.agent_id).is_none() {
        return Err(anyhow::anyhow!("agent {} is offline", snapshot.agent_id));
    }
    Ok(())
}

fn dispatch_next_command_if_idle_locked(
    state: &mut KernelState,
    effects: &RuntimePorts,
    command_session_id: &str,
) -> anyhow::Result<Option<CommandExecutionSnapshot>> {
    let Some(session_snapshot) = state.command_session_snapshot(command_session_id) else {
        return Ok(None);
    };
    if session_snapshot.status != CommandSessionStatus::Open {
        return Ok(None);
    }
    if state
        .command_session_active_command_id(command_session_id)
        .is_some()
    {
        return Ok(None);
    }
    let Some(next_command) = state.next_queued_command_for_session(command_session_id) else {
        return Ok(None);
    };
    send_server_command_to_agent(
        state,
        effects,
        &session_snapshot.agent_id,
        ServerCommand::ExecuteCommandSession {
            command_session_id: command_session_id.to_string(),
            request_id: next_command.command_id.clone(),
            line: next_command.line.clone(),
        },
        "command sender closed while dispatching command session execute",
    )?;
    let dispatched_at = now_ts();
    if let Some(command) =
        state.mark_command_dispatched(command_session_id, &next_command.command_id, dispatched_at)
    {
        effects.publish(&WebEvent::CommandUpdated {
            command: command.clone(),
        });
        return Ok(Some(command));
    }
    Ok(None)
}

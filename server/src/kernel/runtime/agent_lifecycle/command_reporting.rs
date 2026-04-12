use std::sync::Arc;

use tokio::sync::RwLock;

use crate::protocol::{CommandOutputChunk, CommandOutputStream, WebEvent};

use super::super::{command_sessions, effects::RuntimePorts, now_ts};
use crate::kernel::state::KernelState;

#[allow(clippy::too_many_arguments)]
pub(super) async fn handle_command_session_result(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    session_id: u64,
    command_session_id: String,
    request_id: String,
    line: String,
    cwd_before: String,
    cwd_after: String,
    exit_code: i32,
    stdout: String,
    stderr: String,
    success: bool,
) {
    let now = now_ts();
    {
        let mut state = state.write().await;
        if state.update_last_seen(session_id, now).is_some() {
            if let Some(session) = state.session_mut(session_id) {
                effects.persist_agent_online(session.snapshot());
            }
            if let Some(result) = state.finish_command_execute(
                &command_session_id,
                &request_id,
                line,
                cwd_before,
                cwd_after,
                exit_code,
                stdout,
                stderr,
                success,
                now,
            ) {
                if let Some(snapshot) = state.command_session_snapshot(&command_session_id) {
                    effects.publish(&WebEvent::CommandSessionUpdated { session: snapshot });
                }
                if let Some(command) = state.command_execution_snapshot(&request_id) {
                    effects.publish(&WebEvent::CommandUpdated { command });
                }
                effects.publish(&WebEvent::CommandSessionResult { result });
            }
        }
    }
    command_sessions::dispatch_next_command_if_idle(state, effects, &command_session_id).await;
}

pub(super) async fn handle_command_session_started(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    session_id: u64,
    command_session_id: String,
    request_id: String,
) {
    let now = now_ts();
    let mut state = state.write().await;
    if state.update_last_seen(session_id, now).is_some() {
        if let Some(session) = state.session_mut(session_id) {
            effects.persist_agent_online(session.snapshot());
        }
        if let Some(command) = state.mark_command_running(&command_session_id, &request_id, now) {
            effects.publish(&WebEvent::CommandUpdated { command });
        }
    }
}

pub(super) async fn handle_command_output_chunk(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    session_id: u64,
    command_session_id: String,
    request_id: String,
    stream: CommandOutputStream,
    chunk: String,
    sequence: u32,
) {
    let now = now_ts();
    let mut state = state.write().await;
    if state.update_last_seen(session_id, now).is_some() {
        if let Some(session) = state.session_mut(session_id) {
            effects.persist_agent_online(session.snapshot());
        }
        if let Some(command) = state.append_command_output_chunk(
            &command_session_id,
            &request_id,
            &stream,
            &chunk,
            now,
        ) {
            effects.publish(&WebEvent::CommandUpdated { command });
            effects.publish(&WebEvent::CommandOutputChunk {
                chunk: CommandOutputChunk {
                    command_session_id,
                    request_id,
                    stream,
                    chunk,
                    sequence,
                    emitted_at: now,
                },
            });
        }
    }
}

pub(super) async fn handle_command_session_opened(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    session_id: u64,
    command_session_id: String,
    cwd: String,
) {
    let now = now_ts();
    let mut state = state.write().await;
    let Some(agent_id) = state.update_last_seen(session_id, now).flatten() else {
        return;
    };
    if let Some(session) = state.session_mut(session_id) {
        effects.persist_agent_online(session.snapshot());
    }
    if let Some(snapshot) = state.activate_command_session(&command_session_id, cwd, now) {
        effects.publish(&WebEvent::CommandSessionOpened { session: snapshot });
    } else {
        let _ = agent_id;
    }
}

pub(super) async fn handle_command_session_closed(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    session_id: u64,
    command_session_id: String,
) {
    let now = now_ts();
    let mut state = state.write().await;
    let agent_id = state.update_last_seen(session_id, now).flatten();
    if let Some(session) = state.session_mut(session_id) {
        effects.persist_agent_online(session.snapshot());
    }
    for command_id in state.drain_command_session_queue(&command_session_id) {
        if let Some(command) = state.drop_command_execution(
            &command_id,
            format!("command session {} closed by agent", command_session_id),
            now,
        ) {
            effects.publish(&WebEvent::CommandUpdated { command });
        }
        state.fail_pending_command_execute(
            &command_id,
            &format!("command session {} closed by agent", command_session_id),
        );
    }
    if let Some(active_command_id) = state.clear_active_command_for_session(&command_session_id) {
        if let Some(command) = state.drop_command_execution(
            &active_command_id,
            format!("command session {} closed by agent", command_session_id),
            now,
        ) {
            effects.publish(&WebEvent::CommandUpdated { command });
        }
        state.fail_pending_command_execute(
            &active_command_id,
            &format!("command session {} closed by agent", command_session_id),
        );
    }
    if let Some(snapshot) = state.close_command_session(&command_session_id, now) {
        effects.publish(&WebEvent::CommandSessionClosed {
            command_session_id: snapshot.command_session_id,
            agent_id: snapshot.agent_id,
        });
    } else if let Some(agent_id) = agent_id {
        effects.publish(&WebEvent::CommandSessionClosed {
            command_session_id,
            agent_id,
        });
    }
}

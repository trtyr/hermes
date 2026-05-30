use std::{net::SocketAddr, sync::Arc};

use tokio::sync::{RwLock, mpsc};

use crate::console;
use crate::protocol::{ServerCommand, WebEvent};

use super::super::{effects::RuntimePorts, now_ts};
use crate::kernel::state::{AgentSession, KernelState};

const UNREGISTERED_SESSION_TIMEOUT_MS: u64 = 10_000;
const HEARTBEAT_GRACE_MS: u64 = 10_000;
const MIN_HEARTBEAT_TIMEOUT_MS: u64 = 5_000;

pub(super) async fn handle_agent_connected(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    session_id: u64,
    listener_id: Option<i64>,
    listener_name: Option<String>,
    peer_addr: SocketAddr,
    sender: mpsc::UnboundedSender<ServerCommand>,
) {
    let now = now_ts();
    let mut state = state.write().await;
    state.insert_session(AgentSession {
        session_id,
        agent_id: None,
        listener_id,
        listener_name,
        hostname: None,
        username: None,
        os: None,
        arch: None,
        pid: None,
        internal_ip: None,
        tags: Vec::new(),
        sleep_interval: 0,
        jitter: 0,
        peer_addr,
        connected_at: now,
        last_seen: now,
        sender,
        privilege: String::new(),
    });

    effects.publish(&WebEvent::AgentConnected {
        session_id,
        peer_addr: peer_addr.to_string(),
        connected_at: now,
    });
}

pub(super) async fn handle_agent_disconnected(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    session_id: u64,
) {
    let mut state = state.write().await;
    if let Some(session) = state.remove_session_preserving_agent_index(session_id) {
        cleanup_session_disconnect(effects, session);
    }
}

pub(super) async fn disconnect_agent(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    agent_id: String,
) {
    let mut state = state.write().await;
    let sender = state
        .session_by_agent_id(&agent_id)
        .map(|session| session.sender.clone());

    if let Some(sender) = sender {
        if sender
            .send(ServerCommand::Disconnect {
                reason: Some("requested by server".to_string()),
            })
            .is_ok()
        {
            effects.publish(&WebEvent::AgentDisconnectRequested {
                agent_id: agent_id.clone(),
            });
            console::agent_offline(&agent_id, "", "disconnected by server request");
        }
    }

    if let Some(session) = state.remove_existing_session_for_agent(&agent_id) {
        cleanup_session_expired(&mut state, effects, session, "was disconnected by server");
    }
}

pub(super) async fn sweep_heartbeats(state: &Arc<RwLock<KernelState>>, effects: &RuntimePorts) {
    let now = now_ts();
    let timed_out_session_ids = {
        let state = state.read().await;
        state.timed_out_session_ids(
            now,
            UNREGISTERED_SESSION_TIMEOUT_MS,
            HEARTBEAT_GRACE_MS,
            MIN_HEARTBEAT_TIMEOUT_MS,
        )
    };

    if timed_out_session_ids.is_empty() {
        return;
    }

    let sweep_count = timed_out_session_ids.len();
    let mut state = state.write().await;
    for session_id in timed_out_session_ids {
        if let Some(session) = state.remove_session(session_id) {
            if let Some(ref agent_id) = session.agent_id {
                let elapsed = now.saturating_sub(session.last_seen);
                console::agent_heartbeat_timeout(agent_id, &session.hostname.clone().unwrap_or_default(), session_id, elapsed);
            }
            cleanup_session_expired(&mut state, effects, session, "heartbeat timed out");
        }
    }
    console::heartbeat_sweep(sweep_count);
}

pub(super) fn cleanup_session_disconnect(effects: &RuntimePorts, session: AgentSession) {
    let now = now_ts();
    let agent_id = session.agent_id.clone();
    let ln = session.listener_name.clone().unwrap_or_else(|| "?".to_string());
    if let Some(ref aid) = agent_id {
        console::agent_offline(aid, &session.hostname.clone().unwrap_or_default(), "session disconnected");
        effects.mark_agent_offline(aid.clone(), now);
    }
    console::session_disconnected(session.session_id, agent_id.as_deref(), "disconnected", &ln);
    if let Some(aid) = agent_id {
        effects.publish(&WebEvent::AgentDisconnected {
            session_id: session.session_id,
            agent_id: Some(aid),
        });
    }
}

pub(super) fn cleanup_session_expired(
    state: &mut KernelState,
    effects: &RuntimePorts,
    session: AgentSession,
    reason: &str,
) {
    let now = now_ts();
    if let Some(agent_id) = session.agent_id.clone() {
        console::agent_offline(&agent_id, &session.hostname.clone().unwrap_or_default(), reason);
        effects.mark_agent_offline(agent_id.clone(), now);
        let command_session_ids = state.command_session_ids_for_agent(&agent_id);
        for command_session_id in &command_session_ids {
            for command_id in state.drain_command_session_queue(command_session_id) {
                if let Some(command) = state.drop_command_execution(
                    &command_id,
                    format!("agent {} {}", agent_id, reason),
                    now,
                ) {
                    effects.publish(&WebEvent::CommandUpdated { command });
                }
                state.fail_pending_command_execute(
                    &command_id,
                    &format!("agent {} {}", agent_id, reason),
                );
            }
            if let Some(active_command_id) =
                state.clear_active_command_for_session(command_session_id)
            {
                if let Some(command) = state.drop_command_execution(
                    &active_command_id,
                    format!("agent {} {}", agent_id, reason),
                    now,
                ) {
                    effects.publish(&WebEvent::CommandUpdated { command });
                }
                state.fail_pending_command_execute(
                    &active_command_id,
                    &format!("agent {} {}", agent_id, reason),
                );
            }
        }
        let closed_sessions = state.close_command_sessions_for_agent(&agent_id, now);
        for command_session in closed_sessions {
            effects.publish(&WebEvent::CommandSessionClosed {
                command_session_id: command_session.command_session_id,
                agent_id: agent_id.clone(),
            });
        }
        state.fail_pending_command_sessions_for_agent(
            &agent_id,
            &format!("agent {} {}", agent_id, reason),
        );
        state.fail_pending_agent_beacon_updates_for_agent(
            &agent_id,
            &format!("agent {} {}", agent_id, reason),
        );
        // Fail active tasks (Dispatched/Running/CancelRequested)
        let interrupted = state.active_task_ids_for_agent(&agent_id);
        for task_id in &interrupted {
            if let Some(task) = state.mark_task_failed(
                task_id,
                format!("agent {} {} before reporting result", agent_id, reason),
                now,
            ) {
                let parent_task = state.parent_task_snapshot(&task.task_id);
                effects.task_updated(task);
                if let Some(task) = parent_task {
                    effects.task_updated(task);
                }
            }
        }
        // Also fail Pending tasks — agent is gone, they'll never be dispatched
        let pending = state.pending_task_ids_for_agent(&agent_id);
        for task_id in &pending {
            if let Some(task) = state.mark_task_failed(
                task_id,
                format!("agent {} {} before task was dispatched", agent_id, reason),
                now,
            ) {
                let parent_task = state.parent_task_snapshot(&task.task_id);
                effects.task_updated(task);
                if let Some(task) = parent_task {
                    effects.task_updated(task);
                }
            }
        }
    }
    console::session_disconnected(session.session_id, session.agent_id.as_deref(), reason, &session.listener_name.clone().unwrap_or_else(|| "?".to_string()));
    // Only publish disconnect event for registered sessions (those with agent_id).
    if session.agent_id.is_some() {
        effects.publish(&WebEvent::AgentDisconnected {
            session_id: session.session_id,
            agent_id: session.agent_id,
        });
    }
}

pub(in crate::kernel::runtime) fn send_server_command_to_agent(
    state: &mut KernelState,
    effects: &RuntimePorts,
    agent_id: &str,
    command: ServerCommand,
    _disconnect_reason: &str,
) -> anyhow::Result<()> {
    let (session_id, sender) = {
        let Some(session) = state.session_by_agent_id(agent_id) else {
            return Err(anyhow::anyhow!("agent {} is offline", agent_id));
        };
        (session.session_id, session.sender.clone())
    };

    if sender.send(command).is_err() {
        if let Some(session) = state.remove_session_preserving_agent_index(session_id) {
            cleanup_session_disconnect(effects, session);
        }
        return Err(anyhow::anyhow!("agent {} command sender closed", agent_id));
    }

    Ok(())
}

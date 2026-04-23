use std::sync::Arc;

use tokio::sync::RwLock;

use crate::protocol::{ServerCommand, WebEvent};

use super::super::{effects::RuntimePorts, now_ts, task_flow};
use super::connection::cleanup_removed_session;
use crate::kernel::state::{AgentIdentity, KernelState};

pub(super) async fn handle_register(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    session_id: u64,
    agent_id: String,
    hostname: String,
    username: Option<String>,
    os: Option<String>,
    arch: Option<String>,
    pid: Option<u32>,
    internal_ip: Option<String>,
    tags: Vec<String>,
    sleep_interval: u64,
    jitter: u32,
    elevated: bool,
) {
    let now = now_ts();
    let mut state = state.write().await;
    let registered_agent_id = agent_id.clone();

    if state.session_mut(session_id).is_none() {
        return;
    }

    if let Some(old_session_id) = state.existing_session_for_agent(&agent_id) {
        if old_session_id != session_id {
            if let Some(old_session) = state.remove_existing_session_for_agent(&agent_id) {
                let _ = old_session.sender.send(ServerCommand::Disconnect {
                    reason: Some("superseded by a newer session".to_string()),
                });
                cleanup_removed_session(
                    &mut state,
                    effects,
                    old_session,
                    "was superseded by a newer session",
                );
            }
        }
    }

    let identity = AgentIdentity {
        agent_id,
        hostname,
        username,
        os,
        arch,
        pid,
        internal_ip,
        tags,
        sleep_interval,
        jitter,
        last_seen: now,
        elevated,
    };

    if let Some(snapshot) = state.upsert_agent_identity(session_id, identity) {
        effects.persist_agent_online(snapshot.clone());
        if let Some(session) = state.session_mut(session_id) {
            let _ = session.sender.send(ServerCommand::Ack {
                message: "register_ok".to_string(),
            });
        }
        effects.publish(&WebEvent::AgentRegistered { agent: snapshot });
        task_flow::dispatch_pending_tasks_for_agent(&mut state, effects, &registered_agent_id);
    }
}

pub(super) async fn handle_heartbeat(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    session_id: u64,
    agent_id: Option<String>,
) {
    let now = now_ts();
    let mut state = state.write().await;
    if let Some(current_agent_id) = state.update_last_seen(session_id, now) {
        if let Some(session) = state.session_mut(session_id) {
            effects.persist_agent_online(session.snapshot());
        }
        if let Some(agent_id) = current_agent_id.clone() {
            task_flow::dispatch_pending_tasks_for_agent(&mut state, effects, &agent_id);
        }
        effects.publish(&WebEvent::AgentHeartbeat {
            session_id,
            agent_id: agent_id.or(current_agent_id),
            last_seen: now,
        });
    }
}

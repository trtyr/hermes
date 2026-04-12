use std::sync::Arc;

use tokio::sync::{RwLock, oneshot};

use crate::protocol::{AgentSnapshot, ServerCommand, WebEvent};

use super::{super::effects::RuntimePorts, send_server_command_to_agent};
use crate::kernel::state::KernelState;

pub(super) async fn update_beacon_config(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    agent_id: String,
    request_id: String,
    sleep_interval: u64,
    jitter: u32,
    respond_to: oneshot::Sender<anyhow::Result<AgentSnapshot>>,
) {
    let mut state = state.write().await;
    if state.session_by_agent_id(&agent_id).is_none() {
        let _ = respond_to.send(Err(anyhow::anyhow!("agent {} is offline", agent_id)));
        return;
    }
    state.register_pending_agent_beacon_update(request_id.clone(), agent_id.clone(), respond_to);
    if send_server_command_to_agent(
        &mut state,
        effects,
        &agent_id,
        ServerCommand::UpdateBeaconConfig {
            request_id: request_id.clone(),
            sleep_interval,
            jitter,
        },
        "command sender closed while updating beacon config",
    )
    .is_err()
    {
        state.abort_pending_agent_beacon_update(&request_id);
    }
}

pub(super) async fn handle_config_updated(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    session_id: u64,
    request_id: String,
    sleep_interval: u64,
    jitter: u32,
) {
    let mut state = state.write().await;
    if let Some(snapshot) =
        state.complete_agent_beacon_update(session_id, &request_id, sleep_interval, jitter)
    {
        effects.persist_agent_online(snapshot.clone());
        effects.publish(&WebEvent::AgentUpdated { agent: snapshot });
    }
}

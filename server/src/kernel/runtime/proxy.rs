use std::sync::Arc;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use tokio::sync::{oneshot, RwLock};

use crate::protocol::{ProxySessionSnapshot, ProxySessionStatus, ServerCommand, WebEvent};

use super::{agent_lifecycle::send_server_command_to_agent, effects::RuntimePorts, now_ts};
use crate::kernel::state::KernelState;

pub(super) async fn start_proxy_session(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    agent_id: String,
    proxy_id: String,
    bind_addr: String,
    respond_to: oneshot::Sender<anyhow::Result<ProxySessionSnapshot>>,
) {
    let now = now_ts();
    let mut state = state.write().await;
    if state.session_by_agent_id(&agent_id).is_none() {
        let _ = respond_to.send(Err(anyhow::anyhow!("agent not connected")));
        return;
    }
    state.insert_proxy_session(
        proxy_id.clone(),
        agent_id.clone(),
        bind_addr.clone(),
        now,
        respond_to,
    );
    effects.persist_proxy_session(
        &proxy_id,
        &agent_id,
        &bind_addr,
        &ProxySessionStatus::Opening,
        0,
        None,
        now,
        now,
    );
    let _ = send_server_command_to_agent(
        &mut state,
        effects,
        &agent_id,
        ServerCommand::OpenProxy {
            proxy_id,
            bind_addr,
        },
        "command sender closed while opening proxy",
    );
}

pub(super) async fn open_stream(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    proxy_id: String,
    stream_id: String,
    host: String,
    port: u16,
    client_sender: tokio::sync::mpsc::UnboundedSender<Option<Vec<u8>>>,
    respond_to: oneshot::Sender<anyhow::Result<()>>,
) {
    let mut state = state.write().await;
    let Some(agent_id) = state.proxy_agent_id(&proxy_id) else {
        let _ = respond_to.send(Err(anyhow::anyhow!("proxy session not found")));
        return;
    };
    if let Err(error) = state.attach_proxy_stream(
        &proxy_id,
        stream_id.clone(),
        host.clone(),
        port,
        client_sender,
    ) {
        let _ = respond_to.send(Err(error));
        return;
    }
    state.register_pending_proxy_stream_open(stream_id.clone(), respond_to);
    let _ = send_server_command_to_agent(
        &mut state,
        effects,
        &agent_id,
        ServerCommand::ProxyConnect {
            proxy_id,
            stream_id,
            host,
            port,
        },
        "command sender closed while opening proxy stream",
    );
}

pub(super) async fn client_data(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    proxy_id: String,
    stream_id: String,
    data: Vec<u8>,
) {
    let mut state = state.write().await;
    let Some(agent_id) = state.proxy_agent_id(&proxy_id) else {
        return;
    };
    let _ = send_server_command_to_agent(
        &mut state,
        effects,
        &agent_id,
        ServerCommand::ProxyData {
            proxy_id,
            stream_id,
            data_base64: STANDARD.encode(data),
        },
        "command sender closed while proxying client data",
    );
}

pub(super) async fn client_closed(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    proxy_id: String,
    stream_id: String,
) {
    let mut state = state.write().await;
    let Some(agent_id) = state.proxy_agent_id(&proxy_id) else {
        return;
    };
    let _ = state.remove_proxy_stream(&proxy_id, &stream_id, now_ts());
    let _ = send_server_command_to_agent(
        &mut state,
        effects,
        &agent_id,
        ServerCommand::ProxyClose { proxy_id, stream_id },
        "command sender closed while closing proxy stream",
    );
}

pub(super) async fn handle_proxy_opened(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    proxy_id: String,
) {
    let mut state = state.write().await;
    if let Some(snapshot) = state.activate_proxy_session(&proxy_id, now_ts()) {
        effects.persist_proxy_session(
            &snapshot.proxy_id,
            &snapshot.agent_id,
            &snapshot.bind_addr,
            &snapshot.status,
            snapshot.active_streams,
            snapshot.last_error.as_deref(),
            snapshot.created_at,
            snapshot.updated_at,
        );
        effects.publish(&WebEvent::ProxySessionOpened { proxy: snapshot });
    }
}

pub(super) async fn handle_proxy_connect_result(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    proxy_id: String,
    stream_id: String,
    success: bool,
    detail: Option<String>,
) {
    let now = now_ts();
    let mut state = state.write().await;
    let snapshot = if success {
        state.confirm_proxy_stream_open(&proxy_id, &stream_id)
    } else {
        state.fail_proxy_stream_open(&proxy_id, &stream_id, detail.unwrap_or_else(|| "proxy connect failed".to_string()), now)
    };
    if let Some(proxy) = snapshot {
        effects.publish(&WebEvent::ProxySessionUpdated { proxy });
    }
}

pub(super) async fn handle_proxy_data(
    state: &Arc<RwLock<KernelState>>,
    _effects: &RuntimePorts,
    _proxy_id: String,
    stream_id: String,
    data_base64: String,
) {
    let sender = {
        let state = state.read().await;
        state.proxy_stream_sender(&stream_id)
    };
    let Some(sender) = sender else {
        return;
    };
    if let Ok(bytes) = STANDARD.decode(data_base64) {
        let _ = sender.send(Some(bytes));
    }
}

pub(super) async fn handle_proxy_closed(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    proxy_id: String,
    stream_id: String,
) {
    let sender = {
        let state = state.read().await;
        state.proxy_stream_sender(&stream_id)
    };
    if let Some(sender) = sender {
        let _ = sender.send(None);
    }
    let mut state = state.write().await;
    if let Some(proxy) = state.remove_proxy_stream(&proxy_id, &stream_id, now_ts()) {
        effects.publish(&WebEvent::ProxySessionUpdated { proxy });
    }
}

pub(super) async fn handle_proxy_error(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    proxy_id: String,
    stream_id: Option<String>,
    detail: String,
) {
    let now = now_ts();
    let mut state = state.write().await;
    let snapshot = if let Some(stream_id) = stream_id {
        state.fail_proxy_stream_open(&proxy_id, &stream_id, detail, now)
    } else {
        state.fail_proxy_stream_open(&proxy_id, "", detail, now)
    };
    if let Some(proxy) = snapshot {
        effects.publish(&WebEvent::ProxySessionUpdated { proxy });
    }
}

pub(super) async fn handle_proxy_session_closed(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    proxy_id: String,
) {
    let mut state = state.write().await;
    if let Some(snapshot) = state.close_proxy_session(&proxy_id, now_ts()) {
        effects.persist_proxy_session(
            &snapshot.proxy_id,
            &snapshot.agent_id,
            &snapshot.bind_addr,
            &snapshot.status,
            snapshot.active_streams,
            snapshot.last_error.as_deref(),
            snapshot.created_at,
            snapshot.updated_at,
        );
        effects.publish(&WebEvent::ProxySessionClosed {
            proxy_id: snapshot.proxy_id,
            agent_id: snapshot.agent_id,
        });
    }
}

pub(super) async fn delete_proxy_session(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    proxy_id: String,
    respond_to: oneshot::Sender<anyhow::Result<ProxySessionSnapshot>>,
) {
    let mut state = state.write().await;
    let snapshot = state.remove_proxy_session(&proxy_id);
    match snapshot {
        Some(snapshot) => {
            effects.delete_proxy_session(&proxy_id);
            effects.publish(&WebEvent::ProxySessionClosed {
                proxy_id: snapshot.proxy_id.clone(),
                agent_id: snapshot.agent_id.clone(),
            });
            let _ = respond_to.send(Ok(snapshot));
        }
        None => {
            let _ = respond_to.send(Err(anyhow::anyhow!("proxy session not found")));
        }
    }
}

use tokio::net::TcpListener;

use crate::{
    console,
    kernel::{AgentAuthMode, KernelHandle},
    protocol::{ListenerRecord, ListenerRuntimeStatus},
};

use super::accept::spawn_agent_connection;

pub(super) async fn run_tcp_json_listener(
    kernel: KernelHandle,
    listener: ListenerRecord,
    agent_token: Option<String>,
    agent_auth_mode: AgentAuthMode,
) {
    let bind_addr = format!("{}:{}", listener.bind_host, listener.bind_port);
    match TcpListener::bind(&bind_addr).await {
        Ok(tcp_listener) => {
            kernel.listener_commands().update_runtime_state(
                listener.listener_id,
                ListenerRuntimeStatus::Running,
                None,
            );
            console::startup_listener(
                listener.listener_id,
                &listener.name,
                "tcp_json_v1",
                tcp_listener
                    .local_addr()
                    .map(|addr| addr.to_string())
                    .unwrap_or(bind_addr.clone()),
            );
            loop {
                match tcp_listener.accept().await {
                    Ok((socket, peer_addr)) => {
                        spawn_agent_connection(
                            kernel.clone(),
                            listener.listener_id,
                            listener.name.clone(),
                            socket,
                            peer_addr,
                            agent_token.clone(),
                            agent_auth_mode,
                        );
                    }
                    Err(error) => {
                        kernel.listener_commands().update_runtime_state(
                            listener.listener_id,
                            ListenerRuntimeStatus::Error,
                            Some(error.to_string()),
                        );
                        console::listener_error(
                            &format!("accept {} ({})", listener.listener_id, listener.name),
                            error,
                        );
                        break;
                    }
                }
            }
        }
        Err(error) => {
            kernel.listener_commands().update_runtime_state(
                listener.listener_id,
                ListenerRuntimeStatus::Error,
                Some(error.to_string()),
            );
            console::listener_error(
                &format!(
                    "bind {} ({}) on {}",
                    listener.listener_id, listener.name, bind_addr
                ),
                error,
            );
        }
    }
}

use tokio::net::TcpStream;

use crate::{
    console,
    kernel::{AgentAuthMode, KernelHandle},
};

use super::super::session::handle_json_line_agent_connection;
use super::super::session::protocol::TRANSPORT_PROFILE_TCP_JSON_V1;

pub(super) fn spawn_agent_connection(
    kernel: KernelHandle,
    listener_id: i64,
    listener_name: String,
    socket: TcpStream,
    peer_addr: std::net::SocketAddr,
    agent_token: Option<String>,
    agent_auth_mode: AgentAuthMode,
) {
    tokio::spawn(async move {
        if let Err(error) = handle_json_line_agent_connection(
            kernel,
            socket,
            Some(listener_id),
            Some(listener_name),
            peer_addr,
            agent_token,
            agent_auth_mode,
            TRANSPORT_PROFILE_TCP_JSON_V1,
        )
        .await
        {
            console::listener_error(&format!("agent connection {}", peer_addr), error);
        }
    });
}

use crate::kernel::{AgentAuthMode, KernelHandle};

use super::listeners::run_listener_manager;

pub async fn run_agent_gateway(
    kernel: KernelHandle,
    tcp_addr: (std::net::Ipv4Addr, u16),
    agent_token: Option<String>,
    agent_auth_mode: AgentAuthMode,
) -> anyhow::Result<()> {
    run_listener_manager(kernel, tcp_addr, agent_token, agent_auth_mode).await
}

use std::sync::{Arc, RwLock};

use crate::kernel::{AgentAuthConfig, KernelHandle};

use super::listeners::run_listener_manager;

pub async fn run_agent_gateway(
    kernel: KernelHandle,
    tcp_addr: (std::net::Ipv4Addr, u16),
    agent_auth_config: Arc<RwLock<AgentAuthConfig>>,
) -> anyhow::Result<()> {
    run_listener_manager(kernel, tcp_addr, agent_auth_config).await
}

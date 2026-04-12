use std::{collections::HashMap, time::Duration};

use crate::kernel::{AgentAuthMode, KernelHandle};

mod reconcile;
mod types;

use reconcile::reconcile_listeners;
use types::ActiveListener;

pub async fn run_listener_manager(
    kernel: KernelHandle,
    tcp_addr: (std::net::Ipv4Addr, u16),
    agent_token: Option<String>,
    agent_auth_mode: AgentAuthMode,
) -> anyhow::Result<()> {
    kernel
        .listener_commands()
        .bootstrap_default_tcp("default-agent-tcp", &tcp_addr.0.to_string(), tcp_addr.1)
        .await?;

    let mut active = HashMap::<i64, ActiveListener>::new();
    loop {
        reconcile_listeners(&kernel, &agent_token, agent_auth_mode, &mut active).await;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

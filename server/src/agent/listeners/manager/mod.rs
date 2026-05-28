use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};

use crate::kernel::{AgentAuthConfig, KernelHandle};

mod reconcile;
mod types;

use reconcile::reconcile_listeners;
use types::ActiveListener;

pub async fn run_listener_manager(
    kernel: KernelHandle,
    tcp_addr: (std::net::Ipv4Addr, u16),
    agent_auth_config: Arc<RwLock<AgentAuthConfig>>,
) -> anyhow::Result<()> {
    kernel
        .listener_commands()
        .bootstrap_default_tcp("default-agent-tcp", &tcp_addr.0.to_string(), tcp_addr.1)
        .await?;

    let mut active = HashMap::<i64, ActiveListener>::new();
    loop {
        let (token, mode) = {
            let cfg = agent_auth_config.read().unwrap();
            (cfg.agent_token.clone(), cfg.agent_auth_mode)
        };
        reconcile_listeners(&kernel, &token, mode, &mut active).await;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

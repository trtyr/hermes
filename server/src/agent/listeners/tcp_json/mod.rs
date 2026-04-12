use tokio::task::JoinHandle;

use crate::{
    kernel::{AgentAuthMode, KernelHandle},
    protocol::ListenerRecord,
};

use super::registry::ListenerDriver;

mod accept;
mod runtime;

use runtime::run_tcp_json_listener;

pub struct TcpJsonListenerDriver;

impl ListenerDriver for TcpJsonListenerDriver {
    fn spawn(
        &self,
        kernel: KernelHandle,
        listener: ListenerRecord,
        agent_token: Option<String>,
        agent_auth_mode: AgentAuthMode,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            run_tcp_json_listener(kernel, listener, agent_token, agent_auth_mode).await;
        })
    }
}

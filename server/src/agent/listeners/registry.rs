use tokio::task::JoinHandle;

use crate::{
    kernel::{AgentAuthMode, KernelHandle},
    protocol::{ListenerKind, ListenerRecord},
};

use super::https_json::HttpsJsonListenerDriver;
use super::tcp_json::TcpJsonListenerDriver;

pub trait ListenerDriver: Send + Sync {
    fn fingerprint(&self, listener: &ListenerRecord) -> String {
        format!(
            "{:?}|{}|{}|{}|{}",
            listener.kind,
            listener.bind_host,
            listener.bind_port,
            listener.enabled,
            listener.config
        )
    }

    fn spawn(
        &self,
        kernel: KernelHandle,
        listener: ListenerRecord,
        agent_token: Option<String>,
        agent_auth_mode: AgentAuthMode,
    ) -> JoinHandle<()>;
}

static TCP_JSON_DRIVER: TcpJsonListenerDriver = TcpJsonListenerDriver;
static HTTPS_JSON_DRIVER: HttpsJsonListenerDriver = HttpsJsonListenerDriver;

pub fn driver_for(kind: ListenerKind) -> Option<&'static dyn ListenerDriver> {
    match kind {
        ListenerKind::TcpJson => Some(&TCP_JSON_DRIVER),
        ListenerKind::HttpsJson => Some(&HTTPS_JSON_DRIVER),
        ListenerKind::PrivateProto => None,
    }
}

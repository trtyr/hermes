use super::KernelHandle;
use crate::protocol::{ListenerKind, ListenerRecord, ListenerRuntimeStatus};

#[derive(Clone)]
pub struct ListenerCommandFacade {
    pub(super) kernel: KernelHandle,
}

impl ListenerCommandFacade {
    pub async fn bootstrap_default_tcp(
        &self,
        name: &str,
        bind_host: &str,
        bind_port: u16,
    ) -> anyhow::Result<()> {
        self.kernel
            .storage
            .bootstrap_default_listener(name, bind_host, bind_port)
            .await
    }

    pub async fn create(
        &self,
        name: String,
        kind: ListenerKind,
        bind_host: String,
        bind_port: u16,
        enabled: bool,
        config: serde_json::Value,
    ) -> anyhow::Result<ListenerRecord> {
        self.kernel
            .storage
            .create_listener_record(name, kind, bind_host, bind_port, enabled, config)
            .await
    }

    pub async fn update(
        &self,
        listener_id: i64,
        name: Option<String>,
        bind_host: Option<String>,
        bind_port: Option<u16>,
        config: Option<serde_json::Value>,
    ) -> anyhow::Result<Option<ListenerRecord>> {
        self.kernel
            .storage
            .update_listener_record(listener_id, name, bind_host, bind_port, config)
            .await
    }

    pub async fn set_enabled(
        &self,
        listener_id: i64,
        enabled: bool,
    ) -> anyhow::Result<Option<ListenerRecord>> {
        self.kernel
            .storage
            .set_listener_enabled(listener_id, enabled)
            .await
    }

    pub async fn delete(&self, listener_id: i64) -> anyhow::Result<bool> {
        self.kernel
            .storage
            .delete_listener_record(listener_id)
            .await
    }

    pub fn update_runtime_state(
        &self,
        listener_id: i64,
        runtime_status: ListenerRuntimeStatus,
        last_error: Option<String>,
    ) {
        self.kernel
            .storage
            .update_listener_runtime_state(listener_id, runtime_status, last_error);
    }
}

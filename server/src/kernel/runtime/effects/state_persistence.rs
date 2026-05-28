use crate::protocol::{AgentSnapshot, ProxySessionStatus, TaskSnapshot};

use crate::kernel::storage::Storage;

#[derive(Clone)]
pub(super) struct StatePersistence {
    storage: Storage,
}

impl StatePersistence {
    pub(super) fn new(storage: Storage) -> Self {
        Self { storage }
    }

    pub(super) async fn persist_task(&self, task: TaskSnapshot) {
        let storage = self.storage.clone();
        let _ = tokio::task::spawn_blocking(move || {
            storage.persist_task(task);
        })
        .await;
    }

    pub(super) async fn persist_agent_online(&self, agent: AgentSnapshot) {
        let storage = self.storage.clone();
        let _ = tokio::task::spawn_blocking(move || {
            storage.persist_agent_snapshot(agent, true);
        })
        .await;
    }

    pub(super) async fn mark_agent_offline(&self, agent_id: String, updated_at: u64) {
        let storage = self.storage.clone();
        let _ = tokio::task::spawn_blocking(move || {
            storage.mark_agent_offline(agent_id, updated_at);
        })
        .await;
    }

    pub(super) async fn persist_proxy_session(
        &self,
        proxy_id: String,
        agent_id: String,
        bind_addr: String,
        status: ProxySessionStatus,
        active_streams: usize,
        last_error: Option<String>,
        created_at: u64,
        updated_at: u64,
    ) {
        let storage = self.storage.clone();
        let _ = tokio::task::spawn_blocking(move || {
            storage.persist_proxy_session(
                &proxy_id,
                &agent_id,
                &bind_addr,
                &status,
                active_streams,
                last_error.as_deref(),
                created_at,
                updated_at,
            );
        })
        .await;
    }

    pub(super) async fn delete_proxy_session(&self, proxy_id: String) {
        let storage = self.storage.clone();
        let _ = tokio::task::spawn_blocking(move || {
            storage.delete_proxy_session(&proxy_id);
        })
        .await;
    }
}

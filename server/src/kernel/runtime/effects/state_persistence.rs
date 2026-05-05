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

    pub(super) fn persist_task(&self, task: TaskSnapshot) {
        self.storage.persist_task(task);
    }

    pub(super) fn persist_agent_online(&self, agent: AgentSnapshot) {
        self.storage.persist_agent_snapshot(agent, true);
    }

    pub(super) fn mark_agent_offline(&self, agent_id: String, updated_at: u64) {
        self.storage.mark_agent_offline(agent_id, updated_at);
    }

    pub(super) fn persist_proxy_session(
        &self,
        proxy_id: &str,
        agent_id: &str,
        bind_addr: &str,
        status: &ProxySessionStatus,
        active_streams: usize,
        last_error: Option<&str>,
        created_at: u64,
        updated_at: u64,
    ) {
        self.storage.persist_proxy_session(
            proxy_id,
            agent_id,
            bind_addr,
            status,
            active_streams,
            last_error,
            created_at,
            updated_at,
        );
    }

    pub(super) fn delete_proxy_session(&self, proxy_id: &str) {
        self.storage.delete_proxy_session(proxy_id);
    }
}

use crate::protocol::{AgentSnapshot, TaskSnapshot};

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
}

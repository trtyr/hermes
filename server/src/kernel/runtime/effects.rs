mod event_publisher;
mod state_persistence;

use crate::protocol::{AgentSnapshot, TaskSnapshot, WebEvent};

use super::EventBus;
use crate::kernel::storage::Storage;

#[derive(Clone)]
pub(super) struct RuntimePorts {
    publisher: event_publisher::EventPublisher,
    persistence: state_persistence::StatePersistence,
}

impl RuntimePorts {
    pub(super) fn new(events: EventBus, storage: Storage) -> Self {
        Self {
            publisher: event_publisher::EventPublisher::new(events),
            persistence: state_persistence::StatePersistence::new(storage),
        }
    }

    pub(super) fn publish(&self, event: &WebEvent) {
        self.publisher.publish(event);
    }

    pub(super) fn task_updated(&self, task: TaskSnapshot) {
        self.persistence.persist_task(task.clone());
        self.publisher.publish(&WebEvent::TaskUpdated { task });
    }

    pub(super) fn persist_agent_online(&self, agent: AgentSnapshot) {
        self.persistence.persist_agent_online(agent);
    }

    pub(super) fn mark_agent_offline(&self, agent_id: String, updated_at: u64) {
        self.persistence.mark_agent_offline(agent_id, updated_at);
    }
}

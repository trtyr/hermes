use crate::protocol::WebEvent;

use super::super::EventBus;

#[derive(Clone)]
pub(super) struct EventPublisher {
    events: EventBus,
}

impl EventPublisher {
    pub(super) fn new(events: EventBus) -> Self {
        Self { events }
    }

    pub(super) fn publish(&self, event: &WebEvent) {
        if let Ok(payload) = serde_json::to_string(event) {
            let _ = self.events.send(payload);
        }
    }
}

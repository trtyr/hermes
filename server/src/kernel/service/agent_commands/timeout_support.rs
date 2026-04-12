use std::{fmt, time::Duration};

#[derive(Debug)]
pub(super) struct AgentBeaconUpdateTimeout {
    pub(super) timeout: Duration,
}

impl fmt::Display for AgentBeaconUpdateTimeout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "agent beacon update timed out after {}s waiting for agent confirmation",
            self.timeout.as_secs()
        )
    }
}

impl std::error::Error for AgentBeaconUpdateTimeout {}

pub(super) fn beacon_timeout_error(timeout: Duration) -> anyhow::Error {
    AgentBeaconUpdateTimeout { timeout }.into()
}

use std::{fmt, time::Duration};

#[derive(Debug)]
pub(super) struct CommandSessionTimeout {
    pub(super) operation: &'static str,
    pub(super) timeout: Duration,
}

impl fmt::Display for CommandSessionTimeout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "command session {} timed out after {}s waiting for agent response",
            self.operation,
            self.timeout.as_secs()
        )
    }
}

impl std::error::Error for CommandSessionTimeout {}

pub(super) fn timeout_error(operation: &'static str, timeout: Duration) -> anyhow::Error {
    CommandSessionTimeout { operation, timeout }.into()
}

pub fn is_command_session_timeout(error: &anyhow::Error) -> bool {
    error.downcast_ref::<CommandSessionTimeout>().is_some()
}

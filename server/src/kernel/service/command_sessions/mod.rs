use std::time::Duration;

use tokio::sync::oneshot;
use tokio::time::timeout;

use super::KernelHandle;
use crate::kernel::message::CommandSessionKernelMessage;
use crate::protocol::{CommandExecutionSnapshot, CommandSessionResult, CommandSessionSnapshot};

mod queries;
mod requests;
mod timeout_support;

#[cfg(test)]
mod tests;

pub use timeout_support::is_command_session_timeout;

const COMMAND_SESSION_OPEN_TIMEOUT: Duration = Duration::from_secs(5);
const COMMAND_SESSION_EXECUTE_TIMEOUT: Duration = Duration::from_secs(20);
const COMMAND_SESSION_CLOSE_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Clone)]
pub struct CommandSessionFacade {
    pub(super) kernel: KernelHandle,
}

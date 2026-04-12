use std::time::Duration;

use tokio::sync::{mpsc, oneshot};
use tokio::time::timeout;

use super::KernelHandle;
use crate::kernel::message::{AgentKernelMessage, KernelMessage, TaskKernelMessage};
use crate::protocol::AgentSnapshot;

mod commands;
mod timeout_support;

use timeout_support::beacon_timeout_error;

const AGENT_BEACON_UPDATE_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Clone)]
pub struct AgentCommandFacade {
    pub(super) kernel: KernelHandle,
}

// Runtime is the kernel dispatcher layer. It routes domain messages,
// coordinates state transitions, and triggers side effects via runtime ports.
mod agent_lifecycle;
mod bootstrap;
mod command_sessions;
mod dispatch;
mod effects;
mod proxy;
mod task_flow;
mod watchdog;

#[cfg(test)]
mod tests;

use std::{
    sync::{Arc, atomic::AtomicU64},
    time::{SystemTime, UNIX_EPOCH},
};

use tokio::{
    sync::{RwLock, broadcast, mpsc},
    time::{Duration, interval},
};

use super::{
    auth::AuthService,
    bus::KernelBus,
    message::{
        AgentKernelMessage, CommandSessionKernelMessage, KernelMessage, ProxyKernelMessage,
        TaskKernelMessage,
    },
    service::KernelHandle,
    state::KernelState,
    storage::Storage,
};
use effects::RuntimePorts;

pub type EventBus = broadcast::Sender<String>;

pub use bootstrap::new_kernel;

pub(super) fn now_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or(0)
}

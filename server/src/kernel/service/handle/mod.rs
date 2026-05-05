use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use tokio::sync::{RwLock, mpsc};

use super::{
    AgentBuildFacade, AgentCommandFacade, AgentQueryFacade, AuthFacade, CommandSessionFacade,
    ListenerCommandFacade, ListenerQueryFacade, ProxyFacade, TaskFacade,
};
use crate::kernel::{
    auth::AuthService,
    bus::KernelBus,
    message::{
        AgentKernelMessage, CommandSessionKernelMessage, KernelMessage, ProxyKernelMessage,
        TaskKernelMessage,
    },
    runtime::EventBus,
    state::KernelState,
    storage::{AuditRecordFilter, Storage},
};
use crate::protocol::{AuditRecord, WebEvent};

mod audit;
mod capabilities;
mod ids;
mod messaging;
mod types;

pub use types::KernelHandle;

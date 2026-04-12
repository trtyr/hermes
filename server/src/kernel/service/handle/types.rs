use super::*;

#[derive(Clone)]
pub struct KernelHandle {
    pub(in crate::kernel::service) bus: KernelBus,
    pub(in crate::kernel::service) state: Arc<RwLock<KernelState>>,
    pub(in crate::kernel::service) events: EventBus,
    pub(in crate::kernel::service) storage: Storage,
    pub(in crate::kernel::service) auth: AuthService,
    pub(in crate::kernel::service) next_session_id: Arc<AtomicU64>,
    pub(in crate::kernel::service) next_task_id: Arc<AtomicU64>,
    pub(in crate::kernel::service) next_command_session_id: Arc<AtomicU64>,
    pub(in crate::kernel::service) next_command_request_id: Arc<AtomicU64>,
    pub(in crate::kernel::service) next_agent_request_id: Arc<AtomicU64>,
}

impl KernelHandle {
    pub fn new(
        bus: KernelBus,
        state: Arc<RwLock<KernelState>>,
        events: EventBus,
        storage: Storage,
        auth: AuthService,
        next_session_id: Arc<AtomicU64>,
        next_task_id: Arc<AtomicU64>,
        next_command_session_id: Arc<AtomicU64>,
        next_command_request_id: Arc<AtomicU64>,
        next_agent_request_id: Arc<AtomicU64>,
    ) -> Self {
        Self {
            bus,
            state,
            events,
            storage,
            auth,
            next_session_id,
            next_task_id,
            next_command_session_id,
            next_command_request_id,
            next_agent_request_id,
        }
    }
}

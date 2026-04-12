use super::*;

impl KernelHandle {
    pub fn allocate_session_id(&self) -> u64 {
        self.next_session_id.fetch_add(1, Ordering::Relaxed)
    }

    pub(in crate::kernel::service) fn allocate_task_id(&self) -> String {
        let id = self.next_task_id.fetch_add(1, Ordering::Relaxed);
        format!("task-{id}")
    }

    pub(in crate::kernel::service) fn allocate_command_session_id(&self) -> String {
        let id = self.next_command_session_id.fetch_add(1, Ordering::Relaxed);
        format!("cmdsess-{id}")
    }

    pub(in crate::kernel::service) fn allocate_command_request_id(&self) -> String {
        let id = self.next_command_request_id.fetch_add(1, Ordering::Relaxed);
        format!("cmdreq-{id}")
    }

    pub(in crate::kernel::service) fn allocate_agent_request_id(&self) -> String {
        let id = self.next_agent_request_id.fetch_add(1, Ordering::Relaxed);
        format!("agentreq-{id}")
    }
}

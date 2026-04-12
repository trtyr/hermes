use super::KernelHandle;
use crate::kernel::storage::AgentRecordFilter;
use crate::protocol::{AgentRecord, AgentSnapshot};

#[derive(Clone)]
pub struct AgentQueryFacade {
    pub(super) kernel: KernelHandle,
}

impl AgentQueryFacade {
    pub async fn snapshots(&self) -> Vec<AgentSnapshot> {
        let state = self.kernel.state.read().await;
        state.snapshots()
    }

    pub async fn filtered_persisted(
        &self,
        online: Option<bool>,
        disabled: Option<bool>,
        keyword: Option<String>,
        tag: Option<String>,
    ) -> anyhow::Result<Vec<AgentRecord>> {
        self.kernel
            .storage
            .filtered_agent_records(AgentRecordFilter {
                online,
                disabled,
                keyword,
                tag,
            })
            .await
    }

    pub async fn persisted(&self, agent_id: &str) -> anyhow::Result<Option<AgentRecord>> {
        self.kernel.storage.agent_record(agent_id).await
    }

    pub async fn is_connected(&self, agent_id: &str) -> bool {
        let state = self.kernel.state.read().await;
        state.session_by_agent_id(agent_id).is_some()
    }
}

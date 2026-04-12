use super::*;

impl AgentBuildFacade {
    pub async fn filtered_records(
        &self,
        status: Option<AgentBuildStatus>,
        target_triple: Option<String>,
    ) -> anyhow::Result<Vec<AgentBuildRecord>> {
        self.kernel
            .storage
            .filtered_agent_build_records(AgentBuildRecordFilter {
                status,
                target_triple,
            })
            .await
    }

    pub async fn record(&self, build_id: i64) -> anyhow::Result<Option<AgentBuildRecord>> {
        self.kernel.storage.agent_build_record(build_id).await
    }
}

use super::*;

impl AgentCommandFacade {
    pub async fn send_message(
        &self,
        message: AgentKernelMessage,
    ) -> anyhow::Result<(), mpsc::error::SendError<KernelMessage>> {
        self.kernel.send_agent_message(message).await
    }

    pub async fn delete_persisted(&self, agent_id: &str) -> anyhow::Result<bool> {
        self.kernel.storage.delete_agent_record(agent_id).await
    }

    pub async fn set_disabled(&self, agent_id: &str, disabled: bool) -> anyhow::Result<bool> {
        self.kernel
            .storage
            .set_agent_disabled(agent_id, disabled)
            .await
    }

    pub async fn disconnect(
        &self,
        agent_id: String,
    ) -> anyhow::Result<(), mpsc::error::SendError<KernelMessage>> {
        self.kernel
            .send_task_message(TaskKernelMessage::DisconnectAgent { agent_id })
            .await
    }

    pub async fn update_beacon_config(
        &self,
        agent_id: String,
        sleep_interval: u64,
        jitter: u32,
    ) -> anyhow::Result<AgentSnapshot> {
        let request_id = self.kernel.allocate_agent_request_id();
        let (tx, rx) = oneshot::channel();
        self.kernel
            .send_agent_message(AgentKernelMessage::UpdateBeaconConfig {
                agent_id,
                request_id,
                sleep_interval,
                jitter,
                respond_to: tx,
            })
            .await
            .map_err(anyhow::Error::new)?;

        match timeout(AGENT_BEACON_UPDATE_TIMEOUT, rx).await {
            Ok(response) => {
                response.map_err(|_| anyhow::anyhow!("agent beacon update channel closed"))?
            }
            Err(_) => Err(beacon_timeout_error(AGENT_BEACON_UPDATE_TIMEOUT)),
        }
    }
}

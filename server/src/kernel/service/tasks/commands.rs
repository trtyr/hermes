use super::*;

impl TaskFacade {
    pub async fn dispatch_to_agent(
        &self,
        target_agent_id: String,
        command: String,
        payload: Option<String>,
    ) -> anyhow::Result<String, mpsc::error::SendError<KernelMessage>> {
        let task_id = self.kernel.allocate_task_id();
        self.kernel
            .send_task_message(TaskKernelMessage::Dispatch {
                target_agent_id,
                task_id: task_id.clone(),
                command,
                payload,
            })
            .await?;
        Ok(task_id)
    }

    pub async fn broadcast(
        &self,
        command: String,
        payload: Option<String>,
    ) -> anyhow::Result<String, mpsc::error::SendError<KernelMessage>> {
        let task_id = self.kernel.allocate_task_id();
        self.kernel
            .send_task_message(TaskKernelMessage::Broadcast {
                task_id: task_id.clone(),
                command,
                payload,
            })
            .await?;
        Ok(task_id)
    }

    pub async fn cancel(
        &self,
        task_id: String,
    ) -> anyhow::Result<(), mpsc::error::SendError<KernelMessage>> {
        self.kernel
            .send_task_message(TaskKernelMessage::Cancel { task_id })
            .await
    }
}

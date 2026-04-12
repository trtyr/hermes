use super::*;

impl TaskFacade {
    pub async fn snapshot(&self, task_id: &str) -> Option<TaskSnapshot> {
        let state = self.kernel.state.read().await;
        state.task_snapshot(task_id)
    }

    pub async fn filtered_snapshots(
        &self,
        status: Option<TaskStatus>,
        agent_id: Option<String>,
        keyword: Option<String>,
    ) -> Vec<TaskSnapshot> {
        let state = self.kernel.state.read().await;
        let mut tasks = state.task_snapshots();

        if let Some(status) = status {
            tasks.retain(|task| task.status == status);
        }

        if let Some(agent_id) = agent_id {
            tasks.retain(|task| task.target_agent_id.as_deref() == Some(agent_id.as_str()));
        }

        if let Some(keyword) = keyword {
            let keyword = keyword.to_lowercase();
            tasks.retain(|task| {
                task.task_id.to_lowercase().contains(&keyword)
                    || task.command.to_lowercase().contains(&keyword)
                    || task
                        .payload
                        .as_deref()
                        .unwrap_or_default()
                        .to_lowercase()
                        .contains(&keyword)
                    || task
                        .target_agent_id
                        .as_deref()
                        .unwrap_or_default()
                        .to_lowercase()
                        .contains(&keyword)
            });
        }

        tasks
    }
}

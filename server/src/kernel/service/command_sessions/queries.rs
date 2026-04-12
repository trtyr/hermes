use super::*;

impl CommandSessionFacade {
    pub async fn snapshot(&self, command_session_id: &str) -> Option<CommandSessionSnapshot> {
        let state = self.kernel.state.read().await;
        state.command_session_snapshot(command_session_id)
    }

    pub async fn snapshots(&self) -> Vec<CommandSessionSnapshot> {
        let state = self.kernel.state.read().await;
        state.command_session_snapshots()
    }

    pub async fn command_snapshots(
        &self,
        command_session_id: &str,
    ) -> Vec<CommandExecutionSnapshot> {
        let state = self.kernel.state.read().await;
        state.command_execution_snapshots_for_session(command_session_id)
    }

    pub async fn command_snapshot(&self, command_id: &str) -> Option<CommandExecutionSnapshot> {
        let state = self.kernel.state.read().await;
        state.command_execution_snapshot(command_id)
    }
}

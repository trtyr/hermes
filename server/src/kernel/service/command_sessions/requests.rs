use super::*;

use timeout_support::timeout_error;

impl CommandSessionFacade {
    pub async fn open(
        &self,
        agent_id: String,
        created_by: String,
    ) -> anyhow::Result<CommandSessionSnapshot> {
        let command_session_id = self.kernel.allocate_command_session_id();
        let (tx, rx) = oneshot::channel();
        self.kernel
            .send_command_session_message(CommandSessionKernelMessage::Open {
                agent_id,
                command_session_id: command_session_id.clone(),
                created_by,
                respond_to: tx,
            })
            .await
            .map_err(anyhow::Error::new)?;
        match timeout(COMMAND_SESSION_OPEN_TIMEOUT, rx).await {
            Ok(response) => {
                response.map_err(|_| anyhow::anyhow!("command session open channel closed"))?
            }
            Err(_) => {
                let mut state = self.kernel.state.write().await;
                state.abort_pending_open_command_session(&command_session_id);
                Err(timeout_error("open", COMMAND_SESSION_OPEN_TIMEOUT))
            }
        }
    }

    pub async fn execute(
        &self,
        command_session_id: String,
        line: String,
    ) -> anyhow::Result<CommandSessionResult> {
        let command_id = self.kernel.allocate_command_request_id();
        let (tx, rx) = oneshot::channel();
        self.kernel
            .send_command_session_message(CommandSessionKernelMessage::Execute {
                command_session_id: command_session_id.clone(),
                command_id: command_id.clone(),
                line,
                respond_to: tx,
            })
            .await
            .map_err(anyhow::Error::new)?;
        match timeout(COMMAND_SESSION_EXECUTE_TIMEOUT, rx).await {
            Ok(response) => {
                response.map_err(|_| anyhow::anyhow!("command session execute channel closed"))?
            }
            Err(_) => {
                let mut state = self.kernel.state.write().await;
                state.abort_pending_command_execute(&command_id);
                Err(timeout_error("execute", COMMAND_SESSION_EXECUTE_TIMEOUT))
            }
        }
    }

    pub async fn queue(
        &self,
        command_session_id: String,
        line: String,
    ) -> anyhow::Result<CommandExecutionSnapshot> {
        let command_id = self.kernel.allocate_command_request_id();
        let (tx, rx) = oneshot::channel();
        self.kernel
            .send_command_session_message(CommandSessionKernelMessage::Queue {
                command_session_id,
                command_id,
                line,
                respond_to: tx,
            })
            .await
            .map_err(anyhow::Error::new)?;
        rx.await
            .map_err(|_| anyhow::anyhow!("command session queue channel closed"))?
    }

    pub async fn close(
        &self,
        command_session_id: String,
    ) -> anyhow::Result<CommandSessionSnapshot> {
        let (tx, rx) = oneshot::channel();
        self.kernel
            .send_command_session_message(CommandSessionKernelMessage::Close {
                command_session_id: command_session_id.clone(),
                respond_to: tx,
            })
            .await
            .map_err(anyhow::Error::new)?;
        match timeout(COMMAND_SESSION_CLOSE_TIMEOUT, rx).await {
            Ok(response) => {
                response.map_err(|_| anyhow::anyhow!("command session close channel closed"))?
            }
            Err(_) => {
                let mut state = self.kernel.state.write().await;
                state.abort_pending_close_command_session(&command_session_id);
                Err(timeout_error("close", COMMAND_SESSION_CLOSE_TIMEOUT))
            }
        }
    }
}

use super::*;

impl KernelHandle {
    async fn send(
        &self,
        message: KernelMessage,
    ) -> anyhow::Result<(), mpsc::error::SendError<KernelMessage>> {
        self.bus.send_message(message).await
    }

    pub async fn send_agent_message(
        &self,
        message: AgentKernelMessage,
    ) -> anyhow::Result<(), mpsc::error::SendError<KernelMessage>> {
        self.send(KernelMessage::Agent(message)).await
    }

    pub(in crate::kernel::service) async fn send_task_message(
        &self,
        message: TaskKernelMessage,
    ) -> anyhow::Result<(), mpsc::error::SendError<KernelMessage>> {
        self.send(KernelMessage::Task(message)).await
    }

    pub(in crate::kernel::service) async fn send_command_session_message(
        &self,
        message: CommandSessionKernelMessage,
    ) -> anyhow::Result<(), mpsc::error::SendError<KernelMessage>> {
        self.send(KernelMessage::CommandSession(message)).await
    }

    pub(in crate::kernel::service) async fn send_proxy_message(
        &self,
        message: ProxyKernelMessage,
    ) -> anyhow::Result<(), mpsc::error::SendError<KernelMessage>> {
        self.send(KernelMessage::Proxy(message)).await
    }
}

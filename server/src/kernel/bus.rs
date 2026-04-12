use tokio::sync::mpsc;

use super::message::KernelMessage;

#[derive(Clone)]
pub struct KernelBus {
    message_sender: mpsc::Sender<KernelMessage>,
}

impl KernelBus {
    pub fn new(buffer_size: usize) -> (Self, mpsc::Receiver<KernelMessage>) {
        let (sender, receiver) = mpsc::channel(buffer_size);
        (
            Self {
                message_sender: sender,
            },
            receiver,
        )
    }

    pub async fn send_message(
        &self,
        message: KernelMessage,
    ) -> anyhow::Result<(), mpsc::error::SendError<KernelMessage>> {
        self.message_sender.send(message).await?;
        Ok(())
    }
}

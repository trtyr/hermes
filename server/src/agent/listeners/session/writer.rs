use tokio::{io::AsyncWriteExt, sync::mpsc};

use crate::protocol::ServerCommand;

pub(super) async fn write_loop<W>(
    mut writer: W,
    mut receiver: mpsc::UnboundedReceiver<ServerCommand>,
) where
    W: AsyncWriteExt + Unpin + Send,
{
    while let Some(command) = receiver.recv().await {
        match serde_json::to_string(&command) {
            Ok(payload) => {
                if writer.write_all(payload.as_bytes()).await.is_err() {
                    break;
                }
                if writer.write_all(b"\n").await.is_err() {
                    break;
                }
            }
            Err(error) => {
                eprintln!("Failed to serialize server command: {}", error);
            }
        }
    }
}

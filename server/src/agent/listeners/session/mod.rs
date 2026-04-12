use anyhow::Context;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    sync::mpsc,
};

use crate::{
    kernel::{AgentAuthMode, AgentKernelMessage, KernelHandle},
    protocol::{AgentMessage, ServerCommand},
};

mod auth;
pub(super) mod protocol;
mod writer;

#[cfg(test)]
mod tests;

use auth::{generate_session_nonce, is_agent_token_valid};
use protocol::{AGENT_PROTOCOL_VERSION, TRANSPORT_CAPABILITIES};
use writer::write_loop;

pub async fn handle_json_line_agent_connection<S>(
    kernel: KernelHandle,
    socket: S,
    listener_id: Option<i64>,
    listener_name: Option<String>,
    peer_addr: std::net::SocketAddr,
    expected_agent_token: Option<String>,
    agent_auth_mode: AgentAuthMode,
    transport_profile: &str,
) -> anyhow::Result<()>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static,
{
    let session_id = kernel.allocate_session_id();
    let (reader, writer) = tokio::io::split(socket);
    let (sender, receiver) = mpsc::unbounded_channel::<ServerCommand>();
    let session_nonce = if expected_agent_token.is_some()
        && matches!(agent_auth_mode, AgentAuthMode::ChallengeResponse)
    {
        Some(generate_session_nonce()?)
    } else {
        None
    };

    let write_task = tokio::spawn(write_loop(writer, receiver));
    kernel
        .agent_commands()
        .send_message(AgentKernelMessage::Connected {
            session_id,
            listener_id,
            listener_name: listener_name.clone(),
            peer_addr,
            sender: sender.clone(),
        })
        .await
        .context("failed to notify kernel about agent connection")?;

    if let Some(session_nonce) = &session_nonce {
        sender
            .send(ServerCommand::Hello {
                protocol_version: AGENT_PROTOCOL_VERSION,
                session_nonce: session_nonce.clone(),
                listener_id,
                listener_name: listener_name.clone(),
                transport: transport_profile.to_string(),
                capabilities: TRANSPORT_CAPABILITIES
                    .iter()
                    .map(|item| (*item).to_string())
                    .collect(),
                auth_mode: "challenge_response".to_string(),
            })
            .context("failed to send server hello")?;
    }
    let mut lines = BufReader::new(reader).lines();
    let mut registered = false;

    while let Some(line) = lines.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }

        match serde_json::from_str::<AgentMessage>(&line) {
            Ok(frame) => {
                if !registered {
                    match &frame {
                        AgentMessage::Register {
                            agent_id,
                            token,
                            session_nonce: provided_nonce,
                            auth_response,
                            ..
                        } => {
                            if !is_agent_token_valid(
                                agent_auth_mode,
                                expected_agent_token.as_deref(),
                                token.as_deref(),
                                session_nonce.as_deref(),
                                provided_nonce.as_deref(),
                                auth_response.as_deref(),
                                agent_id,
                            ) {
                                eprintln!("Rejected agent {} due to invalid token", peer_addr);
                                break;
                            }
                            match kernel.agent_queries().persisted(agent_id).await {
                                Ok(Some(agent)) if agent.is_disabled => {
                                    eprintln!(
                                        "Rejected disabled agent {} ({})",
                                        agent_id, peer_addr
                                    );
                                    break;
                                }
                                Ok(_) => {}
                                Err(error) => {
                                    eprintln!(
                                        "Failed to check disabled state for {}: {}",
                                        peer_addr, error
                                    );
                                    break;
                                }
                            }
                            registered = true;
                        }
                        _ => {
                            eprintln!(
                                "Rejected agent {} because first frame was not register",
                                peer_addr
                            );
                            break;
                        }
                    }
                }
                kernel
                    .agent_commands()
                    .send_message(AgentKernelMessage::Frame { session_id, frame })
                    .await
                    .context("failed to forward agent frame into kernel")?;
            }
            Err(error) => {
                eprintln!("Invalid agent frame from {}: {}", peer_addr, error);
            }
        }
    }

    let _ = registered;
    kernel
        .agent_commands()
        .send_message(AgentKernelMessage::Disconnected { session_id })
        .await
        .context("failed to notify kernel about agent disconnect")?;

    let _ = write_task.await;
    Ok(())
}

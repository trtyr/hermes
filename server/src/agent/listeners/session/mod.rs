use anyhow::Context;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    sync::mpsc,
};

use crate::{
    console,
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

fn parse_agent_auth_mode(value: Option<&str>) -> Option<AgentAuthMode> {
    match value {
        Some("plain_token") => Some(AgentAuthMode::PlainToken),
        Some("challenge_response") => Some(AgentAuthMode::ChallengeResponse),
        _ => None,
    }
}

async fn resolve_listener_agent_auth(
    kernel: &KernelHandle,
    listener_id: Option<i64>,
    fallback_agent_token: Option<String>,
    fallback_agent_auth_mode: AgentAuthMode,
) -> (Option<String>, AgentAuthMode) {
    let Some(listener_id) = listener_id else {
        return (fallback_agent_token, fallback_agent_auth_mode);
    };

    let Ok(Some(listener)) = kernel.listener_queries().record(listener_id).await else {
        return (fallback_agent_token, fallback_agent_auth_mode);
    };

    let Some(listener_agent_token) = listener
        .config
        .get("agent_token")
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
    else {
        return (fallback_agent_token, fallback_agent_auth_mode);
    };

    let listener_agent_auth_mode = parse_agent_auth_mode(
        listener
            .config
            .get("agent_auth_mode")
            .and_then(|value| value.as_str()),
    )
    .unwrap_or(fallback_agent_auth_mode);

    (Some(listener_agent_token), listener_agent_auth_mode)
}

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
    let (expected_agent_token, agent_auth_mode) = resolve_listener_agent_auth(
        &kernel,
        listener_id,
        expected_agent_token,
        agent_auth_mode,
    )
    .await;
    let session_id = kernel.allocate_session_id();
    let ln = listener_name.clone().unwrap_or_else(|| "?".to_string());
    console::session_connected(session_id, &peer_addr.to_string(), &ln, listener_id);
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
    let mut invalid_frame_count: u32 = 0;
    const INVALID_FRAME_LOG_MAX: u32 = 3;

    loop {
        let line_result = lines.next_line().await;
        match line_result {
            Ok(Some(line)) => {
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
                                        console::session_register_rejected(session_id, &peer_addr.to_string(), &ln, "invalid token");
                                        break;
                                    }
                                    match kernel.agent_queries().persisted(agent_id).await {
                                        Ok(Some(agent)) if agent.is_disabled => {
                                            console::session_register_rejected(session_id, &peer_addr.to_string(), &ln, &format!("agent {} is disabled", agent_id));
                                            // Tell the agent to disconnect so it can reconnect later
                                            let _ = sender.send(ServerCommand::Disconnect {
                                                reason: Some("agent is disabled".to_string()),
                                            });
                                            break;
                                        }
                                        Ok(_) => {}
                                        Err(error) => {
                                            console::session_error(session_id, "disabled check failed", &error);
                                            break;
                                        }
                                    }
                                    registered = true;
                                    let auth_mode_str = match agent_auth_mode {
                                        AgentAuthMode::PlainToken => "plain_token",
                                        AgentAuthMode::ChallengeResponse => "challenge_response",
                                    };
                                    console::session_register_ok(session_id, agent_id, "", "", "", &peer_addr.to_string(), &ln, auth_mode_str);
                                }
                                _ => {
                                    console::session_register_rejected(session_id, &peer_addr.to_string(), &ln, "first frame was not Register");
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
                        invalid_frame_count += 1;
                        if invalid_frame_count <= INVALID_FRAME_LOG_MAX {
                            console::session_error(session_id, "invalid frame", &error);
                        }
                    }
                }
            }
            Ok(None) => {
                console::session_disconnected(session_id, None, "EOF (client closed)", &ln);
                break;
            }
            Err(error) => {
                console::session_error(session_id, "read error", &error);
                break;
            }
        }
    }

    if invalid_frame_count > INVALID_FRAME_LOG_MAX {
        console::session_error(
            session_id,
            "suppressed invalid frames",
            &format!(
                "{} additional invalid frames suppressed ({} total)",
                invalid_frame_count - INVALID_FRAME_LOG_MAX,
                invalid_frame_count,
            ),
        );
    }

    console::session_disconnected(session_id, None, "connection ended", &ln);

    // Drop local sender first so write_loop's receiver can close
    // once the kernel also drops its clone (triggered by Disconnected below).
    drop(sender);

    // Only notify kernel about disconnect if the agent was successfully registered.
    // Unregistered connections (auth failures, protocol errors) never came online,
    // so they should not trigger AgentDisconnected events to WebSocket clients.
    if registered {
        kernel
            .agent_commands()
            .send_message(AgentKernelMessage::Disconnected { session_id })
            .await
            .context("failed to notify kernel about agent disconnect")?;
    }

    let _ = write_task.await;
    Ok(())
}

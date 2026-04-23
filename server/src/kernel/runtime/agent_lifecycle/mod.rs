mod beacon_config;
mod command_reporting;
mod connection;
mod registration;
mod task_reporting;

use std::{net::SocketAddr, sync::Arc};

use tokio::sync::{RwLock, mpsc, oneshot};

use crate::protocol::{AgentMessage, AgentSnapshot, ServerCommand};

use super::effects::RuntimePorts;
use crate::kernel::state::KernelState;

pub(super) use connection::send_server_command_to_agent;

pub(super) async fn handle_agent_connected(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    session_id: u64,
    listener_id: Option<i64>,
    listener_name: Option<String>,
    peer_addr: SocketAddr,
    sender: mpsc::UnboundedSender<ServerCommand>,
) {
    connection::handle_agent_connected(
        state,
        effects,
        session_id,
        listener_id,
        listener_name,
        peer_addr,
        sender,
    )
    .await;
}

pub(super) async fn handle_agent_disconnected(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    session_id: u64,
) {
    connection::handle_agent_disconnected(state, effects, session_id).await;
}

pub(super) async fn disconnect_agent(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    agent_id: String,
) {
    connection::disconnect_agent(state, effects, agent_id).await;
}

pub(super) async fn sweep_heartbeats(state: &Arc<RwLock<KernelState>>, effects: &RuntimePorts) {
    connection::sweep_heartbeats(state, effects).await;
}

pub(super) async fn update_beacon_config(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    agent_id: String,
    request_id: String,
    sleep_interval: u64,
    jitter: u32,
    respond_to: oneshot::Sender<anyhow::Result<AgentSnapshot>>,
) {
    beacon_config::update_beacon_config(
        state,
        effects,
        agent_id,
        request_id,
        sleep_interval,
        jitter,
        respond_to,
    )
    .await;
}

pub(super) async fn handle_agent_frame(
    state: &Arc<RwLock<KernelState>>,
    effects: &RuntimePorts,
    session_id: u64,
    frame: AgentMessage,
) {
    match frame {
        AgentMessage::Register {
            agent_id,
            hostname,
            username,
            protocol_version: _,
            os,
            arch,
            pid,
            internal_ip,
            tags,
            sleep_interval,
            jitter,
            token: _,
            privilege,
            ..
        } => {
            registration::handle_register(
                state,
                effects,
                session_id,
                agent_id,
                hostname,
                username,
                os,
                arch,
                pid,
                internal_ip,
                tags,
                sleep_interval,
                jitter,
                privilege,
            )
            .await;
        }
        AgentMessage::Heartbeat { agent_id } => {
            registration::handle_heartbeat(state, effects, session_id, agent_id).await;
        }
        AgentMessage::ConfigUpdated {
            request_id,
            sleep_interval,
            jitter,
        } => {
            beacon_config::handle_config_updated(
                state,
                effects,
                session_id,
                request_id,
                sleep_interval,
                jitter,
            )
            .await;
        }
        AgentMessage::TaskResult {
            task_id,
            success,
            output,
        } => {
            task_reporting::handle_task_result(
                state, effects, session_id, task_id, success, output,
            )
            .await;
        }
        AgentMessage::TaskUpdate {
            task_id,
            status,
            output,
        } => {
            task_reporting::handle_task_update(state, effects, session_id, task_id, status, output)
                .await;
        }
        AgentMessage::CommandSessionOpened {
            command_session_id,
            cwd,
        } => {
            command_reporting::handle_command_session_opened(
                state,
                effects,
                session_id,
                command_session_id,
                cwd,
            )
            .await;
        }
        AgentMessage::CommandSessionStarted {
            command_session_id,
            request_id,
        } => {
            command_reporting::handle_command_session_started(
                state,
                effects,
                session_id,
                command_session_id,
                request_id,
            )
            .await;
        }
        AgentMessage::CommandSessionOutputChunk {
            command_session_id,
            request_id,
            stream,
            chunk,
            sequence,
        } => {
            command_reporting::handle_command_output_chunk(
                state,
                effects,
                session_id,
                command_session_id,
                request_id,
                stream,
                chunk,
                sequence,
            )
            .await;
        }
        AgentMessage::CommandSessionResult {
            command_session_id,
            request_id,
            line,
            cwd_before,
            cwd_after,
            exit_code,
            stdout,
            stderr,
            success,
        } => {
            command_reporting::handle_command_session_result(
                state,
                effects,
                session_id,
                command_session_id,
                request_id,
                line,
                cwd_before,
                cwd_after,
                exit_code,
                stdout,
                stderr,
                success,
            )
            .await;
        }
        AgentMessage::CommandSessionClosed { command_session_id } => {
            command_reporting::handle_command_session_closed(
                state,
                effects,
                session_id,
                command_session_id,
            )
            .await;
        }
    }
}

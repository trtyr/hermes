use std::net::SocketAddr;

use tokio::sync::{mpsc, oneshot};

use crate::protocol::{
    AgentMessage, AgentSnapshot, CommandExecutionSnapshot, CommandSessionResult,
    CommandSessionSnapshot, ServerCommand,
};

#[derive(Debug)]
pub enum KernelMessage {
    Agent(AgentKernelMessage),
    Task(TaskKernelMessage),
    CommandSession(CommandSessionKernelMessage),
}

#[derive(Debug)]
pub enum AgentKernelMessage {
    Connected {
        session_id: u64,
        listener_id: Option<i64>,
        listener_name: Option<String>,
        peer_addr: SocketAddr,
        sender: mpsc::UnboundedSender<ServerCommand>,
    },
    Disconnected {
        session_id: u64,
    },
    Frame {
        session_id: u64,
        frame: AgentMessage,
    },
    UpdateBeaconConfig {
        agent_id: String,
        request_id: String,
        sleep_interval: u64,
        jitter: u32,
        respond_to: oneshot::Sender<anyhow::Result<AgentSnapshot>>,
    },
    SweepHeartbeats,
}

#[derive(Debug)]
pub enum TaskKernelMessage {
    Dispatch {
        target_agent_id: String,
        task_id: String,
        command: String,
        payload: Option<String>,
    },
    Broadcast {
        task_id: String,
        command: String,
        payload: Option<String>,
    },
    Cancel {
        task_id: String,
    },
    DisconnectAgent {
        agent_id: String,
    },
}

#[derive(Debug)]
pub enum CommandSessionKernelMessage {
    Open {
        agent_id: String,
        command_session_id: String,
        created_by: String,
        respond_to: oneshot::Sender<anyhow::Result<CommandSessionSnapshot>>,
    },
    Execute {
        command_session_id: String,
        command_id: String,
        line: String,
        respond_to: oneshot::Sender<anyhow::Result<CommandSessionResult>>,
    },
    Queue {
        command_session_id: String,
        command_id: String,
        line: String,
        respond_to: oneshot::Sender<anyhow::Result<CommandExecutionSnapshot>>,
    },
    Close {
        command_session_id: String,
        respond_to: oneshot::Sender<anyhow::Result<CommandSessionSnapshot>>,
    },
}

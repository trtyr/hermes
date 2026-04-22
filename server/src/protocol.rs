use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AgentMessage {
    Register {
        agent_id: String,
        hostname: String,
        username: Option<String>,
        #[serde(default = "default_agent_protocol_version")]
        protocol_version: u32,
        #[serde(default)]
        os: Option<String>,
        #[serde(default)]
        arch: Option<String>,
        #[serde(default)]
        pid: Option<u32>,
        #[serde(default)]
        internal_ip: Option<String>,
        #[serde(default)]
        tags: Vec<String>,
        #[serde(default)]
        sleep_interval: u64,
        #[serde(default)]
        jitter: u32,
        #[serde(default)]
        token: Option<String>,
        #[serde(default)]
        session_nonce: Option<String>,
        #[serde(default)]
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentTaskStatus {
    Running,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    DispatchTask {
        task_id: String,
        command: String,
        payload: Option<String>,
    },
    Disconnect {
        reason: Option<String>,
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
}

fn default_agent_protocol_version() -> u32 {
    1
}

#[derive(Debug, Clone, Serialize)]
pub struct AgentSnapshot {
    pub session_id: u64,
    pub agent_id: Option<String>,
    pub listener_id: Option<i64>,
    pub listener_name: Option<String>,
    pub hostname: Option<String>,
    pub username: Option<String>,
    pub os: Option<String>,
    pub arch: Option<String>,
    pub pid: Option<u32>,
    pub internal_ip: Option<String>,
    pub external_ip: Option<String>,
    pub tags: Vec<String>,
    pub sleep_interval: u64,
    pub jitter: u32,
    pub peer_addr: String,
    pub connected_at: u64,
    pub last_seen: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct AgentRecord {
    pub agent_id: String,
    pub session_id: Option<u64>,
    pub listener_id: Option<i64>,
    pub listener_name: Option<String>,
    pub hostname: Option<String>,
    pub username: Option<String>,
    pub os: Option<String>,
    pub arch: Option<String>,
    pub pid: Option<u32>,
    pub internal_ip: Option<String>,
    pub external_ip: Option<String>,
    pub tags: Vec<String>,
    pub sleep_interval: u64,
    pub jitter: u32,
    pub peer_addr: String,
    pub connected_at: u64,
    pub last_seen: u64,
    pub is_online: bool,
    pub is_disabled: bool,
    pub updated_at: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuditRecord {
    pub audit_id: i64,
    pub operator: String,
    pub action: String,
    pub target_kind: String,
    pub target_id: Option<String>,
    pub detail: Option<String>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ListenerKind {
    TcpJson,
    HttpsJson,
    PrivateProto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ListenerRuntimeStatus {
    Starting,
    Running,
    Stopped,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenerRecord {
    pub listener_id: i64,
    pub name: String,
    pub kind: ListenerKind,
    pub bind_host: String,
    pub bind_port: u16,
    pub enabled: bool,
    pub config: Value,
    pub runtime_status: ListenerRuntimeStatus,
    pub last_error: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentBuildStatus {
    Pending,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentBuildRecord {
    pub build_id: i64,
    pub target_triple: String,
    pub profile: String,
    pub listener_id: Option<i64>,
    pub server_addr: String,
    pub embedded_agent_token: bool,
    pub artifact_path: Option<String>,
    pub artifact_name: Option<String>,
    pub status: AgentBuildStatus,
    pub detail: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandSessionStatus {
    Open,
    Closed,
}

#[derive(Debug, Clone, Serialize)]
pub struct CommandSessionSnapshot {
    pub command_session_id: String,
    pub agent_id: String,
    pub cwd: String,
    pub status: CommandSessionStatus,
    pub created_by: String,
    pub created_at: u64,
    pub last_active_at: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct CommandSessionResult {
    pub command_session_id: String,
    pub agent_id: String,
    pub request_id: String,
    pub line: String,
    pub cwd_before: String,
    pub cwd_after: String,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
    pub finished_at: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandOutputStream {
    Stdout,
    Stderr,
}

#[derive(Debug, Clone, Serialize)]
pub struct CommandOutputChunk {
    pub command_session_id: String,
    pub request_id: String,
    pub stream: CommandOutputStream,
    pub chunk: String,
    pub sequence: u32,
    pub emitted_at: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandExecutionStatus {
    Queued,
    Dispatched,
    Running,
    Succeeded,
    Failed,
    Cancelled,
    Dropped,
}

#[derive(Debug, Clone, Serialize)]
pub struct CommandExecutionSnapshot {
    pub command_id: String,
    pub command_session_id: String,
    pub agent_id: String,
    pub line: String,
    pub status: CommandExecutionStatus,
    pub queued_at: u64,
    pub updated_at: u64,
    pub dispatched_at: Option<u64>,
    pub started_at: Option<u64>,
    pub finished_at: Option<u64>,
    pub cwd_before: Option<String>,
    pub cwd_after: Option<String>,
    pub exit_code: Option<i32>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub success: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    Dispatched,
    Running,
    CancelRequested,
    Succeeded,
    Failed,
    Cancelled,
    Partial,
}

#[derive(Debug, Clone, Serialize)]
pub struct TaskSnapshot {
    pub task_id: String,
    pub parent_task_id: Option<String>,
    pub target_agent_id: Option<String>,
    pub command: String,
    pub payload: Option<String>,
    pub status: TaskStatus,
    pub created_at: u64,
    pub updated_at: u64,
    pub success: Option<bool>,
    pub output: Option<String>,
    pub children: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WebEvent {
    Snapshot {
        agents: Vec<AgentSnapshot>,
    },
    AgentConnected {
        session_id: u64,
        peer_addr: String,
        connected_at: u64,
    },
    AgentRegistered {
        agent: AgentSnapshot,
    },
    AgentHeartbeat {
        session_id: u64,
        agent_id: Option<String>,
        last_seen: u64,
    },
    AgentUpdated {
        agent: AgentSnapshot,
    },
    AgentDisconnected {
        session_id: u64,
        agent_id: Option<String>,
    },
    TaskDispatched {
        task_id: String,
        target_agent_id: String,
        command: String,
        payload: Option<String>,
    },
    TaskResult {
        task_id: String,
        agent_id: Option<String>,
        success: bool,
        output: String,
    },
    TaskCancelledRequested {
        task_id: String,
        target_agent_id: Option<String>,
    },
    TaskUpdated {
        task: TaskSnapshot,
    },
    AgentDisconnectRequested {
        agent_id: String,
    },
    AgentDeleted {
        agent_id: String,
    },
    AgentDisabled {
        agent_id: String,
    },
    AgentEnabled {
        agent_id: String,
    },
    CommandSessionOpened {
        session: CommandSessionSnapshot,
    },
    CommandSessionUpdated {
        session: CommandSessionSnapshot,
    },
    CommandSessionClosed {
        command_session_id: String,
        agent_id: String,
    },
    CommandSessionResult {
        result: CommandSessionResult,
    },
    CommandOutputChunk {
        chunk: CommandOutputChunk,
    },
    CommandUpdated {
        command: CommandExecutionSnapshot,
    },
    AgentBuildCreated {
        build: AgentBuildRecord,
    },
    AgentBuildCompleted {
        build: AgentBuildRecord,
    },
}

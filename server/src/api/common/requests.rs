use super::*;

#[derive(Deserialize)]
pub(crate) struct DispatchTaskRequest {
    pub(crate) command: String,
    pub(crate) payload: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct AgentBeaconConfigRequest {
    pub(crate) sleep_interval: u64,
    pub(crate) jitter: u32,
}

#[derive(Deserialize)]
pub(crate) struct AgentHistoryQuery {
    pub(crate) online: Option<bool>,
    pub(crate) disabled: Option<bool>,
    pub(crate) keyword: Option<String>,
    pub(crate) tag: Option<String>,
    pub(crate) limit: Option<usize>,
    pub(crate) offset: Option<usize>,
}

#[derive(Deserialize)]
pub(crate) struct TaskListQuery {
    pub(crate) status: Option<String>,
    pub(crate) agent_id: Option<String>,
    pub(crate) command: Option<String>,
    pub(crate) keyword: Option<String>,
    pub(crate) limit: Option<usize>,
    pub(crate) offset: Option<usize>,
}

#[derive(Deserialize)]
pub(crate) struct CommandSessionListQuery {
    pub(crate) agent_id: Option<String>,
    pub(crate) status: Option<String>,
    pub(crate) limit: Option<usize>,
    pub(crate) offset: Option<usize>,
}

#[derive(Deserialize)]
pub(crate) struct CommandLineRequest {
    pub(crate) line: String,
}

#[derive(Deserialize)]
pub(crate) struct WebTerminalOpenRequest {
    pub(crate) agent_id: String,
}

#[derive(Deserialize)]
pub(crate) struct WebTerminalCommandRequest {
    pub(crate) session_id: String,
    pub(crate) line: String,
}

#[derive(Deserialize)]
pub(crate) struct WebTerminalCloseRequest {
    pub(crate) session_id: String,
}

#[derive(Deserialize)]
pub(crate) struct AuthLoginRequest {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Deserialize)]
pub(crate) struct CommandExecutionListQuery {
    pub(crate) limit: Option<usize>,
    pub(crate) offset: Option<usize>,
}

#[derive(Deserialize)]
pub(crate) struct ListenerListQuery {
    pub(crate) enabled: Option<bool>,
    pub(crate) kind: Option<String>,
    pub(crate) keyword: Option<String>,
    pub(crate) limit: Option<usize>,
    pub(crate) offset: Option<usize>,
}

#[derive(Deserialize)]
pub(crate) struct ListenerCreateRequest {
    pub(crate) name: String,
    pub(crate) kind: String,
    pub(crate) bind_host: String,
    pub(crate) bind_port: u16,
    #[serde(default)]
    pub(crate) enabled: bool,
    #[serde(default)]
    pub(crate) config: serde_json::Value,
}

#[derive(Deserialize)]
pub(crate) struct ListenerUpdateRequest {
    pub(crate) name: Option<String>,
    pub(crate) bind_host: Option<String>,
    pub(crate) bind_port: Option<u16>,
    pub(crate) config: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub(crate) struct AgentBuildListQuery {
    pub(crate) status: Option<String>,
    pub(crate) target_triple: Option<String>,
    pub(crate) limit: Option<usize>,
    pub(crate) offset: Option<usize>,
}

#[derive(Deserialize)]
pub(crate) struct FileUploadRequest {
    pub(crate) remote_path: String,
    pub(crate) content_base64: String,
}

#[derive(Deserialize)]
pub(crate) struct FileDownloadRequest {
    pub(crate) remote_path: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct BrowseFileRequest {
    pub(crate) path: String,
}

#[derive(Deserialize)]
pub(crate) struct AgentBuildCreateRequest {
    pub(crate) target_triple: Option<String>,
    pub(crate) listener_id: Option<i64>,
    pub(crate) server_addr: Option<String>,
    pub(crate) agent_token: Option<String>,
    #[serde(default = "default_build_profile")]
    pub(crate) profile: String,
    pub(crate) heartbeat_secs: Option<u64>,
    pub(crate) jitter: Option<u32>,
}

#[derive(Deserialize)]
pub(crate) struct ListenerAgentBuildRequest {
    pub(crate) target_triple: Option<String>,
    pub(crate) agent_token: Option<String>,
    #[serde(default = "default_build_profile")]
    pub(crate) profile: String,
    pub(crate) heartbeat_secs: Option<u64>,
    pub(crate) jitter: Option<u32>,
}

#[derive(Deserialize, Default)]
pub(crate) struct WsAuthQuery {
    pub(crate) api_token: Option<String>,
    pub(crate) session_token: Option<String>,
}

pub(crate) fn parse_task_status(raw: &str) -> Option<TaskStatus> {
    match raw {
        "pending" => Some(TaskStatus::Pending),
        "dispatched" => Some(TaskStatus::Dispatched),
        "running" => Some(TaskStatus::Running),
        "cancel_requested" => Some(TaskStatus::CancelRequested),
        "succeeded" => Some(TaskStatus::Succeeded),
        "failed" => Some(TaskStatus::Failed),
        "cancelled" => Some(TaskStatus::Cancelled),
        "partial" => Some(TaskStatus::Partial),
        _ => None,
    }
}

pub(crate) fn parse_command_session_status(
    raw: &str,
) -> Option<crate::protocol::CommandSessionStatus> {
    match raw {
        "open" => Some(crate::protocol::CommandSessionStatus::Open),
        "closed" => Some(crate::protocol::CommandSessionStatus::Closed),
        _ => None,
    }
}

pub(crate) fn parse_listener_kind(raw: &str) -> Option<ListenerKind> {
    match raw {
        "tcp_json" => Some(ListenerKind::TcpJson),
        "https_json" => Some(ListenerKind::HttpsJson),
        "private_proto" => Some(ListenerKind::PrivateProto),
        _ => None,
    }
}

pub(crate) fn parse_agent_build_status(raw: &str) -> Option<crate::protocol::AgentBuildStatus> {
    match raw {
        "pending" => Some(crate::protocol::AgentBuildStatus::Pending),
        "succeeded" => Some(crate::protocol::AgentBuildStatus::Succeeded),
        "failed" => Some(crate::protocol::AgentBuildStatus::Failed),
        _ => None,
    }
}

fn default_build_profile() -> String {
    "release".to_string()
}

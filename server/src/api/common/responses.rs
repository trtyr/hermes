use super::*;

#[derive(Serialize)]
pub(crate) struct AgentsResponse {
    pub(crate) agents: Vec<crate::protocol::AgentSnapshot>,
}

#[derive(Serialize)]
pub(crate) struct AgentRecordsResponse {
    pub(crate) agents: Vec<crate::protocol::AgentRecord>,
    pub(crate) total: usize,
    pub(crate) limit: usize,
    pub(crate) offset: usize,
}

#[derive(Serialize)]
pub(crate) struct TasksResponse {
    pub(crate) tasks: Vec<crate::protocol::TaskSnapshot>,
    pub(crate) total: usize,
    pub(crate) limit: usize,
    pub(crate) offset: usize,
}

#[derive(Serialize)]
pub(crate) struct CommandSessionsResponse {
    pub(crate) sessions: Vec<crate::protocol::CommandSessionSnapshot>,
    pub(crate) total: usize,
    pub(crate) limit: usize,
    pub(crate) offset: usize,
}

#[derive(Serialize)]
pub(crate) struct CommandExecutionsResponse {
    pub(crate) commands: Vec<crate::protocol::CommandExecutionSnapshot>,
    pub(crate) total: usize,
    pub(crate) limit: usize,
    pub(crate) offset: usize,
}

#[derive(Serialize)]
pub(crate) struct ListenersResponse {
    pub(crate) listeners: Vec<ListenerRecord>,
    pub(crate) total: usize,
    pub(crate) limit: usize,
    pub(crate) offset: usize,
}

#[derive(Serialize)]
pub(crate) struct ListenerCreateResponse {
    pub(crate) success: bool,
    pub(crate) detail: String,
    pub(crate) listener: ListenerRecord,
}

#[derive(Serialize)]
pub(crate) struct AgentBuildsResponse {
    pub(crate) builds: Vec<crate::protocol::AgentBuildRecord>,
    pub(crate) total: usize,
    pub(crate) limit: usize,
    pub(crate) offset: usize,
}

#[derive(Serialize)]
pub(crate) struct AgentBuildCreateResponse {
    pub(crate) success: bool,
    pub(crate) detail: String,
    pub(crate) build: crate::protocol::AgentBuildRecord,
}

#[derive(Serialize)]
pub(crate) struct DashboardOverviewResponse {
    pub(crate) generated_at: u64,
    pub(crate) server: HostOpsSummary,
    pub(crate) agents: AgentOverviewSummary,
    pub(crate) listeners: ListenerOverviewSummary,
}

#[derive(Serialize)]
pub(crate) struct HostOpsSummary {
    pub(crate) hostname: Option<String>,
    pub(crate) os_name: Option<String>,
    pub(crate) os_version: Option<String>,
    pub(crate) kernel_version: Option<String>,
    pub(crate) architecture: Option<String>,
    pub(crate) uptime_seconds: u64,
    pub(crate) cpu_cores: Option<usize>,
    pub(crate) load_average: HostLoadAverage,
    pub(crate) memory: HostMemorySummary,
}

#[derive(Serialize)]
pub(crate) struct HostLoadAverage {
    pub(crate) one: f64,
    pub(crate) five: f64,
    pub(crate) fifteen: f64,
}

#[derive(Serialize)]
pub(crate) struct HostMemorySummary {
    pub(crate) total_bytes: u64,
    pub(crate) used_bytes: u64,
    pub(crate) available_bytes: u64,
}

#[derive(Serialize)]
pub(crate) struct AgentOverviewSummary {
    pub(crate) total: usize,
    pub(crate) online: usize,
    pub(crate) offline: usize,
    pub(crate) disabled: usize,
    pub(crate) connected_sessions: usize,
}

#[derive(Serialize)]
pub(crate) struct ListenerOverviewSummary {
    pub(crate) total: usize,
    pub(crate) enabled: usize,
    pub(crate) disabled: usize,
    pub(crate) running: usize,
    pub(crate) stopped: usize,
    pub(crate) starting: usize,
    pub(crate) error: usize,
    pub(crate) by_kind: ListenerKindSummary,
}

#[derive(Serialize, Default)]
pub(crate) struct ListenerKindSummary {
    pub(crate) tcp_json: usize,
    pub(crate) https_json: usize,
    pub(crate) private_proto: usize,
}

#[derive(Serialize)]
pub(crate) struct ApiResponse {
    pub(crate) success: bool,
    pub(crate) detail: String,
    pub(crate) task_id: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct AuthLoginResponse {
    pub(crate) success: bool,
    pub(crate) detail: String,
    pub(crate) session_token: String,
    pub(crate) username: String,
    pub(crate) expires_at: u64,
}

#[derive(Serialize)]
pub(crate) struct AuthMeResponse {
    pub(crate) authenticated: bool,
    pub(crate) username: String,
    pub(crate) expires_at: u64,
}

#[derive(Serialize)]
pub(crate) struct AgentMutationResponse {
    pub(crate) success: bool,
    pub(crate) detail: String,
    pub(crate) agent: crate::protocol::AgentRecord,
}

#[derive(Serialize)]
pub(crate) struct CommandSessionCreateResponse {
    pub(crate) success: bool,
    pub(crate) detail: String,
    pub(crate) session: crate::protocol::CommandSessionSnapshot,
}

#[derive(Serialize)]
pub(crate) struct CommandSessionExecuteResponse {
    pub(crate) success: bool,
    pub(crate) detail: String,
    pub(crate) result: crate::protocol::CommandSessionResult,
}

#[derive(Serialize)]
pub(crate) struct CommandQueueResponse {
    pub(crate) success: bool,
    pub(crate) detail: String,
    pub(crate) command: crate::protocol::CommandExecutionSnapshot,
}

#[derive(Serialize)]
pub(crate) struct WebTerminalResponse<T> {
    pub(crate) success: bool,
    pub(crate) message: String,
    pub(crate) data: T,
}

#[derive(Serialize)]
pub(crate) struct WebTerminalSessionData {
    pub(crate) session_id: String,
    pub(crate) cwd: String,
    pub(crate) status: String,
}

#[derive(Serialize)]
pub(crate) struct WebTerminalCommandData {
    pub(crate) session_id: String,
    pub(crate) command_id: String,
    pub(crate) state: String,
}

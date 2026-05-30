use super::*;

pub struct KernelState {
    pub(super) sessions: HashMap<u64, AgentSession>,
    pub(super) agent_index: HashMap<String, u64>,
    pub(super) tasks: HashMap<String, TaskRecord>,
    pub(super) pending_task_chunks: HashMap<String, Vec<(u32, String)>>,
    pub(super) command_sessions: HashMap<String, CommandSessionRecord>,
    pub(super) command_executions: HashMap<String, CommandExecutionRecord>,
    pub(super) proxy_sessions: HashMap<String, ProxySessionRecord>,
    pub(super) proxy_streams: HashMap<String, ProxyStreamRecord>,
    pub(super) pending_open_command_sessions:
        HashMap<String, oneshot::Sender<anyhow::Result<CommandSessionSnapshot>>>,
    pub(super) pending_command_executes: HashMap<String, PendingCommandExecute>,
    pub(super) pending_close_command_sessions:
        HashMap<String, oneshot::Sender<anyhow::Result<CommandSessionSnapshot>>>,
    pub(super) pending_agent_beacon_updates: HashMap<String, PendingAgentBeaconUpdate>,
    pub(super) pending_proxy_stream_opens: HashMap<String, oneshot::Sender<anyhow::Result<()>>>,
    pub(super) pending_proxy_session_starts:
        HashMap<String, oneshot::Sender<anyhow::Result<ProxySessionSnapshot>>>,
    pub(super) pending_proxy_session_stops:
        HashMap<String, oneshot::Sender<anyhow::Result<ProxySessionSnapshot>>>,
}

pub struct AgentSession {
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
    pub tags: Vec<String>,
    pub sleep_interval: u64,
    pub jitter: u32,
    pub peer_addr: SocketAddr,
    pub connected_at: u64,
    pub last_seen: u64,
    pub sender: mpsc::UnboundedSender<ServerCommand>,
    pub privilege: String,
}

#[derive(Clone)]
pub struct TaskRecord {
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

pub struct AgentIdentity {
    pub agent_id: String,
    pub hostname: String,
    pub username: Option<String>,
    pub os: Option<String>,
    pub arch: Option<String>,
    pub pid: Option<u32>,
    pub internal_ip: Option<String>,
    pub tags: Vec<String>,
    pub sleep_interval: u64,
    pub jitter: u32,
    pub last_seen: u64,
    pub privilege: String,
}

pub struct NewTask {
    pub task_id: String,
    pub parent_task_id: Option<String>,
    pub target_agent_id: Option<String>,
    pub command: String,
    pub payload: Option<String>,
    pub created_at: u64,
}

#[derive(Clone)]
pub struct CommandSessionRecord {
    pub command_session_id: String,
    pub agent_id: String,
    pub cwd: String,
    pub status: CommandSessionStatus,
    pub created_by: String,
    pub created_at: u64,
    pub last_active_at: u64,
    pub active_command_id: Option<String>,
    pub queued_command_ids: VecDeque<String>,
}

#[derive(Clone)]
pub struct CommandExecutionRecord {
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

#[derive(Clone)]
pub struct ProxySessionRecord {
    pub proxy_id: String,
    pub agent_id: String,
    pub bind_addr: String,
    pub status: ProxySessionStatus,
    pub active_stream_ids: HashSet<String>,
    pub created_at: u64,
    pub updated_at: u64,
    pub last_error: Option<String>,
}

pub struct ProxyStreamRecord {
    pub stream_id: String,
    pub proxy_id: String,
    pub target_host: String,
    pub target_port: u16,
    pub client_sender: mpsc::UnboundedSender<Option<Vec<u8>>>,
}

pub(super) struct PendingCommandExecute {
    pub(super) command_id: String,
    pub(super) sender: oneshot::Sender<anyhow::Result<CommandSessionResult>>,
}

pub(super) struct PendingAgentBeaconUpdate {
    pub(super) agent_id: String,
    pub(super) sender: oneshot::Sender<anyhow::Result<AgentSnapshot>>,
}

impl KernelState {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            agent_index: HashMap::new(),
            tasks: HashMap::new(),
            pending_task_chunks: HashMap::new(),
            command_sessions: HashMap::new(),
            command_executions: HashMap::new(),
            proxy_sessions: HashMap::new(),
            proxy_streams: HashMap::new(),
            pending_open_command_sessions: HashMap::new(),
            pending_command_executes: HashMap::new(),
            pending_close_command_sessions: HashMap::new(),
            pending_agent_beacon_updates: HashMap::new(),
            pending_proxy_stream_opens: HashMap::new(),
            pending_proxy_session_starts: HashMap::new(),
            pending_proxy_session_stops: HashMap::new(),
        }
    }
}

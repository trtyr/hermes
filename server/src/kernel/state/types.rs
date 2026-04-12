use super::*;

pub struct KernelState {
    pub(super) sessions: HashMap<u64, AgentSession>,
    pub(super) agent_index: HashMap<String, u64>,
    pub(super) tasks: HashMap<String, TaskRecord>,
    pub(super) command_sessions: HashMap<String, CommandSessionRecord>,
    pub(super) command_executions: HashMap<String, CommandExecutionRecord>,
    pub(super) pending_open_command_sessions:
        HashMap<String, oneshot::Sender<anyhow::Result<CommandSessionSnapshot>>>,
    pub(super) pending_command_executes: HashMap<String, PendingCommandExecute>,
    pub(super) pending_close_command_sessions:
        HashMap<String, oneshot::Sender<anyhow::Result<CommandSessionSnapshot>>>,
    pub(super) pending_agent_beacon_updates: HashMap<String, PendingAgentBeaconUpdate>,
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
            command_sessions: HashMap::new(),
            command_executions: HashMap::new(),
            pending_open_command_sessions: HashMap::new(),
            pending_command_executes: HashMap::new(),
            pending_close_command_sessions: HashMap::new(),
            pending_agent_beacon_updates: HashMap::new(),
        }
    }
}

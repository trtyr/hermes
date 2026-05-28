# 附录C 关键代码

本附录收录论文第四章系统设计中所引用的核心数据结构定义和关键实现代码，供读者参考。正文中不再出现完整代码块，仅以文字描述设计思路。

## C.1 心跳抖动算法实现（§4.3.3）

Agent 端 HeartbeatService 的 `schedule_from` 方法负责计算下一次心跳的到期时间，采用三因子混合伪随机种子生成抖动延迟：

```rust
fn schedule_from(&mut self, now: Instant) {
    let base_ms = self.interval_secs.saturating_mul(1000);
    let max_jitter_ms = base_ms.saturating_mul(self.jitter as u64) / 100;
    let jitter_ms = if max_jitter_ms == 0 {
        0
    } else {
        self.sequence = self.sequence.wrapping_add(1);
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0)
            ^ ((std::process::id() as u64) << 16)
            ^ self.sequence;
        seed % (max_jitter_ms + 1)
    };
    self.next_due = now + Duration::from_millis(base_ms.saturating_add(jitter_ms));
}
```

随机种子由当前时间戳（纳秒级）、进程 ID 和递增序列号三个因子通过异或混合生成，保证不同 Agent 实例之间、同一 Agent 的连续心跳之间均产生不同的抖动值。

## C.2 微内核消息定义（§4.4.1）

内核统一消息类型 `KernelMessage` 按业务域组织为四类：

```rust
pub enum KernelMessage {
    Agent(AgentKernelMessage),
    Task(TaskKernelMessage),
    CommandSession(CommandSessionKernelMessage),
    Proxy(ProxyKernelMessage),
}
```

以 Agent 域为例，其消息变体覆盖 Agent 从连接到退出的完整生命周期：

```rust
pub enum AgentKernelMessage {
    Connected { session_id, listener_id, peer_addr, sender },
    Disconnected { session_id },
    Frame { session_id, frame: AgentMessage },
    UpdateBeaconConfig { agent_id, sleep_interval, jitter, respond_to },
    SweepHeartbeats,
}
```

其中 `Connected` 消息携带一个 `mpsc` 通道 `sender`，用于后续向该 Agent 发送 `ServerCommand`；`UpdateBeaconConfig` 通过 `oneshot` 通道实现同步等待语义。

## C.3 内核状态结构（§4.4.1）

`KernelState` 是 Server 在内存中的权威状态容器，由一系列 HashMap 组成：

```rust
pub struct KernelState {
    sessions: HashMap<u64, AgentSession>,      // session_id → 会话
    agent_index: HashMap<String, u64>,          // agent_id → session_id
    tasks: HashMap<String, TaskRecord>,
    command_sessions: HashMap<String, CommandSessionRecord>,
    command_executions: HashMap<String, CommandExecutionRecord>,
    proxy_sessions: HashMap<String, ProxySessionRecord>,
    proxy_streams: HashMap<String, ProxyStreamRecord>,
    pending_open_command_sessions: HashMap<String, oneshot::Sender<...>>,
    pending_command_executes: HashMap<String, PendingCommandExecute>,
    // ... 其他 pending 映射
}
```

`sessions` 和 `agent_index` 构成 Agent 的双向索引。`pending_*` 系列 HashMap 存储异步操作的 `oneshot` 回复通道，用于实现请求-响应的同步等待。

## C.4 副作用出口结构（§4.4.1）

```rust
pub struct RuntimePorts {
    publisher: EventBus,   // 事件广播至 WebSocket 订阅者
    persistence: Storage,  // SQLite 数据持久化
}
```

Runtime 在完成状态变更后通过 `RuntimePorts` 执行副作用，将状态计算与副作用执行分离，便于单元测试。

## C.5 监听器数据模型（§4.4.2）

```rust
pub struct ListenerRecord {
    pub listener_id: i64,
    pub name: String,
    pub kind: ListenerKind,         // TcpJson / HttpsJson / PrivateProto
    pub bind_host: String,
    pub bind_port: u16,
    pub enabled: bool,
    pub config: Value,              // JSON 配置
    pub runtime_status: ListenerRuntimeStatus,  // Starting / Running / Stopped / Error
    pub last_error: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}
```

## C.6 Agent 会话数据模型（§4.4.3）

```rust
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
```

`sender` 字段是 Server 向 Agent 发送指令的唯一通道；`last_seen` 在每次收到 Agent 消息时更新，供 Watchdog 超时检测使用。

## C.7 任务记录数据模型（§4.4.4）

```rust
pub struct TaskRecord {
    pub task_id: String,             // 格式 task_{seq}
    pub parent_task_id: Option<String>,  // 广播任务的父任务 ID
    pub target_agent_id: Option<String>,
    pub command: String,
    pub payload: Option<String>,
    pub status: TaskStatus,          // pending / dispatched / completed / failed / cancelled
    pub created_at: u64,
    pub updated_at: u64,
    pub success: Option<bool>,
    pub output: Option<String>,
    pub children: Vec<String>,       // 子任务 ID 列表
}
```

`parent_task_id` 和 `children` 字段支持任务嵌套：广播任务为每个在线 Agent 生成一个子任务，子任务指向父广播任务。

## C.8 命令会话数据模型（§4.4.5）

```rust
pub struct CommandSessionRecord {
    pub command_session_id: String,
    pub agent_id: String,
    pub cwd: String,                           // 当前工作目录
    pub status: CommandSessionStatus,
    pub created_by: String,
    pub created_at: u64,
    pub last_active_at: u64,
    pub active_command_id: Option<String>,      // 当前执行的命令 ID
    pub queued_command_ids: VecDeque<String>,   // 排队等待的命令 ID 队列
}
```

`active_command_id` 和 `queued_command_ids` 实现了命令队列机制，同一时刻只有一个命令在执行，后续命令排队等待。

## C.9 代理转发数据模型（§4.4.6）

```rust
pub struct ProxySessionRecord {
    pub proxy_id: String,
    pub agent_id: String,
    pub bind_addr: String,                   // Server 端 SOCKS5 绑定地址
    pub status: ProxySessionStatus,          // opening / open / closed / error
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
```

一个 `ProxySession` 可包含多条并发的 `ProxyStream`，每条 Stream 对应一个到内网目标主机的 TCP 连接。

## C.10 Agent 构建任务数据模型（§4.4.7）

```rust
pub struct AgentBuildRecord {
    pub build_id: i64,
    pub target_triple: String,          // 目标平台三元组
    pub profile: String,                // release / debug
    pub listener_id: Option<i64>,
    pub server_addr: String,
    pub embedded_agent_token: bool,
    pub artifact_path: Option<String>,
    pub artifact_name: Option<String>,
    pub status: AgentBuildStatus,       // Pending / Succeeded / Failed
    pub detail: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}
```

## C.11 Agent 指令分发逻辑（§4.6.2）

Agent 主循环中对 `ServerCommand` 的分发通过 `match` 表达式实现：

```rust
match cmd {
    ServerCommand::DispatchTask { task_id, command, payload } => {
        task.lock().unwrap().dispatch(&task_id, &command, payload.as_deref());
    }
    ServerCommand::OpenCommandSession { command_session_id } => {
        session.lock().unwrap().open(&command_session_id);
    }
    ServerCommand::ExecuteCommandSession { command_session_id, request_id, line } => {
        session.lock().unwrap().execute(&command_session_id, &request_id, &line);
    }
    ServerCommand::CloseCommandSession { command_session_id } => {
        session.lock().unwrap().close(&command_session_id);
    }
    ServerCommand::Disconnect { .. } => return Ok(()),
    // ... Proxy 相关命令类似
}
```

## C.12 通信协议消息示例（§4.3.2）

Agent 注册消息（JSON 格式）：

```json
{"type":"register","agent_id":"DESKTOP-A1B2C3","hostname":"DESKTOP-A1B2C3","username":"admin",
 "protocol_version":1,"os":"Windows 10","arch":"x86_64","pid":4812,
 "internal_ip":"192.168.1.105","privilege":"Medium","tags":[],
 "sleep_interval":15,"jitter":20}
```

Server 握手回应消息（JSON 格式）：

```json
{"type":"hello","protocol_version":1,"session_nonce":"a3f2b1c4",
 "listener_id":1,"listener_name":"default","transport":"tcp",
 "capabilities":["task","session","proxy"],"auth_mode":"none"}
```


## C.13 微内核运行时与消息调度核心代码（§4.4.1）

以下代码展示服务端微内核如何用统一消息枚举、消息总线、内核状态和运行时循环组织控制平面的状态变更。

```rust
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
    Disconnected { session_id: u64 },
    Frame { session_id: u64, frame: AgentMessage },
    UpdateBeaconConfig { /* ... */ },
    SweepHeartbeats,
}

#[derive(Clone)]
pub struct KernelBus {
    message_sender: mpsc::Sender<KernelMessage>,
}

pub struct KernelState {
    pub(super) sessions: HashMap<u64, AgentSession>,
    pub(super) agent_index: HashMap<String, u64>,
    pub(super) tasks: HashMap<String, TaskRecord>,
    pub(super) command_sessions: HashMap<String, CommandSessionRecord>,
    pub(super) command_executions: HashMap<String, CommandExecutionRecord>,
    pub(super) proxy_sessions: HashMap<String, ProxySessionRecord>,
    pub(super) proxy_streams: HashMap<String, ProxyStreamRecord>,
    // ... pending maps omitted
}

pub(super) async fn kernel_loop(
    mut receiver: mpsc::Receiver<KernelMessage>,
    state: Arc<RwLock<KernelState>>,
    events: EventBus,
    storage: Storage,
) {
    let effects = RuntimePorts::new(events, storage);
    while let Some(message) = receiver.recv().await {
        match message {
            KernelMessage::Agent(msg) => {
                route_agent_message(&state, &effects, msg).await;
            }
            KernelMessage::Task(msg) => {
                route_task_message(&state, &effects, msg).await;
            }
            KernelMessage::CommandSession(msg) => {
                route_command_session_message(&state, &effects, msg).await;
            }
            KernelMessage::Proxy(msg) => {
                route_proxy_message(&state, &effects, msg).await;
            }
        }
    }
}
```

## C.14 HTTP API路由与Handler实现（§4.1.3、§4.4.1）

以下代码展示 Axum 路由集中注册方式，以及 API Handler 仅完成认证、调用外观接口和返回响应的适配层职责。

```rust
let app = Router::new()
    .route("/health", get(system::health))
    .route("/auth/login", post(auth::login))
    .route("/agents", get(agents::list_agents))
    .route("/agents/{agent_id}/tasks", post(agents::dispatch_task))
    .route("/agents/{agent_id}/upload", post(agents::upload_file))
    .route("/agents/{agent_id}/download", post(agents::download_file))
    .route("/agents/{agent_id}/browse", post(agents::browse_file))
    .route(
        "/command-sessions/{session_id}/execute",
        post(command_sessions::execute_command_session),
    )
    .route(
        "/command-sessions/{session_id}/close",
        post(command_sessions::close_command_session),
    )
    .route("/events/ws", get(system::ws_events))
    .route("/web/terminal/ws", get(web_terminal::terminal_ws_events))
    // ... more routes
    .layer(middleware::from_fn(log_http_request))
    .layer(cors)
    .with_state(AppState { kernel });

pub async fn list_agents(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let agents = state.kernel.agent_queries().snapshots().await;
    Json(agents)
}
```

## C.15 WebSocket实时推送实现（§4.5.2）

以下代码展示 WebSocket 连接建立后先推送当前快照，再持续订阅事件总线并向浏览器转发状态变更。

```rust
async fn handle_ws(mut socket: WebSocket, state: AppState) {
    let snapshot = WebEvent::Snapshot {
        agents: state.kernel.agent_queries().snapshots().await,
    };
    if send_ws_event(&mut socket, &snapshot).await.is_err() {
        return;
    }

    let mut event_rx = state.kernel.events().subscribe();

    loop {
        tokio::select! {
            event = event_rx.recv() => {
                match event {
                    Ok(payload) => {
                        if socket.send(Message::Text(payload.into())).await.is_err() {
                            break;
                        }
                    }
                    Err(RecvError::Lagged(_)) => continue,
                    Err(RecvError::Closed) => break,
                }
            }
            inbound = socket.recv() => {
                match inbound {
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }
        }
    }
}
```

## C.16 Agent注册消息与会话登记实现（§4.3.2、§4.4.3）

以下代码展示 Agent 注册消息在协议层的结构。服务端收到该消息后补全会话记录、建立索引并广播上线事件。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AgentMessage {
    Register {
        agent_id: String,
        hostname: String,
        username: Option<String>,
        os: Option<String>,
        arch: Option<String>,
        pid: Option<u32>,
        internal_ip: Option<String>,
        sleep_interval: u64,
        jitter: u32,
        token: Option<String>,
        privilege: String,
        // ... other fields
    },
    Heartbeat { agent_id: Option<String> },
    TaskResult { task_id: String, success: bool, output: String },
    // ... other variants
}
```

## C.17 交互式终端会话打开与队列状态（§4.4.5、§4.5.3）

以下代码展示 Web 终端打开会话时的服务端入口，以及内核保存交互式会话状态所需的核心字段。

```rust
pub(crate) async fn open_terminal_session(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<WebTerminalOpenRequest>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    if !state.kernel.agent_queries().is_connected(&request.agent_id).await {
        return (StatusCode::CONFLICT, Json(/* error */)).into_response();
    }
    match state
        .kernel
        .command_sessions()
        .open(request.agent_id.clone(), operator)
        .await
    {
        Ok(session) => (StatusCode::CREATED, Json(/* session data */)).into_response(),
        Err(error) => web_terminal_error_response(error),
    }
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
```

## C.18 Agent二进制构建服务实现（§4.4.7）

以下代码展示构建服务先创建构建记录，再将实际编译过程放入后台异步任务的实现方式。

```rust
pub async fn build_agent_binary(
    &self,
    target_triple: Option<String>,
    listener_id: Option<i64>,
    server_addr: Option<String>,
    agent_token: Option<String>,
    profile: String,
    heartbeat_secs: Option<u64>,
    jitter: Option<u32>,
) -> anyhow::Result<AgentBuildRecord> {
    // Resolve target triple, listener, server_addr...
    let build = self
        .kernel
        .storage
        .create_agent_build_record(/* ... */)
        .await?;
    self.kernel.publish_web_event(WebEvent::AgentBuildCreated {
        build: build.clone(),
    });
    let kernel = self.kernel.clone();
    tokio::spawn(async move {
        let result = run_build(/* params */).await;
        // Update build record and notify WebSocket
    });
    Ok(build)
}
```

## C.19 服务端代理转发消息模型（§4.4.6）

以下代码展示服务端代理转发模块的内核消息结构。代理会话、代理流、客户端数据和客户端关闭事件均通过同一消息通道进入内核。

```rust
#[derive(Debug)]
pub enum ProxyKernelMessage {
    StartSession {
        agent_id: String,
        proxy_id: String,
        bind_addr: String,
        respond_to: oneshot::Sender<anyhow::Result<ProxySessionSnapshot>>,
    },
    StopSession { proxy_id: String, respond_to: /* ... */ },
    OpenStream {
        proxy_id: String,
        stream_id: String,
        host: String,
        port: u16,
        client_sender: mpsc::UnboundedSender<Option<Vec<u8>>>,
        respond_to: oneshot::Sender<anyhow::Result<()>>,
    },
    ClientData { proxy_id: String, stream_id: String, data: Vec<u8> },
    ClientClosed { proxy_id: String, stream_id: String },
}
```

## C.20 前端交互式终端消息处理（§4.5.3）

以下代码展示前端终端组件如何通过组合函数获得终端容器与连接状态，并根据 WebSocket 消息更新终端输出。

```rust
const { terminalContainer, sessionId, wsConnected } = useTerminal(agentId.value);

ws.onmessage = (event) => {
    const payload = JSON.parse(event.data);
    if (payload.type !== 'terminal' || payload.session_id !== sessionId.value) return;
    if (payload.event === 'command' && (payload.state === 'done' || payload.state === 'error')) {
        term?.write('\x1b[1A\x1b[2K');
        if (payload.stdout) {
            term?.write(payload.stdout.replace(/\n/g, '\r\n'));
        }
        if (payload.stderr) {
            term?.write('\x1b[31m' + payload.stderr.replace(/\n/g, '\r\n') + '\x1b[0m');
        }
        if (payload.cwd) cwd.value = payload.cwd;
        onCommandDone();
    }
};
```

## C.21 Agent主循环与心跳调度实现（§4.6.1）

以下代码展示 Agent 启动时的服务初始化、重连循环，以及心跳服务如何根据基础间隔和抖动比例计算下一次发送时间。

```rust
fn main() {
    std::panic::set_hook(Box::new(|panic_info| {
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("agent_debug.log")
        {
            let _ = writeln!(f, "[PANIC] {}", panic_info);
        }
    }));

    let cfg = match Config::load() {
        Ok(c) => c,
        Err(_) => std::process::exit(0),
    };

    let (outbox_tx, outbox_rx) = mpsc::channel::<AgentMessage>();
    let network = Arc::new(Mutex::new(NetworkService::new()));
    let heartbeat = Arc::new(Mutex::new(HeartbeatService::new()));
    // ... init other services

    loop {
        if run_once(/* all services */).is_err() {
            // log failure
        }
        kernel.sleep(Duration::from_secs(cfg.reconnect_secs));
    }
}

fn schedule_from(&mut self, now: Instant) {
    let base_ms = self.interval_secs.saturating_mul(1000);
    let max_jitter_ms = base_ms.saturating_mul(self.jitter as u64) / 100;
    let jitter_ms = if max_jitter_ms == 0 {
        0
    } else {
        self.sequence = self.sequence.wrapping_add(1);
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
            ^ ((std::process::id() as u64) << 16)
            ^ self.sequence;
        seed % (max_jitter_ms + 1)
    };
    self.next_due = now + Duration::from_millis(base_ms.saturating_add(jitter_ms));
}
```

## C.22 Shell命令执行、超时终止与输出解码（§4.6.2、§4.6.5）

以下代码展示 Agent 执行 Shell 命令时的进程启动、任务分发、超时终止和 Windows 输出编码处理。

```rust
fn spawn_shell_process(command: &str, cwd: Option<&str>) -> std::io::Result<std::process::Child> {
    let (program, args) = parse_command(command);
    let mut cmd = std::process::Command::new(&program);
    if !args.is_empty() {
        cmd.raw_arg(&args);
    }
    if let Some(cwd) = cwd {
        cmd.current_dir(cwd);
    }
    cmd.stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    cmd.creation_flags(0x08000000);
    cmd.spawn()
}

pub fn dispatch(&mut self, task_id: &str, command: &str, payload: Option<&str>) {
    if super::file_ops::is_file_op(command) {
        // Route to file_ops handler in separate thread
        return;
    }
    if super::sys_ops::is_sys_op(command) {
        // Route to sys_ops handler
        return;
    }
    // Generic shell command
    let child = match spawn_operation(actual_cmd, None, None) { /* ... */ };
    let pid = child.id();
    let result = wait_child(child, timeout_secs);
    // Send TaskResult back via outbox
}

pub fn terminate_process(pid: u32) -> bool {
    use windows_sys::Win32::Foundation::CloseHandle;
    use windows_sys::Win32::System::Threading::{
        OpenProcess, TerminateProcess, PROCESS_TERMINATE,
    };
    unsafe {
        let handle = OpenProcess(PROCESS_TERMINATE, 0, pid);
        if handle == 0 {
            return false;
        }
        let result = TerminateProcess(handle, 1);
        CloseHandle(handle);
        result != 0
    }
}

pub fn decode_output(raw: &[u8]) -> String {
    if raw.is_empty() {
        return String::new();
    }
    if let Ok(s) = std::str::from_utf8(raw) {
        return s.to_string();
    }
    let (decoded, _, _) = encoding_rs::GBK.decode(raw);
    decoded.to_string()
}
```

## C.23 Agent文件上传处理实现（§4.6.3）

以下代码展示上传任务如何解析 JSON 参数、解码 base64 文件内容并写入目标路径。

```rust
pub fn handle_upload(task_id: &str, payload: &str, sender: &Sender<AgentMessage>) {
    #[derive(serde::Deserialize)]
    struct UploadPayload {
        remote_path: String,
        content_base64: String,
    }
    let parsed: UploadPayload = serde_json::from_str(payload)?;
    let content = STANDARD.decode(&parsed.content_base64)?;
    write_file(&parsed.remote_path, &content)?;
}
```

## C.24 Agent代理流连接与数据回传实现（§4.6.4）

以下代码展示 Agent 收到代理连接请求后，如何连接目标主机、登记数据流并在独立线程中回传目标主机数据。

```rust
pub fn connect(&mut self, proxy_id: &str, stream_id: &str, host: &str, port: u16) {
    match TcpStream::connect((host, port)) {
        Ok(stream) => {
            let read_stream = stream.try_clone().unwrap();
            let stream = Arc::new(Mutex::new(stream));
            // Register in maps
            let _ = self.sender.send(AgentMessage::ProxyConnectResult {
                proxy_id: proxy_id.to_string(),
                stream_id: stream_id.to_string(),
                success: true,
                detail: None,
            });
            std::thread::spawn(move || {
                let mut buf = vec![0u8; 8192];
                loop {
                    match read_stream.read(&mut buf) {
                        Ok(0) => break,
                        Ok(n) => {
                            let _ = sender.send(AgentMessage::ProxyData {
                                proxy_id: proxy_id.clone(),
                                stream_id: stream_id.clone(),
                                data_base64: STANDARD.encode(&buf[..n]),
                            });
                        }
                        Err(_) => break,
                    }
                }
            });
        }
        Err(error) => {
            // Send ProxyConnectResult with failure detail
        }
    }
}
```

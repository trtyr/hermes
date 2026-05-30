# Bug Tracker

## 🔴 OPEN

### BUG-001: 载荷构建完成通知显示 undefined

**页面:** `/payload` (http://localhost:5173/payload)

**现象:** 构建完成后弹窗显示 `Build #undefined — undefined`

**根因分析:**
- Server 发送的 WebSocket 事件结构: `{ type: "agent_build_completed", build: { build_id, status, ... } }`（嵌套在 `build` 字段内）
- 客户端直接取 `event.build_id` 和 `event.status`（扁平结构），导致 undefined
- 已修复 `.ts` 源文件（`store/events.ts`, `store/notifications.ts`, `views/payload/index.vue`）
- 已同步修复对应的 `.js` 文件（`store/notifications.js`, `views/payload/index.vue.js`）
- **但修复未生效**，可能原因：
  - Vite 缓存未刷新
  - `.js` 文件是编译产物，不是源文件，Vite 实际加载的是 `.vue` 文件中的 `<script>` 块
  - 需要进一步排查 Vite 的模块解析逻辑

**涉及文件:**
- `client/src/store/events.ts` (line 22-24) — BackendEvent 类型定义
- `client/src/store/notifications.ts` (line 90-97) — 通知处理
- `client/src/store/notifications.js` (line 71-78) — 编译产物
- `client/src/views/payload/index.vue` (line 397-402) — 弹窗处理
- `client/src/views/payload/index.vue.js` (line 150-156) — 编译产物

**状态:** 待排查

---

### BUG-002: 认证失败的 Agent 触发虚假"离线"通知（通知风暴）

**现象:** Agent token 不匹配时，前端疯狂弹出"Agent 离线 — unknown"通知

**根因分析（多层问题）:**

#### 问题 1: Server 对所有连接结束都发送 Disconnected 事件
- `server/src/agent/listeners/session/mod.rs` (line 185-189)
- 连接结束时**无条件**发送 `AgentKernelMessage::Disconnected`，包括认证失败的连接
- 认证失败的 Agent 从未真正上线，不应触发 disconnect 事件
- **修复方向:** 只在 `registered = true` 时才发送 Disconnected 事件

#### 问题 2: 前端通知无去重/限流机制
- `client/src/store/notifications.ts` — 每个 `agent_disconnected` 事件都创建新通知
- 短时间内大量相同通知会淹没用户
- **修复方向:** 添加通知去重（相同类型+相同 agent_id 在 N 秒内只显示一次）或限流

#### 问题 3: 通知文案不准确
- 认证失败的 Agent 显示"Agent 离线"，但它们从未上线
- `event.agent_id` 为 `unknown`（因为未注册，没有 agent_id）
- **修复方向:** 
  - Server 端区分"认证失败断开"和"正常离线"
  - 前端根据情况显示不同文案（如"Agent 认证失败" vs "Agent 离线"）

#### 问题 4: Agent 重试无退避（次要）
- Agent 被拒绝后立即重试，导致短时间内大量连接
- 这是预期行为（保证网络恢复后自动重连），但加剧了通知风暴
- **修复方向:** Agent 端添加指数退避（被认证拒绝后等待更长时间再重试）

**涉及文件:**
- `server/src/agent/listeners/session/mod.rs` (line 185-189) — 无条件发送 Disconnected
- `client/src/store/notifications.ts` (line 74-81) — 离线通知处理
- `client/src/store/notifications.js` (line 55-62) — 编译产物

**状态:** 待修复

---

### BUG-003: 权限列显示多余的 `Admin:` 前缀

**页面:** `/agent` (http://localhost:5173/agent)

**现象:** 权限列显示 `Admin: SeIncreaseQuotaPrivilege, SeSecurityPrivilege, ...`

**问题:** `Admin:` 前缀是多余的，权限列表本身已经说明了一切

**修复方向:** 去掉 `Admin:` 前缀，直接显示权限列表

**涉及文件（待确认）:**
- `client/src/views/agent/index.vue` — 权限列渲染
- `server/src/protocol.rs` 或 `server/src/kernel/state/` — privilege 字段的来源

**状态:** 待修复

---

### BUG-004: 命令执行失败且错误信息不友好

**页面:** `/agent/:id/session` (终端会话)

**现象:** 
- `ls`、`dir` 等命令执行后显示 `exec failed`，退出码 1
- 错误信息只有 `exec failed`，没有具体原因

**问题:**
1. 命令执行失败原因不明（Agent 端不支持？权限问题？命令格式问题？）
2. 前端错误信息不友好，无法帮助用户判断问题

**修复方向:**
- Agent 端：返回具体的错误原因（如"命令不支持"、"权限不足"、"路径不存在"）
- 前端：显示详细的错误信息，而不是只显示 `exec failed`

**涉及文件（待确认）:**
- `agent/src/services/` — 命令执行逻辑
- `server/src/api/command_sessions/` — 命令会话 API
- `client/src/views/agent/hooks/useTerminalSocket.ts` — 错误显示逻辑

**状态:** 待排查

---

### BUG-005: 文件下载任务完成但浏览器无下载响应

**页面:** `/agent/:id/files` (文件浏览器)

**现象:** 
- 点击文件下载后，通知显示"下载任务已下发"
- 任务完成后通知显示"任务完成 Task task-6"
- 但浏览器没有任何下载响应，文件没有下载到本地

**问题:**
- 不清楚文件传输链路是否完整：
  - 路径 A: Agent → Server → Client（浏览器下载）
  - 路径 B: Agent → Server，但 Server → Client 的传输断了
- 下载任务在 Agent 端完成，但文件可能没有传回给浏览器

**修复方向（待排查）:**
- 检查 Server 端文件下载 API 是否正确返回文件流
- 检查前端下载触发逻辑（是否创建了 `<a>` 标签或使用了 `Blob`）
- 检查 WebSocket 任务完成后是否触发了实际的 HTTP 下载请求

**涉及文件（待确认）:**
- `server/src/api/` — 文件下载 API
- `client/src/views/agent/` — 文件下载触发逻辑
- `client/src/composables/` — 文件操作相关

**状态:** 待排查

---

### BUG-006: 离线通知显示 UUID 而非 hostname

**现象:** 
- Agent 离线通知显示 `e2785793-b4d7-4f79-87d1-f170fd723a16`（数据库 UUID）
- Agent 上线通知显示 `qwpbj9xk04amqt6 (10.0.152.185)`（hostname + IP）
- 用户无法通过离线通知识别是哪个 Agent

**日志证据:**
```
[server] session_id=70: connection read error: Connection reset by peer (os error 104)
[server] connection ended for session_id=70
[server] new connection from 8.139.128.91:46355
[server] agent registered: session_id=71
```
- 断线后 Agent 立即重连（session_id 70→71），断线原因是 TCP reset
- 上线后 API 请求用的是 UUID：`GET /agents/e2785793-b4d7-4f79-87d1-f170fd723a16`

**断线原因分析:**
- `Connection reset by peer (os error 104)` = `ECONNRESET`，TCP 层面收到对端 RST 包
- 不是 server 踢的，是 Agent 端主动断开（或 Agent 进程异常退出）
- 断开后立即重连，说明大概率是 Agent 端主动断开重连机制触发，不是崩溃
- 可能原因：Agent 的心跳/重连机制、Agent 端认为连接异常主动重建、网络中间设备干预
- 这是正常的重连行为，不影响功能，但触发了离线/上线通知

**问题:**
- Disconnected 事件发送时用的是 `agent_id`（UUID），前端直接显示
- 上线事件携带完整 agent 信息（含 hostname），所以显示正常
- 断线时 session 可能已丢失 agent 详情，只有 UUID

**修复方向:**
- 方案 A：Disconnected 事件携带 hostname，前端显示 hostname
- 方案 B：前端收到 UUID 的离线事件时，从已有 agent 列表中查找 hostname 显示

**状态:** 待修复

---

### BUG-007: 任务通知缺少上下文信息

**现象:**
- 通知只显示"任务完成 Task task-6"
- 不知道是什么类型的任务（命令执行？文件下载？文件上传？）
- 不知道任务的具体内容（下载了什么文件？执行了什么命令？）

**问题:**
- 通知文案缺少任务类型和关键信息
- 用户无法通过通知判断任务结果

**期望:**
- 通知应显示任务类型和关键信息，如：
  - "文件下载完成: secret.docx"
  - "命令执行完成: whoami"
  - "文件上传完成: config.ini"

**修复方向:**
- Server 端：在任务完成事件中携带任务类型和摘要信息
- 前端：根据任务类型生成有意义的通知文案

**涉及文件（待确认）:**
- `server/src/` — 任务完成事件的 payload
- `client/src/store/notifications.ts` — 通知文案生成逻辑

**状态:** 待修复

---

## ✅ FIXED

_(无)_

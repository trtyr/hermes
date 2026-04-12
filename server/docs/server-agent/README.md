# Server 和 Agent

这一组文档只讨论 `server` 和 `agent client` 之间的控制通道，不讨论浏览器前端页面。

这里的边界很明确：

- `server <-> web_client` 是管理面。
- `server <-> agent` 是控制面。
- 心跳、抖动、任务拉取、命令执行都属于控制面语义。

## 文档列表

### 通道与协议

- `docs/server-agent/agent-protocol.md`
  Agent 与 Server 之间的传输协议、消息格式与帧语义。
- `docs/server-agent/c2-agent-channel-design.md`
  Server 与 Agent 之间 C2 风格通信通道的抽象与约束。
- `docs/server-agent/c2-strengthening-roadmap.md`
  当前控制通道后续还可以继续强化的方向、优先级与微内核拆分建议。

### 生命周期与在线状态

- `docs/server-agent/agent-lifecycle.md`
  Agent 资产、在线会话、上下线与禁用启用等生命周期语义。
- `docs/server-agent/beacon-heartbeat-design.md`
  心跳、抖动、拉取任务时机和在线判定的设计说明。

### 命令执行与会话

- `docs/server-agent/command-session-design.md`
  无 PTY 命令会话、队列、状态流转与执行结果模型。
- `docs/server-agent/web-terminal-cwd-resolution.md`
  Web Terminal 在无 PTY 语义下的 `cwd` 解析与上下文保持设计。

## 不包含什么

- 不讲前端登录页。
- 不讲浏览器如何连接 `/events/ws`。
- 不讲页面级接口如何渲染表格、抽屉和终端页。

这些内容去看：

- `docs/server-web-client/README.md`

## 建议阅读顺序

如果你在改 Agent 侧链路，建议按这个顺序看：

1. `docs/server-agent/agent-protocol.md`
2. `docs/server-agent/c2-agent-channel-design.md`
3. `docs/server-agent/c2-strengthening-roadmap.md`
4. `docs/server-agent/agent-lifecycle.md`
5. `docs/server-agent/beacon-heartbeat-design.md`
6. `docs/server-agent/command-session-design.md`
7. `docs/server-agent/web-terminal-cwd-resolution.md`

## 适合什么时候读

- 你在改 Agent 注册、心跳、任务拉取或下发逻辑。
- 你在排查为什么某个 Agent 在线、离线、禁用、可见但不可控。
- 你在补命令执行链路，需要确认队列、会话和结果回传的边界。

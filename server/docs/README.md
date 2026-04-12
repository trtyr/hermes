# Hermes Server 文档总览

当前文档统一按三条主线组织：

1. `Server 架构`
2. `Server 和 Web Client`
3. `Server 和 Agent`

这样拆分之后，每一组文档都只回答一类问题，避免 API、协议、页面、内核设计混在一起。

## 1. Server 架构

这一组文档只关心 `server` 自身：

- 它的微内核边界是什么。
- 模块应该怎么分层。
- 发布和回归时要检查什么。

入口文档：

- `docs/server-architecture/README.md`
- `docs/server-architecture/server-architecture.md`
- `docs/server-architecture/kernel-runtime.md`
- `docs/server-architecture/kernel-auth.md`
- `docs/server-architecture/kernel-state-storage.md`
- `docs/server-architecture/kernel-agent-domain.md`
- `docs/server-architecture/kernel-listener-domain.md`
- `docs/server-architecture/kernel-task-domain.md`
- `docs/server-architecture/kernel-command-session-domain.md`
- `docs/server-architecture/kernel-agent-build-domain.md`
- `docs/server-architecture/release-process.md`

## 2. Server 和 Web Client

这一组文档只关心前后端对接：

- Web 登录怎么走后端认证。
- 前端页面该调用哪些 HTTP API。
- WebSocket 事件如何同步页面状态。
- Web Terminal 怎么和后端交互。

入口文档：

- `docs/server-web-client/README.md`
- `docs/server-web-client/http-api.md`
- `docs/server-web-client/openapi.yaml`
- `docs/server-web-client/web-login-api.md`
- `docs/server-web-client/dashboard-overview-api.md`
- `docs/server-web-client/agent-management-api.md`
- `docs/server-web-client/agent-page-api.md`
- `docs/server-web-client/agent-build-api.md`
- `docs/server-web-client/websocket-events-api.md`
- `docs/server-web-client/web-terminal-simple-api.md`
- `docs/server-web-client/web-terminal-command-api.md`
- `docs/server-web-client/c2-frontend-expected-api.md`

## 3. Server 和 Agent

这一组文档只关心控制通道：

- Agent 如何接入 listener 并注册。
- 心跳、抖动、在线判定是什么关系。
- 任务如何派发到 Agent。
- 命令会话、结果回传、运行时状态如何维护。

入口文档：

- `docs/server-agent/README.md`
- `docs/server-agent/agent-protocol.md`
- `docs/server-agent/agent-lifecycle.md`
- `docs/server-agent/beacon-heartbeat-design.md`
- `docs/server-agent/c2-agent-channel-design.md`
- `docs/server-agent/c2-strengthening-roadmap.md`
- `docs/server-agent/command-session-design.md`
- `docs/server-agent/web-terminal-cwd-resolution.md`

## 建议阅读顺序

第一次熟悉仓库，建议按这个顺序看：

1. `docs/server-architecture/README.md`
2. `docs/server-architecture/server-architecture.md`
3. `docs/server-web-client/README.md`
4. `docs/server-agent/README.md`

如果你正在写 Web 前端，建议按这个顺序看：

1. `docs/server-web-client/README.md`
2. `docs/server-web-client/web-login-api.md`
3. `docs/server-web-client/dashboard-overview-api.md`
4. `docs/server-web-client/agent-page-api.md`
5. `docs/server-web-client/websocket-events-api.md`
6. `docs/server-web-client/web-terminal-simple-api.md`
7. `docs/server-web-client/web-terminal-command-api.md`

如果你正在处理 Agent 通信、心跳或任务派发，建议按这个顺序看：

1. `docs/server-agent/README.md`
2. `docs/server-agent/agent-protocol.md`
3. `docs/server-agent/c2-agent-channel-design.md`
4. `docs/server-agent/c2-strengthening-roadmap.md`
5. `docs/server-agent/agent-lifecycle.md`
6. `docs/server-agent/beacon-heartbeat-design.md`
7. `docs/server-agent/command-session-design.md`

## 文档归类规则

为了防止后续再次变乱，分类规则固定如下：

- 讲 `server` 内部结构、模块边界、运行层次、发布流程的，放 `Server 架构`。
- 讲前端登录、HTTP API、页面接口、WebSocket、Web Terminal 接入的，放 `Server 和 Web Client`。
- 讲 listener、注册、心跳、任务下发、命令会话、Agent 协议的，放 `Server 和 Agent`。

如果一篇文档跨两个方向，优先放到“主语义”所在目录，并在正文开头链接到另一组文档，不重复维护两份。

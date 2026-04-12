# HTTP API 总览

这份文档是 `server` 面向 `web_client` 的 HTTP API 总入口。

如果你需要机器可读规范，请直接看：

- `docs/server-web-client/openapi.yaml`

如果你需要某个能力的详细说明，请看对应专题文档：

- 登录：`docs/server-web-client/web-login-api.md`
- 总览：`docs/server-web-client/dashboard-overview-api.md`
- Agent 管理：`docs/server-web-client/agent-management-api.md`
- Agent 页面：`docs/server-web-client/agent-page-api.md`
- WebSocket：`docs/server-web-client/websocket-events-api.md`
- 终端：`docs/server-web-client/web-terminal-simple-api.md`
- 终端底层：`docs/server-web-client/web-terminal-command-api.md`

## 1. 基础地址

示例：

```text
http://127.0.0.1:3000
```

运行时文档接口：

- `GET /docs`
- `GET /openapi.yaml`

## 2. 认证模型

当前主认证模型是后端统一登录：

1. 前端调用 `POST /auth/login`
2. 后端返回 `session_token`
3. 前端后续请求带上 `Authorization: Bearer <session_token>`
4. WebSocket 使用 `session_token` 查询参数

兼容方式仍然保留：

- `Authorization: Bearer <api_token>`
- `x-api-token: <api_token>`
- `x-session-token: <session_token>`
- cookie `hermes_session=<session_token>`

可选审计身份头：

- `x-operator: <name>`

## 3. 鉴权规则

- 公共接口：`GET /health`
- 受保护接口：除 `GET /health` 外的其他管理接口

## 4. 接口分组

### 4.1 系统与认证

- `GET /health`
- `POST /auth/login`
- `POST /auth/logout`
- `GET /auth/me`
- `GET /events/ws`

### 4.2 总览

- `GET /dashboard/stats`

### 4.3 Agent 管理

- `GET /agents`
- `GET /agents/history`
- `GET /agents/:agent_id`
- `DELETE /agents/:agent_id`
- `POST /agents/:agent_id/disable`
- `POST /agents/:agent_id/enable`
- `POST /agents/:agent_id/disconnect`
- `POST /agents/:agent_id/beacon-config`
- `POST /agents/:agent_id/tasks`
- `POST /agents/:agent_id/command-sessions`

### 4.4 命令会话

- `GET /command-sessions`
- `GET /command-sessions/:command_session_id`
- `GET /command-sessions/:command_session_id/commands`
- `GET /command-sessions/:command_session_id/commands/:command_id`
- `POST /command-sessions/:command_session_id/commands`
- `POST /command-sessions/:command_session_id/execute`
- `POST /command-sessions/:command_session_id/close`

### 4.5 任务

- `GET /tasks`
- `GET /tasks/:task_id`
- `POST /tasks/broadcast`

### 4.6 Listener

- `GET /listeners`
- `GET /listeners/:listener_id`
- `POST /listeners`
- `PATCH /listeners/:listener_id`
- `POST /listeners/:listener_id/enable`
- `POST /listeners/:listener_id/disable`
- `DELETE /listeners/:listener_id`

### 4.7 Agent 构建

- `GET /agent-builds`
- `GET /agent-builds/:build_id`
- `POST /agent-builds`

### 4.8 Web Terminal 轻量包装

- `POST /web/terminal/open`
- `GET /web/terminal/session/{session_id}`
- `POST /web/terminal/command`
- `POST /web/terminal/close`
- `GET /web/terminal/ws`

## 5. 推荐阅读方式

- 想先看全局：读这篇
- 想做精确联调：看 `openapi.yaml`
- 想写某个页面：直接去读对应专题文档

## 6. 文档边界

这份文档只做总览，不再维护所有字段的超长逐项定义。

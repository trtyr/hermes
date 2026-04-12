# 前端集成契约

这份文档从前端工程的视角，说明接入 `server` 时应固定遵守哪些约束。

## 1. 基础约定

### 1.1 后端地址

前端应允许用户输入或配置后端地址，例如：

```text
http://127.0.0.1:3000
```

### 1.2 登录模型

前端统一走后端登录：

1. 调用 `POST /auth/login`
2. 拿到 `session_token`
3. 后续 HTTP 和 WebSocket 统一复用这个会话

### 1.3 认证头

HTTP 推荐：

```http
Authorization: Bearer <session_token>
```

WebSocket 推荐：

```text
/events/ws?session_token=<session_token>
```

## 2. 页面数据源约定

- 总览页：`GET /dashboard/stats`
- Agent 主表：`GET /agents/history`
- Agent 详情：`GET /agents/{agent_id}`
- 实时刷新：`GET /events/ws`
- 终端页：优先读 `web-terminal-simple-api.md`

## 3. 数据处理约定

### 3.1 不压扁双状态轴

Agent 至少有两条状态轴：

- `is_online`
- `is_disabled`

### 3.2 不把前端当成状态源

在线状态、`last_seen`、任务状态、会话状态都以后端返回和事件流为准。

### 3.3 不把终端当成 PTY

当前终端页是命令会话，不是 SSH。

## 4. 推荐前端接法

1. `GET /health`
2. `POST /auth/login`
3. 保存 `session_token`
4. 首屏拉业务接口
5. 建立 `/events/ws`

## 5. 文档索引

建议阅读顺序：

1. `docs/server-web-client/web-login-api.md`
2. `docs/server-web-client/dashboard-overview-api.md`
3. `docs/server-web-client/agent-page-api.md`
4. `docs/server-web-client/websocket-events-api.md`
5. `docs/server-web-client/web-terminal-simple-api.md`
6. `docs/server-web-client/web-terminal-command-api.md`

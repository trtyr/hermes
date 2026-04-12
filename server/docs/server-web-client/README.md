# Server 和 Web Client

这一组文档只讨论浏览器前端如何连接 `server`。

边界固定如下：

- 前端只通过 HTTP 和 WebSocket 调用 `server`
- 前端不直接参与 Agent 控制通道
- 登录、页面数据、实时事件、Web Terminal 都属于这一组

## 文档列表

### 接入与认证

- `docs/server-web-client/web-login-api.md`
  后端统一登录、会话获取、退出登录、当前会话查询。
- `docs/server-web-client/http-api.md`
  面向人的 HTTP API 总览，适合先看全局。
- `docs/server-web-client/openapi.yaml`
  机器可读接口规范，适合生成类型或做精确字段对齐。

### 页面接口

- `docs/server-web-client/dashboard-overview-api.md`
  总览页接口。
- `docs/server-web-client/agent-management-api.md`
  Agent 管理相关原子接口。
- `docs/server-web-client/agent-page-api.md`
  Agent 页面如何组合调用这些接口。
- `docs/server-web-client/agent-build-api.md`
  Agent 生成、listener 绑定、产物与 manifest 说明。

### 实时事件

- `docs/server-web-client/websocket-events-api.md`
  浏览器与后端之间的 WebSocket 事件流。

### Web Terminal

- `docs/server-web-client/web-terminal-simple-api.md`
  面向前端的轻量终端包装接口。
- `docs/server-web-client/web-terminal-command-api.md`
  更完整的命令会话模型和终端页接入说明。

### 集成约束

- `docs/server-web-client/c2-frontend-expected-api.md`
  前端接入时的统一约束和推荐做法。

## 不包含什么

- 不讲 Agent 注册报文、心跳报文、任务派发帧
- 不讲 listener 驱动和控制通道实现
- 不讲 `server` 内核如何分层

这些内容分别去看：

- `docs/server-agent/README.md`
- `docs/server-architecture/README.md`

## 建议阅读顺序

如果你在写前端，建议按这个顺序看：

1. `docs/server-web-client/web-login-api.md`
2. `docs/server-web-client/dashboard-overview-api.md`
3. `docs/server-web-client/agent-page-api.md`
4. `docs/server-web-client/websocket-events-api.md`
5. `docs/server-web-client/web-terminal-simple-api.md`
6. `docs/server-web-client/web-terminal-command-api.md`

如果你在做字段对齐、联调或类型生成，建议再看：

1. `docs/server-web-client/http-api.md`
2. `docs/server-web-client/openapi.yaml`

## 适合什么时候读

- 你在写登录页、总览页、Agent 管理页、终端页
- 你要确认某个前端功能到底该调哪一个接口
- 你要确认 HTTP 和 WebSocket 是否复用同一套后端认证

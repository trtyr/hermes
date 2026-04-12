# Server 架构

这一组文档只讨论 `server` 自身的微内核结构，不讨论前端页面协议，也不讨论 Agent 传输细节。

这组文档的目标是把架构说明拆成两层：

1. 一份总览文档，解释整体边界、层次和依赖方向。
2. 多份内核能力文档，每份只讲一个能力域，不再把所有东西堆在一篇里。

## 文档列表

### 总览

- `docs/server-architecture/server-architecture.md`
  整体架构总览、分层关系、依赖方向和阅读地图。

### 内核公共机制

- `docs/server-architecture/kernel-runtime.md`
  内核启动、消息总线、运行时分发、watchdog 和副作用端口。
- `docs/server-architecture/kernel-auth.md`
  后端认证内核，包括 Web 登录会话和兼容性 API Token。
- `docs/server-architecture/kernel-state-storage.md`
  内存状态、SQLite 持久化、冷启动装载和状态落盘边界。

### 内核能力域

- `docs/server-architecture/kernel-agent-domain.md`
  Agent 生命周期内核，包括连接、注册、心跳、下线和 Beacon 配置。
- `docs/server-architecture/kernel-listener-domain.md`
  Listener 管理内核，包括配置记录、驱动注册、运行时启停。
- `docs/server-architecture/kernel-task-domain.md`
  任务流转内核，包括单播、广播、取消和结果聚合。
- `docs/server-architecture/kernel-command-session-domain.md`
  无 PTY 命令会话内核，包括打开、排队、执行、关闭和结果回传。
- `docs/server-architecture/kernel-agent-build-domain.md`
  Agent 构建能力，包括构建请求、监听器绑定、产物记录。

### 工程交付

- `docs/server-architecture/release-process.md`
  发布、回归、构建和文档校对时的检查项。

## 不包含什么

- 不讲前端登录页、页面接口和 WebSocket 事件格式。
- 不讲 Agent 协议帧、注册报文和心跳报文细节。

这些内容分别去看：

- `docs/server-web-client/README.md`
- `docs/server-agent/README.md`

## 建议阅读顺序

1. `docs/server-architecture/server-architecture.md`
2. `docs/server-architecture/kernel-runtime.md`
3. `docs/server-architecture/kernel-state-storage.md`
4. 按你正在修改的能力域，继续读对应文档
5. `docs/server-architecture/release-process.md`

## 适合什么时候读

- 你要判断当前代码是不是还守着微内核设计。
- 你要新增一个能力，但不想把代码继续堆到 HTTP handler 或大杂烩 service 里。
- 你要明确某段逻辑应不应该进入内核。
- 你要做发布、回归、交付前校验。

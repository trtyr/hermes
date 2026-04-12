# Server 架构总览

这份文档只做一件事：给出 `server` 的整体架构地图。

具体的内核能力已经拆到独立文档中，不再全部堆在这一篇里。

## 1. 系统边界

当前系统是三段式结构：

- `server`
  控制平面进程，负责管理面 API、控制面协调、状态维护和持久化。
- `web_client`
  浏览器前端，只通过 HTTP 和 WebSocket 调用 `server`。
- `agent client`
  部署在目标主机上的客户端，只通过 listener 接入 `server`。

这里最重要的边界是：

- `server <-> web_client` 是管理面。
- `server <-> agent client` 是控制面。
- 微内核主要存在于 `server` 进程内部。

## 2. 设计目标

`server` 当前的微内核化目标不是“形式上分目录”，而是这四条：

1. 外部适配层不直接改内存状态。
2. 内核能力通过统一入口暴露，不让 API 和 listener 代码直接拼业务逻辑。
3. 状态变更由运行时统一调度，不把状态机散落在各处。
4. 持久化和事件发布作为副作用端口，从运行时中分离出去。

## 3. 分层结构

推荐从上往下理解：

### 3.1 协议层

对应代码：

- `src/protocol.rs`

职责：

- 定义跨层共享的数据结构。
- 定义 agent/server 帧。
- 定义 task、command session、audit、snapshot 等 DTO。

规则：

- 协议层只放数据契约，不放业务逻辑。

### 3.2 外部适配层

对应代码：

- `src/api/*`
- `src/agent/gateway.rs`
- `src/agent/listeners/*`

职责：

- 接收 HTTP、WebSocket、TCP listener 输入。
- 做鉴权、参数解析、传输层校验。
- 把外部输入翻译成内核入口调用。

规则：

- 适配层不能直接修改 `KernelState`。
- 适配层只能通过 `KernelHandle` 或 facade 进入内核。

### 3.3 服务入口层

对应代码：

- `src/kernel/service/*`

职责：

- 给外层提供稳定的内核调用入口。
- 屏蔽底层消息构造、ID 分配和查询细节。
- 按能力域暴露 facade，例如 `auth`、`tasks`、`command_sessions`、`listeners`。

规则：

- 新功能优先加到正确的 facade，不要让 API handler 直接拼运行时消息。

### 3.4 内核运行时

对应代码：

- `src/kernel/runtime/*`
- `src/kernel/message.rs`
- `src/kernel/bus.rs`

职责：

- 接收域消息。
- 路由到具体能力域处理器。
- 驱动状态变更。
- 触发持久化和事件发布。

规则：

- 运行时只关心领域语义，不关心 HTTP/TCP 细节。

### 3.5 状态层与持久化层

对应代码：

- `src/kernel/state/*`
- `src/kernel/storage/*`

职责：

- `state` 维护权威内存状态。
- `storage` 负责 SQLite 装载、查询和落盘。

规则：

- 状态机不要在 API 层和存储层重复实现。
- 存储层不拥有编排权，只做数据持久化和装载。

## 4. 内核主干

当前内核主干可以概括成下面这条链：

`外部输入 -> facade -> KernelMessage -> runtime dispatcher -> state mutation -> effects -> storage / event bus`

这条链是当前微内核设计是否成立的核心判断标准。

如果某段代码绕开这条链，直接从适配层去改状态或拼副作用，通常就意味着结构开始走偏。

## 5. 能力域拆分

当前已经按能力域拆开的内核文档如下：

- `docs/server-architecture/kernel-runtime.md`
- `docs/server-architecture/kernel-auth.md`
- `docs/server-architecture/kernel-state-storage.md`
- `docs/server-architecture/kernel-agent-domain.md`
- `docs/server-architecture/kernel-listener-domain.md`
- `docs/server-architecture/kernel-task-domain.md`
- `docs/server-architecture/kernel-command-session-domain.md`
- `docs/server-architecture/kernel-agent-build-domain.md`

每一篇都只回答一个问题，不再把所有能力揉在一起。

## 6. 依赖方向

推荐依赖方向固定如下：

- `main` 只做组装和启动。
- `api` / `gateway` / `listeners` 依赖 `kernel/service`。
- `kernel/service` 依赖 `kernel/message`、`kernel/state`、`kernel/storage`。
- `kernel/runtime` 依赖 `kernel/state` 和 `runtime effects`。
- `runtime effects` 依赖 `storage` 和 `event bus`。

要避免的反向依赖：

- `state` 依赖 `api`
- `storage` 依赖 `runtime handler`
- `api` 直接依赖 `storage` 做领域决策

## 7. 当前代码里的微内核体现

当前代码已经具备比较明确的“内核形状”：

- 有统一消息总线：`src/kernel/bus.rs`
- 有域消息模型：`src/kernel/message.rs`
- 有统一分发器：`src/kernel/runtime/dispatch.rs`
- 有集中状态：`src/kernel/state/*`
- 有独立副作用端口：`src/kernel/runtime/effects.rs`
- 有 facade 入口层：`src/kernel/service/*`

所以现在更准确的说法是：

- 这不是教科书式微内核。
- 但已经是明显的“微内核化控制平面”。

## 8. 读图方式

如果你准备改某个能力，建议按这个顺序进入：

1. 先看这篇总览，确认它属于哪个能力域。
2. 再看对应的能力域文档，确认入口、运行时、状态和持久化边界。
3. 最后再去读那一组代码。

这样做的目的，是让“改一个功能”先变成“找到它属于哪个内核能力”，而不是一上来就在项目里到处搜。

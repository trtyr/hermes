# 内核运行时

这篇文档描述 `server` 微内核里最核心的公共机制：

- 内核如何启动
- 域消息如何进入总线
- 运行时如何分发
- watchdog 如何定期驱动系统
- 副作用如何从状态机中分离

## 1. 对应代码

- `src/kernel/mod.rs`
- `src/kernel/runtime/mod.rs`
- `src/kernel/runtime/bootstrap.rs`
- `src/kernel/runtime/dispatch.rs`
- `src/kernel/runtime/watchdog.rs`
- `src/kernel/message.rs`
- `src/kernel/bus.rs`
- `src/kernel/runtime/effects.rs`
- `src/kernel/runtime/effects/event_publisher.rs`
- `src/kernel/runtime/effects/state_persistence.rs`

## 2. 运行时的角色

运行时不是 API 层，也不是存储层。

它的职责只有四类：

1. 接收消息
2. 路由消息
3. 驱动状态变更
4. 触发副作用

如果把领域逻辑比作“内核”，那么运行时就是内核的调度器。

## 3. 启动流程

入口在 `src/kernel/runtime/bootstrap.rs` 的 `new_kernel(...)`。

当前启动顺序是：

1. 初始化 `Storage`
2. 从 SQLite 做冷启动装载
3. 创建 `KernelBus`
4. 创建 Web 事件总线
5. 构造 `KernelState`
6. 构造 `KernelHandle`
7. 启动 `dispatch::kernel_loop`
8. 启动 `watchdog::heartbeat_watchdog`

这里的关键点是：

- 冷启动装载先于运行时循环
- `KernelHandle` 是内核对外统一句柄
- 运行时和 watchdog 都是后台任务

## 4. 消息总线

对应代码：

- `src/kernel/bus.rs`
- `src/kernel/message.rs`

总线使用 `tokio::mpsc`。

总线本身不做业务判断，只负责把 `KernelMessage` 送入内核循环。

当前消息按领域分三类：

- `KernelMessage::Agent(...)`
- `KernelMessage::Task(...)`
- `KernelMessage::CommandSession(...)`

这样设计的好处是：

- 外层只需提交“领域意图”
- 运行时统一做状态机处理
- 不同能力域之间的边界清晰

## 5. 分发器

对应代码：

- `src/kernel/runtime/dispatch.rs`

`kernel_loop` 是当前的统一分发器。

它做的事情很简单：

1. 从总线取出下一条消息
2. 按领域类型路由
3. 调用具体领域处理器

当前三条主路由是：

- `route_agent_message`
- `route_task_message`
- `route_command_session_message`

这意味着：

- 领域处理器自己负责状态变更
- 分发器不承担复杂业务逻辑

## 6. watchdog

对应代码：

- `src/kernel/runtime/watchdog.rs`

当前 watchdog 每秒触发一次，向总线发送：

- `AgentKernelMessage::SweepHeartbeats`

注意这里的设计：

- watchdog 不直接改状态
- watchdog 只是发一条“请执行心跳扫描”的领域消息

这非常符合微内核设计，因为定时器机制和领域处理解耦了。

## 7. 副作用端口

对应代码：

- `src/kernel/runtime/effects.rs`

当前运行时副作用被收束为 `RuntimePorts`，内部主要有两类端口：

- `EventPublisher`
  向 Web 侧广播 `WebEvent`
- `StatePersistence`
  把状态变化落到 SQLite

这层的意义非常大：

- 运行时专注状态机
- 事件推送不散落在各个 handler
- 持久化不散落在各个 handler

## 8. 设计规则

关于运行时，建议固定遵守下面几条：

1. 外部输入必须先转成领域消息，再进入运行时
2. 运行时 handler 不处理 HTTP/TCP 细节
3. 运行时 handler 可以改 `KernelState`
4. 运行时 handler 触发副作用时只能走 `RuntimePorts`
5. 新能力优先新增领域消息和 handler，不要在 facade 或 API 层偷做状态变更

## 9. 你改代码时怎么判断有没有越界

如果你发现某段逻辑：

- 在 API handler 里直接改状态
- 在 listener 会话里直接改 SQLite
- 在 watchdog 里直接处理领域规则

那基本就是越界了。

正确方向通常应该是：

- 适配层提交消息
- 运行时处理消息
- 副作用端口做发布和落盘

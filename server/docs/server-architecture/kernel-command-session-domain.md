# 命令会话内核

这篇文档描述 `server` 中负责无 PTY 命令会话的能力域。

这里讨论的是：

- 会话打开
- 命令排队
- 命令执行
- 会话关闭
- 执行结果回传

这里不讨论：

- 前端终端页的 UI
- 全屏交互终端仿真

## 1. 对应代码

- `src/kernel/runtime/command_sessions/mod.rs`
- `src/kernel/runtime/command_sessions/open.rs`
- `src/kernel/runtime/command_sessions/execute.rs`
- `src/kernel/runtime/command_sessions/close.rs`
- `src/kernel/runtime/agent_lifecycle/command_reporting.rs`
- `src/kernel/service/command_sessions/*`
- `src/kernel/state/command_state.rs`
- `src/api/command_sessions/*`
- `src/api/web_terminal/*`

## 2. 这个能力域的定位

当前终端模型不是 PTY，而是 `command session`。

这意味着：

- 后端维护一个轻量会话上下文
- 每条命令仍然是离散执行
- 会话主要用于保持 `cwd` 和队列语义
- 前端虽然可以画成终端，但底层不是字节流 shell

## 3. 当前入口

命令会话通过 `CommandSessionKernelMessage` 进入运行时。

主要入口包括：

- `Open`
- `Execute`
- `Queue`
- `Close`

这说明“终端能力”在内核里已经是独立领域，而不是任务域里的一个特例。

## 4. 当前内部结构

### 4.1 open

负责：

- 检查目标 Agent
- 创建会话记录
- 请求 Agent 打开命令会话

### 4.2 execute / queue

负责：

- 把命令作为执行单元加入会话
- 在空闲时立即派发
- 在忙碌时进入队列

### 4.3 close

负责：

- 关闭会话
- 更新会话状态
- 防止后续继续提交命令

### 4.4 command_reporting

负责：

- 接收 Agent 回传的 opened / started / result / closed 事件
- 把这些运行态结果写回内核状态

## 5. 队列语义

这个能力域最关键的价值不是“像终端”，而是“可控队列”。

当前模型更像：

- 一个有状态的命令通道
- 一个顺序执行队列
- 一个带 `cwd` 的上下文容器

这样做的好处是：

- 可审计
- 可追踪
- 比 PTY 更容易和当前 Agent 模型对齐

## 6. 为什么它是独立能力域

如果把它塞回任务域，通常会失去两个关键特性：

- 会话级上下文
- 队列级顺序控制

所以当前单独做成 command session 域是对的。

## 7. 设计规则

关于命令会话，建议固定遵守下面几条：

1. 会话状态由命令会话域统一维护
2. 命令排队顺序不能交给前端假定
3. `cwd` 变化要由后端和 Agent 共同维护
4. 不要把它重新抽象成真实 PTY
5. Web Terminal 只是这个能力域的一个外部适配层

## 8. 当前评价

从代码结构看，命令会话已经是较完整的独立内核模块。

这也是当前微内核设计里最清晰的一块之一。

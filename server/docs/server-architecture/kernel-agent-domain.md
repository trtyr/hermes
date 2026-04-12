# Agent 生命周期内核

这篇文档描述 `server` 内核里和 Agent 生命周期相关的能力域。

这里讨论的是：

- 连接建立
- 注册
- 心跳
- 上下线
- Beacon 配置更新
- 运行中断开

这里不讨论：

- Agent 协议帧字段细节
- 前端页面如何展示 Agent

## 1. 对应代码

- `src/kernel/runtime/agent_lifecycle/mod.rs`
- `src/kernel/runtime/agent_lifecycle/connection.rs`
- `src/kernel/runtime/agent_lifecycle/registration.rs`
- `src/kernel/runtime/agent_lifecycle/beacon_config.rs`
- `src/kernel/runtime/agent_lifecycle/task_reporting.rs`
- `src/kernel/runtime/agent_lifecycle/command_reporting.rs`
- `src/kernel/service/agent_commands/*`
- `src/kernel/service/agent_queries.rs`
- `src/api/agents/*`

## 2. 这个能力域负责什么

Agent 生命周期内核负责维护一个问题：

“这个 Agent 现在是谁、是否在线、和当前连接是什么关系。”

当前包含的核心动作有：

- 新连接建立
- 连接断开
- 注册身份绑定
- 心跳刷新
- 心跳超时扫描
- Beacon 配置变更
- 任务结果回报
- Command Session 结果回报

## 3. 运行时入口

当前这个能力域由 `AgentKernelMessage` 驱动。

主要入口包括：

- `Connected`
- `Disconnected`
- `Frame`
- `UpdateBeaconConfig`
- `SweepHeartbeats`

这说明生命周期语义不是散在各处，而是统一收进 Agent 领域消息里。

## 4. 模块拆分方式

当前 `agent_lifecycle` 内部已经继续细分：

- `connection.rs`
  负责连接建立、断开、心跳清扫
- `registration.rs`
  负责注册和心跳处理
- `beacon_config.rs`
  负责 Beacon 配置更新
- `task_reporting.rs`
  负责任务结果回报
- `command_reporting.rs`
  负责命令会话结果回报

这是一种比较健康的拆法，因为它按“状态变化类型”继续分治，而不是所有 Agent 逻辑塞进一个 handler。

## 5. 典型流转

### 5.1 连接建立

1. listener 接收到连接
2. 外层适配把连接信息送入 `AgentKernelMessage::Connected`
3. 生命周期内核创建会话上下文
4. 后续注册帧再把逻辑 Agent 身份绑定到会话

### 5.2 注册

1. Agent 发来 `Register`
2. 运行时校验并更新 Agent 身份信息
3. 内存状态记录 agent/session 对应关系
4. 触发上线持久化和事件发布

### 5.3 心跳

1. Agent 发来 `Heartbeat`
2. 运行时刷新 `last_seen`
3. watchdog 定期触发超时扫描
4. 超时后把会话标记为离线

## 6. 为什么它属于内核能力

因为这些语义不能只靠 API 层来判断。

例如：

- 一个 Agent 是否在线，取决于运行态会话和心跳
- 一个 Beacon 配置是否可更新，取决于在线状态
- 一个连接断开后该如何影响任务和命令会话，取决于运行时状态

这些都必须属于内核，而不是页面层判断。

## 7. 设计规则

关于 Agent 生命周期，建议固定遵守下面几条：

1. 上下线判断必须由运行时统一维护
2. 注册和心跳必须走同一条 Agent 领域链路
3. Beacon 配置更新要通过领域消息进入运行时
4. 前端看到的在线状态只能来自后端权威状态
5. “禁用”“断开”“离线”“删除记录”这几个语义必须保持分离

## 8. 当前评价

从代码组织看，这个能力域已经比较像一个独立内核模块。

后续要继续守住的是：

- 不要把 Agent 生命周期判断重新挪回 API 层
- 不要把 listener 会话逻辑写成另一套状态机

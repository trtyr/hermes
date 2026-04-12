# 状态与持久化内核

这篇文档描述 `server` 的两块基础设施：

- 内存中的权威运行时状态
- SQLite 持久化与冷启动装载

这两者不是同一层，但它们共同决定了内核“记住了什么”。

## 1. 对应代码

- `src/kernel/state/mod.rs`
- `src/kernel/state/agent_state.rs`
- `src/kernel/state/task_state.rs`
- `src/kernel/state/command_state.rs`
- `src/kernel/state/types.rs`
- `src/kernel/storage/mod.rs`
- `src/kernel/storage/bootstrap.rs`
- `src/kernel/storage/agents.rs`
- `src/kernel/storage/tasks.rs`
- `src/kernel/storage/listeners.rs`
- `src/kernel/storage/audits.rs`
- `src/kernel/storage/agent_builds.rs`

## 2. 为什么要拆成两层

`state` 和 `storage` 的职责必须分开。

### `state`

它负责当前进程中的权威运行态。

例如：

- 哪些 Agent 当前在线
- 哪个 command session 处于打开状态
- 哪些任务在排队、已派发、已完成

### `storage`

它负责持久化记录和重启恢复。

例如：

- Agent 历史记录
- Task 历史记录
- Audit 日志
- Listener 配置
- Agent build 记录

如果这两层混在一起，结果通常就是：

- 一半逻辑在内存里
- 一半逻辑直接查数据库
- 状态语义越来越难维护

## 3. 内存状态里有什么

从 `src/kernel/state/mod.rs` 可以看出，当前状态至少覆盖：

- Agent 会话状态
- Task 状态树
- Command Session 状态
- 一些运行时索引和映射

这些状态的特点是：

- 需要高频读写
- 需要参与运行时判断
- 不能每次都靠 SQLite 现查

## 4. 持久化层里有什么

当前 `storage` 已经按资源类型拆成子模块：

- `agents.rs`
- `tasks.rs`
- `listeners.rs`
- `audits.rs`
- `agent_builds.rs`
- `bootstrap.rs`

这说明持久化层已经不是单文件杂烩，而是按数据域拆开了。

这是对的。

## 5. 冷启动装载

对应代码：

- `src/kernel/storage/bootstrap.rs`
- `src/kernel/runtime/bootstrap.rs`

当前冷启动流程是：

1. `Storage::new(...)` 初始化 SQLite
2. `storage.bootstrap()` 读取启动所需的持久化数据
3. `KernelState::new()` 创建空状态
4. 把启动时需要恢复的数据装回内存

目前已明确装载的内容包括：

- 任务数据
- 下一批 ID / 序号所需信息

这层设计的关键点是：

- 重启恢复逻辑集中在 bootstrap
- 运行时主循环不用自己再去拼恢复逻辑

## 6. 落盘边界

当前运行时通过 `RuntimePorts` 间接触发落盘。

已明确存在的副作用包括：

- task 更新后持久化
- agent 上线/离线状态持久化

这种模式的优点是：

- 运行时知道“什么时候要落盘”
- 但运行时不直接持有 SQL 细节

## 7. 设计规则

关于状态与持久化，建议固定遵守下面几条：

1. 运行态判断优先基于 `KernelState`
2. 历史记录和冷启动恢复走 `Storage`
3. API 层不要直接把 SQLite 当状态机来用
4. 状态迁移规则集中在 `state`
5. 持久化层只负责装载、查询、写入，不负责领域编排

## 8. 什么时候说明结构开始变坏

如果你看到以下迹象，通常说明边界在变差：

- API handler 直接查库后拼“在线状态”
- runtime handler 内联大量 SQL
- 同一个状态迁移规则同时存在于 `state` 和 `api`

这些都应该收回来。

## 9. 当前评价

从目录结构看，`state` 和 `storage` 这两层已经拆得比较合理。

后续真正要守住的是：

- 让运行时持续把状态判断收束到 `state`
- 让持久化持续只做数据事实，而不是业务编排

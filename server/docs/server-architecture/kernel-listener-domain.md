# Listener 管理内核

这篇文档描述 `server` 中负责 listener 管理的能力域。

这里讨论的是：

- Listener 配置记录
- 运行时启停
- 驱动注册
- 传输适配

这里不讨论：

- Agent 协议字段细节
- 前端页面如何渲染 listener 列表

## 1. 对应代码

- `src/kernel/service/listener_commands.rs`
- `src/kernel/service/listener_queries.rs`
- `src/kernel/storage/listeners.rs`
- `src/agent/listeners/manager/mod.rs`
- `src/agent/listeners/manager/reconcile.rs`
- `src/agent/listeners/registry.rs`
- `src/agent/listeners/session/*`
- `src/agent/listeners/tcp_json/*`
- `src/api/listeners/*`

## 2. Listener 能力域的职责

这个能力域解决的问题是：

“后端有哪些 Agent 入口，它们现在是否应该运行，以及具体由哪个驱动承载。”

它覆盖两层：

- 控制平面的 listener 记录
- 运行态的 listener driver 实例

## 3. 当前结构

### 3.1 控制面记录

这些内容通常落在：

- `kernel/service/listener_*`
- `kernel/storage/listeners.rs`

负责：

- 创建 listener
- 修改 listener
- 启用/禁用 listener
- 查询 listener 列表和详情

### 3.2 运行态管理

这些内容通常落在：

- `src/agent/listeners/manager/*`
- `src/agent/listeners/registry.rs`

负责：

- 把数据库中的 listener 记录和运行中的 driver 对齐
- 选择正确的驱动
- 启动或关闭实际监听实例

### 3.3 协议适配

这些内容通常落在：

- `src/agent/listeners/tcp_json/*`
- `src/agent/listeners/session/*`

负责：

- 接收底层连接
- 解析 Agent 帧
- 把会话事件转成内核消息

## 4. 为什么这也是微内核的一部分

虽然 listener 看起来偏网络层，但它并不是一个纯 TCP 工具。

它承载的是：

- Agent 控制面的入口管理
- 协议驱动可扩展点
- 运行态与配置态之间的对齐

所以它应该作为独立能力域存在，而不是散在 `main.rs` 或单个网关文件里。

## 5. 驱动式设计

当前 listener 子系统已经体现出驱动化方向：

- listener 记录来自 SQLite，而不是写死在配置里
- 运行时启停交给 manager 协调
- 具体传输由 driver 实现
- `registry` 负责 driver 查找和注册

这意味着将来新增传输协议时，理想路径是：

1. 增加新的 listener kind
2. 新增对应 driver
3. 在 `registry` 中注册
4. 尽量不改内核主流程

## 6. 设计规则

关于 listener 管理，建议固定遵守下面几条：

1. listener 配置是控制面数据，不是写死常量
2. listener 运行态由 manager 协调，不由 API 直接管理 socket
3. 协议 driver 只负责接入和翻译，不负责领域状态机
4. 新传输协议应优先作为新 driver 接入，而不是改坏现有 runtime

## 7. 当前评价

从现在的目录拆分看，listener 子系统已经比较接近插件式入口层。

这正是微内核里很重要的一点：

- 内核稳定
- 外部接入驱动可替换

# Beacon 与心跳设计

这份文档定义 Hermes 当前的心跳相关语义。

最关键的一点是：

- `server <-> agent` 采用 `beacon` 语义
- `web_client <-> server` 采用事件推送语义
- 这两者有关联，但不是同一套时序模型

## 1. 术语

- `beacon`
  Agent 周期性向后端报到
- `sleep_interval`
  Agent 报告的基础 beacon 间隔，单位秒
- `jitter`
  基础间隔上的抖动百分比
- `last_seen`
  后端最后一次确认该 Agent 活动的时间
- `watchdog`
  后端定时扫描超时会话的机制
- `online`
  后端判断该会话仍在线
- `offline`
  后端判断该会话已失效或断开

## 2. 管理面和控制面分离

系统里有两条平面：

- `frontend <-> backend`
- `backend <-> agent`

控制面是 beacon 驱动的，管理面是事件驱动的。

## 3. 当前运行时语义

当前控制通道特征是：

- 长连接 TCP
- 行分隔 JSON
- Agent 主动发送 beacon
- 后端通过 watchdog 判定超时
- Agent 以 `sleep_interval + jitter` 作为实际 beacon 调度窗口
- 任务结果、命令结果、配置确认先进入 Agent 本地待发队列
- Agent 在 beacon 窗口统一批量发送待发消息
- Agent 收到下行命令并处理后，也会顺带 flush 当前待发消息

## 4. 正常生命周期

1. Agent 连接 listener
2. Agent 完成注册
3. 后端接受该会话并记录身份
4. Agent 周期性发送 beacon
5. 后端刷新 `last_seen`
6. watchdog 在允许时间窗口内维持该会话为在线

## 5. `sleep_interval` 与 `jitter`

### 5.1 `sleep_interval`

表示 Agent 理论上的基础 beacon 周期。

### 5.2 `jitter`

表示该周期允许的浮动范围。

当前实现里，`jitter` 的单位是百分比。

例如：

- `sleep_interval = 10`
- `jitter = 20`

表示下一次 beacon 可能落在 `10s ~ 12s` 之间。

这里的重点不是“每秒轮询一次要不要发”，而是：

- Agent 每次发送 beacon 后，直接计算下一次发送窗口
- watchdog 也按这个最大预期窗口加宽容时间判断超时

这样 `server` 和 `agent` 对时序的理解是一致的。

## 6. 在线判定

在线判定由后端统一负责。

最终以后端 watchdog 和 `last_seen` 判定为准。

watchdog 的超时窗口不是固定常数，而是：

- `sleep_interval`
- `jitter`
- `heartbeat_grace`

共同决定。

## 7. 与任务和命令的关系

beacon 语义会影响：

- 任务派发时机
- Agent 是否仍可接收命令
- 命令会话是否应继续保持活跃

还会影响：

- 任务结果回传时机
- 命令执行结果回传时机
- beacon 配置更新确认时机

当前实现采用“心跳驱动发送”语义：

1. 后台执行线程先把 `task_result`、`command_session_result`、`config_updated` 放进本地待发队列
2. 到了下一次 beacon 窗口时，Agent 先发送 `heartbeat`
3. 然后把当前待发队列里的消息一起发给 `server`

同时，为了避免命令交互显得过于迟钝，Agent 在收到下行命令并处理后，也会顺带 flush 一次当前待发队列。

这个设计保持了两点平衡：

- 主节拍仍然是 beacon，而不是额外的高频泵
- 命令和配置确认不至于完全等到下一轮长周期 beacon 才可见

## 8. Beacon 配置更新

`server` 可以向 Agent 下发：

- `update_beacon_config`

包含：

- `request_id`
- `sleep_interval`
- `jitter`

Agent 收到后会做两件事：

1. 更新本地下一次 beacon 的调度参数
2. 回传 `config_updated`

`server` 在收到 `config_updated` 后，才会把新的 `sleep_interval/jitter` 持久化为当前在线状态。

## 9. 明确不讨论什么

这份文档不讨论：

- 隐蔽调度
- 反分析技巧
- 攻击性心跳策略

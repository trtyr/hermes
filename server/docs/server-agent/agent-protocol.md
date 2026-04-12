# Agent 协议

这份文档定义 `agent client` 和 `server` 之间的控制通道协议。

当前实现特征：

- Agent 连接 TCP listener
- 一行一个 JSON 帧
- Agent 和 Server 都通过 JSON 帧通信

默认接入地址示例：

```text
0.0.0.0:1234
```

## 1. 协议定位

这条通道是控制通道，而不是前端管理通道。

主要语义包括：

- `listener`
- `register`
- `beacon`
- `tasking`
- `result`

## 2. 传输模型

当前已实现传输配置为：

- 长连接 TCP
- 行分隔 JSON
- transport profile：`tcp_json_v1`

## 3. 首帧规则

当前首个核心帧仍然是 `register`。

但在 `challenge_response` 模式下，Server 可以先发一个 `hello`。

## 4. Agent 认证模式

### 4.1 未配置 token

- 不进行 Agent 注册认证

### 4.2 `plain_token`

- `register` 中直接携带共享 token

### 4.3 `challenge_response`

- Server 先发送 `hello`
- Agent 使用共享 token 对 `session_nonce + ":" + agent_id` 做 HMAC-SHA256
- Agent 在 `register` 中提交签名结果

推荐生产模式：

- 配置 `auth.agent_token`
- 配置 `auth.agent_auth_mode = "challenge_response"`

## 5. `hello` 帧

在 `challenge_response` 模式下，Server 可发送类似：

```json
{
  "type": "hello",
  "protocol_version": 2,
  "session_nonce": "16b5bc4f7b9de6b89b17d3a17f7fcbf6",
  "listener_id": 1,
  "listener_name": "default-agent-tcp",
  "transport": "tcp_json_v1",
  "capabilities": [
    "register",
    "beacon",
    "task_dispatch",
    "beacon_config",
    "command_session_queue"
  ],
  "auth_mode": "challenge_response"
}
```

## 6. `register` 帧

典型字段包括：

- `agent_id`
- `hostname`
- `username`
- `protocol_version`
- `os`
- `arch`
- `pid`
- `internal_ip`
- `tags`
- `sleep_interval`
- `jitter`
- `token` 或签名字段

这里的 `sleep_interval` 和 `jitter` 不是展示字段，而是运行时语义字段：

- `server` 会据此计算 watchdog 超时窗口
- Agent 后续也会按这一组参数调度 beacon

## 7. 运行期常见帧

### 7.1 Agent -> Server

- `register`
- `heartbeat` / `beacon`
- `task_result`
- `task_update`
- `config_updated`
- `command_session_opened`
- `command_session_started`
- `command_session_output_chunk`
- `command_session_result`
- `command_session_closed`

### 7.2 Server -> Agent

- `ack`
- `disconnect`
- `dispatch_task`
- `cancel_task`
- `update_beacon_config`
- `open_command_session`
- `execute_command_session`
- `close_command_session`

## 8. 当前发送语义

当前协议虽然跑在长连接 TCP 上，但 Hermes 没有把它做成持续字节流推送模型。

当前 Agent 的上行发送规则是：

1. `register` 在建连后立即发送
2. `heartbeat` 在 beacon 窗口发送
3. `task_result`、`task_update`、`command_session_output_chunk`、`command_session_result`、`config_updated` 先进入本地待发队列
4. 到 beacon 窗口时，Agent 会把这些待发帧和 `heartbeat` 一起发送
5. 当 Agent 收到下行命令并完成一次本地处理后，也会顺带 flush 当前待发队列
6. 当 Agent 内部存在活跃命令执行 worker，或命令结果刚刚生成但尚未刷出时，控制循环会进入短轮询，以便更快发出命令输出

所以从协议视角看：

- 控制通道仍然是 beacon 主节拍
- 但不是“除了 beacon 什么都不发”
- 也不是“高频实时推送字节流”

这是一个偏 C2 语义、但为了 Web 管理体验稍微向可交互性靠拢的折中设计

## 9. 任务派发与取消

### 9.1 任务派发

当前普通任务不是“创建后立刻推送”，而是：

1. `server` 先创建任务并持久化为 `pending`
2. 任务进入 `server` 侧待投递队列
3. Agent 注册成功或发送后续 `heartbeat` 时，`server` 才把该 Agent 的待投递任务下发出去
4. 下发成功后任务状态进入 `dispatched`

这样控制链更符合 beacon 驱动语义。

### 9.2 任务取消

当前任务取消分成两类：

- `pending`
  任务还没投递给 Agent，`server` 可以直接本地取消
- `dispatched` / `running`
  `server` 会先标记为 `cancel_requested`，再向 Agent 下发 `cancel_task`

Agent 侧收到 `cancel_task` 后会尝试中断本地进程。

如果中断成功：

- Agent 回传 `task_update(cancelled)`
- `server` 再把任务落成最终 `cancelled`

如果任务已经自己结束：

- Agent 仍然会回传真实 `task_result`
- `server` 会接受真实终态，而不是强行覆盖成取消

## 10. `command_session_output_chunk`

命令会话当前已经支持分块输出。

语义如下：

1. Agent 收到 `execute_command_session`
2. Agent 先回 `command_session_started`
3. 命令执行结束后，Agent 会把 `stdout` 和 `stderr` 按块拆成多个 `command_session_output_chunk`
4. 全部分块发完后，再发最终 `command_session_result`

注意：

- 这不是 PTY 原始字节流
- 也不是逐字符流
- 仍然是结构化结果，只是把最终输出拆成多个块，方便前端滚动展示

## 11. `update_beacon_config` / `config_updated`

这组帧是当前协议里比较关键的一组闭环：

1. `server -> agent` 发送 `update_beacon_config`
2. Agent 更新本地 beacon 调度参数
3. `agent -> server` 回传 `config_updated`
4. `server` 在收到确认后再更新在线状态快照

这样做的意义是：

- 后端看到的 `sleep_interval/jitter` 是 Agent 已确认生效的值
- 不会出现“接口调用成功，但 Agent 实际没切过去”的幻觉状态

## 12. 设计边界

这份协议强调控制通道的清晰性，而不是隐蔽性。

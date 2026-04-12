# C2 强化路线图

这份文档只讨论 `server <-> agent` 控制通道还可以继续强化的地方。

目标不是做攻击性功能，而是把当前控制链进一步收敛成更稳定、更清晰、也更符合微内核边界的 C2 模型。

## 1. 当前已经具备的基础

当前实现已经有这些核心能力：

- Agent 周期性 beacon
- `sleep_interval + jitter` 驱动的心跳窗口
- 后端 watchdog 超时判定
- 基于 listener 的接入与会话归属
- 基于 token / challenge-response 的 Agent 认证
- 普通任务下发与结果回传
- 无 PTY 命令会话
- 命令会话串行队列
- beacon 配置更新闭环确认

所以当前问题不再是“有没有控制链”，而是“控制链是不是足够稳、足够像一个成熟 C2”。

## 2. 强化目标

建议把下一阶段强化目标收敛成四个方向：

- 把任务派发从“即时推送”收敛成“服务端待投递 + beacon 领取”
- 把取消语义从“状态标记”收敛成“真实中断 + 明确确认”
- 把命令执行从“阻塞控制循环”收敛成“控制面和执行面解耦”
- 把认证从“全局共享”收敛成“按 listener 隔离”

这四件事做完，控制面会明显更稳。

## 3. 优先级 P0

### 3.1 服务端待投递任务队列

当前任务语义更接近即时 RPC：

- Agent 在线时，Server 立即发送
- Agent 不在线时，任务直接失败

这不够像 beacon 驱动的 C2。

建议改成：

1. Server 创建任务时先持久化为 `queued`
2. 任务进入目标 Agent 的待投递队列
3. Agent 在 beacon 窗口领取任务
4. Server 对已领取任务发放短期 lease
5. Agent 未回报结果且 lease 超时后，Server 可决定重投或标记失败

这样会带来三个直接收益：

- Agent 短时掉线后，任务不会立刻丢
- 控制面语义和 beacon 节拍一致
- Web 前端能看到更清晰的任务状态流转

建议新增的内核边界：

- `kernel task queue domain`
- `agent pending delivery state`
- `task lease / ack policy`

### 3.2 真实任务取消

当前 `cancel_task` 只在 Server 状态里把任务记为取消，同时告诉 Agent 尝试取消。

但 Agent 本地现在并没有真正中断运行中的线程或子进程。

这会造成一个问题：

- 页面上任务已经显示 `cancelled`
- 实际命令可能还在主机上继续跑

建议改成两阶段：

1. `server -> agent` 发送 `cancel_task`
2. Agent 尝试中断本地执行单元
3. Agent 回传 `task_update(cancelled)` 或 `task_result(interrupted)`
4. Server 在收到确认后再落最终状态

如果 Agent 无法中断，也应该明确回传：

- `cancel_rejected`
- `cancel_timeout`
- `already_finished`

这样状态才不虚。

建议新增的内核边界：

- `task cancellation controller`
- `task execution handle registry`
- `task cancellation confirmation`

## 4. 优先级 P1

### 4.1 命令执行与控制循环解耦

当前 Agent 收到命令会话执行请求后，会在控制循环里直接执行 shell。

这会带来两个问题：

- 长命令期间，新的控制指令响应会变慢
- 心跳和配置控制会受到影响

建议改成：

- 控制循环只负责收发协议帧
- 命令执行由独立 worker 负责
- worker 执行完成后把结果放入待发队列
- 控制循环继续按 beacon / 下行命令时机 flush

这件事不一定要上 PTY。

即使仍然保持“无 PTY 命令会话”，也应该把控制面和执行面分开。

建议新增的内核边界：

- `command execution worker`
- `command result outbox`
- `control loop scheduler`

### 4.2 会话输出分块

当前任务输出和命令输出都是整块字符串。

这种模型的问题是：

- 大输出内存占用高
- 前端终端不能平滑滚动
- 网络抖动时重传代价大

建议协议逐步补成：

- `command_session_output_chunk`
- `stream_id`
- `chunk_seq`
- `stdout/stderr`
- `eof`

最终再用一个完成帧总结退出码和最终状态。

这样前端终端和普通命令面板都更容易对接。

## 5. 优先级 P2

### 5.1 按 listener 隔离认证材料

当前 listener 管理和 Agent 构建链虽然已经是“按 listener 生成 Agent”，但运行时认证材料仍偏全局。

建议继续收紧：

- 每个 listener 独立 token / key
- Agent 构建时绑定目标 listener 的认证材料
- Server 按 listener 校验注册
- 某个 listener 泄露后只影响该 listener

这样 listener 才是真正的隔离边界，而不是展示字段。

### 5.2 会话级幂等与重放控制

当前 challenge-response 主要保护的是注册阶段。

如果要继续提高协议严谨性，建议加：

- 会话级递增序号
- request_id 幂等去重
- 重放窗口
- 已处理 command / task result 的幂等表

这样能减少：

- 断线重连后的重复处理
- 网络毛刺导致的重复结果入库
- 重放旧帧带来的状态错乱

### 5.3 多传输 profile

当前控制链主实现仍以 `tcp_json_v1` 为主。

如果要继续增强稳定性，可以在协议抽象不变的前提下补：

- `https_json`
- 长轮询 profile
- 多 listener fallback
- Agent 的 profile 优先级列表

这里的重点不是“做复杂”，而是：

- 控制协议语义保持一致
- 传输实现可以替换

这更符合微内核的插件化边界。

## 6. 推荐状态机

### 6.1 任务状态机

推荐从当前状态进一步细化为：

- `queued`
- `leased`
- `dispatched`
- `running`
- `succeeded`
- `failed`
- `cancel_requested`
- `cancelled`
- `dropped`

其中要特别区分：

- `cancel_requested`
  Server 已请求取消，但 Agent 还未确认
- `cancelled`
  Agent 已确认取消成功
- `dropped`
  由于断线、会话关闭或 lease 过期导致结果不可信

### 6.2 命令会话状态机

命令会话建议保持：

- `opening`
- `open`
- `closing`
- `closed`

命令执行建议保持：

- `queued`
- `dispatched`
- `running`
- `succeeded`
- `failed`
- `cancelled`
- `dropped`

这样前端就能清晰区分：

- 命令是不是还在队列里
- 是不是已经下发给 Agent
- 是不是 Agent 已开始执行
- 是正常结束还是异常失效

## 7. 微内核拆分建议

如果按微内核继续拆，建议把强化后的职责边界固定成下面这些域：

- `listener runtime domain`
  负责 listener 生命周期、运行状态与 profile 装配
- `agent session domain`
  负责接入、注册、在线状态、beacon 超时
- `agent auth domain`
  负责 listener 级认证、challenge-response、后续幂等校验
- `task queue domain`
  负责任务待投递、lease、重投、取消确认
- `command session domain`
  负责命令会话、串行队列、结果聚合
- `agent delivery domain`
  负责下行投递策略与上行待发队列 flush 策略
- `transport profile domain`
  负责 `tcp_json`、`https_json` 等传输实现

这样每个域都只回答一类问题，不会把“任务状态机”“连接接入”“认证校验”“传输实现”混在同一个模块里。

## 8. 建议实施顺序

建议按下面顺序推进：

1. 服务端待投递任务队列
2. 真实任务取消
3. 命令执行与控制循环解耦
4. listener 级认证材料隔离
5. 输出分块
6. 幂等与重放控制
7. 多传输 profile

原因很简单：

- 前三项直接影响稳定性和真实控制能力
- 第四项直接影响边界隔离
- 后三项属于继续增强协议成熟度

## 9. 当前最值得先做什么

如果只做一轮增强，我建议优先做这四件事：

1. `server` 侧任务待投递队列
2. Agent 侧真实任务取消
3. Agent 控制循环与命令执行 worker 解耦
4. listener 级独立认证材料

这四件事完成后，Hermes 的 `server <-> agent` 会从“能用的控制链”提升到“语义稳定、边界清晰的控制链”。

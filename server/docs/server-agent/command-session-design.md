# 命令会话设计

这份文档描述 Hermes 在无 PTY 前提下，如何提供接近终端的操作体验。

核心思想不是“做一个真终端”，而是“做一个可控、可审计、可持续保持上下文的命令会话”。

## 1. 目标

- 保持当前 Agent 模型对企业办公环境友好
- 避免 PTY、长驻 shell 和交互式终端进程
- 在多条命令之间保留操作上下文，尤其是 `cwd`
- 保持后端可审计、可控、可回放

## 2. 非目标

- 不做 PTY
- 不支持 `vim`、`top`、`less`、curses 等全屏程序
- 不提供长期 stdin/stdout 字节流

## 3. 核心模型

引入 `command_session`。

它不是 shell 进程，而是一个轻量状态容器。

每个会话至少维护：

- `session_id`
- `agent_id`
- `cwd`
- `created_by`
- `created_at`
- `last_active_at`
- `status`

## 4. 为什么这样设计

如果每条命令都独立执行，那么：

- `cd /var/log` 不会影响下一条命令
- 远程操作体验非常割裂

真正缺少的不是 PTY，而是“跨请求保留上下文”的能力。

## 5. 基本执行流程

### 5.1 创建会话

操作员选择一个 Agent，创建命令会话。

### 5.2 提交命令

操作员提交一行命令。

内建命令如 `cd`、`pwd` 会直接影响会话状态；
普通命令则以一次短生命周期进程执行，但复用当前 `cwd`。

### 5.3 返回结果

执行结果回传给后端，并更新：

- `stdout`
- `stderr`
- `exit_code`
- `success`
- `cwd_before`
- `cwd_after`

当前实现里，这个“回传”不是直接把 stdout/stderr 作为持续字节流推送，而是：

- Agent 先产出结构化结果帧
- 若输出较长，先拆成多个 `command_session_output_chunk`
- 结果帧进入 Agent 本地待发队列
- 在当前 beacon 窗口或命令处理后的 flush 时机批量发给后端

因此命令会话仍然是“请求/结果”模型，不是 PTY 字节流模型。

### 5.4 关闭会话

会话关闭后，不再接受新命令。

## 6. 队列语义

命令会话不是并发命令池，而是受控顺序队列：

- 同一会话内命令按顺序进入队列
- 当前命令未结束时，下一条命令进入等待
- 结果按实际执行顺序回传

当前后端保证：

- 同一 `command_session` 同时只会有一个活动命令
- Agent 回传 `command_session_started` 后，该命令进入运行态
- Agent 回传 `command_session_result` 后，后端再派发下一条排队命令

## 7. Agent 执行模型

当前 Agent 侧命令会话已经不是在主控制循环里同步执行。

现在的语义是：

- 主控制循环负责收协议帧、发协议帧
- `execute_command_session` 到来后，只负责创建 worker
- 具体 shell 执行由独立 worker 完成
- worker 执行结束后，把输出块和最终结果放入待发队列

这样做的作用是：

- 长命令不会堵住主控制循环
- 命令执行期间，Agent 仍能继续处理其他控制指令
- beacon 配置更新、任务取消、断开控制不会被命令执行硬卡住

## 8. 输出分块

为了让 Web Terminal 有更平滑的输出体验，当前实现已经支持分块输出。

具体模型：

1. worker 执行完成后拿到 `stdout/stderr`
2. Agent 先按固定块大小拆成多个 `command_session_output_chunk`
3. `server` 一边聚合这些块，一边把块事件广播给 WebSocket
4. 最终再通过 `command_session_result` 给出完整结果和退出码

这带来两个效果：

- HTTP 同步接口依然能得到完整结果
- WebSocket 终端页可以更自然地按块追加输出

## 9. 前后端职责边界

### 前端

- 渲染终端界面
- 发送命令
- 展示结果

### 后端

- 维护会话状态
- 维护命令队列
- 保存审计和历史
- 聚合命令输出块
- 向 WebSocket 事件流广播输出块和最终状态

### Agent

- 实际执行命令
- 回传命令结果和 `cwd` 变化
- 维护轻量会话态，例如当前 `cwd`
- 在 beacon 节拍内批量发送待发结果
- 在存在活跃命令 worker 时，短时加快待发结果 flush

## 10. 为什么不做 PTY

因为在当前场景里，PTY 带来的成本和复杂度明显高于收益。

## 11. 当前实现补充

为了让前端终端页先稳定工作，当前 Agent 侧已经显式处理了两类内建行为：

- `pwd`
- `cd`

这两类命令会直接更新或读取会话级 `cwd`，再用结构化结果回传。

普通命令则仍然是：

- 在当前 `cwd` 下启动一次短生命周期进程
- 收集 stdout/stderr
- 先按块发送 `command_session_output_chunk`
- 再生成一次结构化 `command_session_result`

这保证了：

- `cd /tmp`
- `pwd`
- 再执行普通命令

这三步在无 PTY 的情况下仍然能形成稳定的“伪终端”体验。

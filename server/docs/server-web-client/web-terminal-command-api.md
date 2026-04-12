# Web Terminal 命令会话接口

这份文档定义终端页背后的命令会话模型。

范围是：

- 浏览器前端如何和后端对接终端页
- 命令会话如何打开、排队、执行、关闭
- 结果如何通过 HTTP 和 WebSocket 同步

## 1. 核心模型

当前终端页不是 PTY，而是 `command_session` 模型。

含义如下：

- 前端可以渲染成终端样式
- 后端维护会话上下文，例如 `cwd`
- 每次提交仍然是一条离散命令
- WebSocket 只负责状态同步，不是原始 stdin/stdout 字节流

## 2. 前端输入

终端页至少需要：

1. `server_url`
2. `session_token`
3. `agent_id`

可选：

4. `operator`
5. `tab_name`

## 3. 认证

HTTP 推荐：

```http
Authorization: Bearer <session_token>
```

WebSocket 推荐：

```text
ws://127.0.0.1:3000/events/ws?session_token=<session_token>
```

## 4. 关键接口

### 4.1 打开命令会话

- `POST /agents/{agent_id}/command-sessions`

### 4.2 查询命令会话

- `GET /command-sessions`
- `GET /command-sessions/{command_session_id}`

### 4.3 查询命令记录

- `GET /command-sessions/{command_session_id}/commands`
- `GET /command-sessions/{command_session_id}/commands/{command_id}`

### 4.4 提交命令

- `POST /command-sessions/{command_session_id}/commands`
- `POST /command-sessions/{command_session_id}/execute`

### 4.5 关闭会话

- `POST /command-sessions/{command_session_id}/close`

## 5. 会话状态

一个命令会话通常会经历：

- `open`
- `closed`
- `expired`

一个命令执行项通常会经历：

- 已创建
- 已排队
- 已开始
- 已完成 / 已失败 / 已取消

## 6. 队列语义

当前推荐前端按下面方式理解：

- 一个会话内的命令是顺序队列
- 前一条未结束时，后一条命令可能进入排队
- 页面不应假定每次输入都会立刻执行完成

## 7. 输出同步

- HTTP 负责命令入口
- WebSocket 负责结果推送

当前 Web Terminal 不再只有最终整块结果。

现在有两层输出：

- `command_output_chunk`
  适合终端页实时追加输出
- `command_updated` / `command_session_result`
  适合状态更新和最终汇总

前端建议优先用 `command_output_chunk` 追加渲染，再用最终结果校准退出码和最终状态。

## 8. WebSocket 终端事件

终端页如果连接的是专用终端事件流，当前最重要的三类事件是：

### 8.1 会话事件

```json
{
  "type": "terminal",
  "event": "session",
  "session_id": "cmdsess-1",
  "state": "open",
  "cwd": "/tmp"
}
```

### 8.2 命令状态事件

```json
{
  "type": "terminal",
  "event": "command",
  "session_id": "cmdsess-1",
  "command_id": "cmd-1",
  "state": "running",
  "cwd": "/tmp",
  "exit_code": null,
  "stdout": "partial or aggregated output",
  "stderr": null
}
```

### 8.3 输出分块事件

```json
{
  "type": "terminal",
  "event": "output",
  "session_id": "cmdsess-1",
  "command_id": "cmd-1",
  "stream": "stdout",
  "sequence": 0,
  "chunk": "hello world"
}
```

含义如下：

- `stream`
  当前块属于 `stdout` 还是 `stderr`
- `sequence`
  同一条命令内的块序号
- `chunk`
  当前输出内容

前端应按 `command_id + stream + sequence` 追加，而不是把整个终端内容重绘一遍。

## 9. 前端实现建议

1. 打开页面时创建或恢复会话
2. 页面输入一行命令后调用命令提交接口
3. 通过 WebSocket 订阅 `session`、`command`、`output` 三类事件
4. 收到 `output` 时实时追加终端文本
5. 收到 `command` 或最终结果时更新状态、退出码和 `cwd`
5. 离开页面时主动关闭会话

## 10. 设计边界

如果你只想要简单可接的接口，请优先看：

- `docs/server-web-client/web-terminal-simple-api.md`

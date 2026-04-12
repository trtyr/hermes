# WebSocket 事件接口

这份文档定义浏览器前端订阅后端实时事件时使用的 WebSocket 接口。

接口：

- `GET /events/ws`

## 1. 作用

这个事件流主要解决三件事：

1. 首次连接后下发一份初始快照
2. 持续推送 Agent、Task、Command Session 的增量变化
3. 让前端页面不必频繁轮询

## 2. 连接地址

示例：

```text
ws://127.0.0.1:3000/events/ws
```

如果前端运行在 HTTPS 下，应使用：

```text
wss://<your-host>/events/ws
```

## 3. 认证方式

推荐方式：

- Header：`Authorization: Bearer <session_token>`
- 浏览器查询参数：`session_token=<session_token>`

兼容方式：

- `Authorization: Bearer <api_token>`
- `x-api-token: <api_token>`
- 查询参数：`api_token=<api_token>`

浏览器前端通常直接使用查询参数形式。

## 4. 连接生命周期

当前后端行为固定如下：

1. 握手成功
2. 先发送一条 `snapshot`
3. 之后持续发送增量事件
4. 客户端普通上行消息当前不参与业务
5. 连接关闭或异常时结束

## 5. 初始快照

第一条消息总是 `snapshot`。

它的作用是：

- 帮前端拿到当前初始状态
- 避免页面进入后必须先手动刷新

## 6. 增量事件

后续事件主要覆盖：

- Agent 在线 / 离线 / 更新
- 任务派发 / 更新 / 取消
- Command Session 打开 / 开始 / 输出块 / 完成 / 关闭

其中和终端页直接相关的事件包括：

- `command_session_opened`
- `command_session_updated`
- `command_session_closed`
- `command_updated`
- `command_output_chunk`
- `command_session_result`

## 7. 前端使用建议

### Agent 页面

- 用 `snapshot` 初始化在线态
- 用增量事件更新 `last_seen`、在线状态和任务状态

### 终端页

- 用增量事件同步命令会话状态和输出结果

如果前端走的是专用终端事件流，后端还会把这些通用事件再简化成：

- `event=session`
- `event=command`
- `event=output`

其中 `event=output` 对应命令输出块，适合直接追加到终端缓冲区。

### 总览页

- 可以用事件触发局部刷新

## 8. 输出分块建议

当前命令输出已经支持分块事件。

前端处理建议：

1. 以 `command_id` 作为一条命令的主键
2. 收到 `command_output_chunk` 时按 `sequence` 追加内容
3. 收到 `command_session_result` 时把该命令标记为最终完成
4. 不要假设只有最终结果帧才会携带输出

## 9. 断线重连建议

前端建议实现：

1. 指数退避或固定间隔重连
2. 重连成功后重新消费新的 `snapshot`
3. 会话过期时跳回登录页

## 10. 文档边界

这份文档只定义浏览器和后端之间的事件流。

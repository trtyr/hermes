# Web Terminal 轻量接口

这份文档定义给前端直接使用的轻量终端接口。

设计目的：

- 给前端一个稳定、薄包装的终端接口层
- 隐藏命令队列和 command session 的内部细节
- 不破坏后端现有微内核结构

## 1. 接口列表

- `POST /web/terminal/open`
- `GET /web/terminal/session/{session_id}`
- `POST /web/terminal/command`
- `POST /web/terminal/close`
- `GET /web/terminal/ws`

## 2. 认证

- HTTP：`Authorization: Bearer <session_token>`
- WebSocket：查询参数 `session_token`
- 兼容：`api_token`

## 3. 打开终端

接口：

- `POST /web/terminal/open`

请求体示例：

```json
{
  "agent_id": "agent-001"
}
```

响应示例：

```json
{
  "success": true,
  "message": "ok",
  "data": {
    "session_id": "cmdsess-1",
    "cwd": "/Users/alice",
    "status": "open"
  }
}
```

## 4. 查询会话

- `GET /web/terminal/session/{session_id}`

用途：

- 页面刷新后恢复当前终端上下文
- 判断会话是否仍处于 `open`

## 5. 提交命令

- `POST /web/terminal/command`

推荐请求体字段：

- `session_id`
- `line`

## 6. 关闭会话

- `POST /web/terminal/close`

## 7. 终端事件流

- `GET /web/terminal/ws`

用途：

- 同步命令执行结果
- 同步 `cwd` 变化
- 同步会话关闭等状态

## 8. 适用场景

如果前端只想要一个“能工作、接口简单、足够稳定”的终端接法，优先用这份文档。

如果需要理解底层 command session 语义，再去看：

- `docs/server-web-client/web-terminal-command-api.md`

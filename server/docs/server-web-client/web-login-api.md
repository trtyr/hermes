# Web 登录接口

这份文档定义浏览器前端连接 `server` 时使用的统一登录流程。

当前推荐模型已经固定：

- 前端输入后端地址、账号、密码
- 前端把账号密码发给后端
- 后端创建会话
- 前端后续所有 HTTP 和 WebSocket 都复用这个后端会话

## 1. 前端登录表单

登录页建议收集三个输入项：

1. `server_url`
2. `username`
3. `password`

推荐含义：

- `server_url`
  `server` 的 HTTP 地址，例如 `http://127.0.0.1:3000`
- `username`
  后端配置文件中的 Web 登录账号
- `password`
  后端配置文件中的 Web 登录密码

## 2. 后端配置来源

后端登录账号配置在 `config.toml`：

```toml
[auth]
web_username = "admin"
web_password = "123456"
session_ttl_secs = 28800
api_token = "dev-api-token"
agent_token = ""
```

说明：

- `web_username` / `web_password` 是主登录入口
- `session_ttl_secs` 是会话有效期，单位秒
- `api_token` 仅保留给兼容调用或脚本使用

## 3. 登录流程

### 3.1 可达性探测

请求：

```http
GET /health
```

作用：

- 检查用户输入的后端地址是否可达
- 检查该地址是否是 Hermes Server

### 3.2 提交账号密码

请求：

```http
POST /auth/login
Content-Type: application/json
```

请求体：

```json
{
  "username": "admin",
  "password": "123456"
}
```

成功响应示例：

```json
{
  "success": true,
  "message": "ok",
  "data": {
    "username": "admin",
    "session_token": "<session_token>",
    "expires_at": 1760000000000
  }
}
```

### 3.3 复用会话

登录成功后，前端后续 HTTP 请求应带：

```http
Authorization: Bearer <session_token>
```

浏览器 WebSocket 建议带：

```text
ws://127.0.0.1:3000/events/ws?session_token=<session_token>
```

## 4. 会话相关接口

### 4.1 查询当前会话

- `GET /auth/me`

作用：

- 页面刷新后恢复登录态
- 检查当前 token 是否仍有效

### 4.2 退出登录

- `POST /auth/logout`

作用：

- 主动删除当前后端会话
- 退出后需要重新登录

## 5. 推荐前端行为

1. 先用 `GET /health` 探测地址
2. 再用 `POST /auth/login` 登录
3. 登录成功后保存 `session_token`
4. 进入主界面后再拉业务接口
5. 会话失效时跳回登录页

## 6. 设计边界

这份文档只讨论前端和后端之间的登录认证。

它不讨论：

- Agent 注册认证
- listener 接入认证
- 前端本地伪登录

# Agent 生成 API

这篇文档只讲一件事：

前端如何请求 `server` 生成一个绑定到指定 listener 的 Agent 二进制。

## 1. 设计目标

当前语义固定如下：

- Agent 的服务端地址在构建期写入 `agent/src/server.rs`，再参与编译
- 生成产物默认绑定某个 listener
- Agent 运行时不依赖环境变量改地址或改身份
- 不做额外加密、混淆或壳处理
- 生成结果必须可追踪、可审计、可复查

这意味着：

- `server` 负责生成
- `listener` 负责提供接入地址
- 产物自己带着构建时写入的地址运行

## 2. 接口列表

### 2.1 `POST /agent-builds`

发起一次新的 Agent 构建。

请求体：

```json
{
  "target_triple": "aarch64-apple-darwin",
  "listener_id": 2,
  "profile": "release"
}
```

字段说明：

- `target_triple`
  Rust 目标平台三元组；不传时默认使用 `server` 当前主机平台。
- `listener_id`
  绑定的 listener；推荐前端总是传这个字段。
- `server_addr`
  可选；如果传了，必须和 `listener_id` 对应 listener 的 `bind_host:bind_port` 完全一致，否则后端拒绝构建。
- `agent_token`
  可选；如果传了，会写入 `agent/src/server.rs` 作为默认 token。
- `profile`
  构建模式，默认 `release`。

后端处理规则：

1. 如果传了 `listener_id`，后端先读取 listener 记录。
2. 如果没有传 `server_addr`，后端自动使用该 listener 的绑定地址。
3. 如果同时传了 `listener_id` 和 `server_addr`，两者必须一致。
4. 构建前，后端会先覆盖 `agent/src/server.rs`，再编译 Agent。
5. 生成产物后，后端会写入构建记录和 manifest。

成功响应示例：

```json
{
  "success": true,
  "detail": "agent build created",
  "build": {
    "build_id": 7,
    "target_triple": "aarch64-apple-darwin",
    "profile": "release",
    "listener_id": 2,
    "server_addr": "127.0.0.1:2234",
    "embedded_agent_token": false,
    "status": "succeeded",
    "artifact_path": "/abs/path/server/data/agent-builds/build-7/agent-aarch64-apple-darwin-agent",
    "artifact_name": "agent-aarch64-apple-darwin-agent",
    "detail": "built aarch64-apple-darwin with embedded server_addr=127.0.0.1:2234 binding=compile_time_only manifest=/abs/path/server/data/agent-builds/build-7/agent-aarch64-apple-darwin-agent.manifest.json",
    "created_at": 1743139200,
    "updated_at": 1743139208
  }
}
```

### 2.2 `GET /agent-builds`

查询构建历史。

常见用途：

- 前端展示构建记录列表
- 下载前先确认最新构建是否成功
- 做审计追踪

### 2.3 `GET /agent-builds/:build_id`

查询单条构建记录。

常见用途：

- 查看产物路径
- 查看失败原因
- 读取 manifest 路径

### 2.4 `POST /listeners/:listener_id/agent-builds`

这是给前端更简化的入口。

如果前端页面已经先选定了 listener，就直接调这个接口，不需要再自己重复传 `listener_id` 或 `server_addr`。

请求体：

```json
{
  "target_triple": "aarch64-apple-darwin",
  "profile": "release",
  "agent_token": "test-token"
}
```

字段说明：

- `target_triple`
  可选；不传则使用 `server` 当前主机平台。
- `profile`
  可选；默认 `release`。
- `agent_token`
  可选；作为构建期默认 token 写入 Agent。

这个接口等价于：

```json
POST /agent-builds
{
  "listener_id": 2,
  "profile": "release"
}
```

只是把“绑定哪个 listener”收敛进 URL 了，前端更省事，语义也更清楚。

## 3. 绑定语义

这个接口的关键不是“下载一个通用 Agent”，而是：

“生成一个已经绑定 listener 的 Agent 产物”。

绑定关系如下：

- `listener_id` 决定接入哪个 listener
- listener 的 `bind_host:bind_port` 决定构建时写入的地址
- 运行该 Agent 时，只使用构建时写入的地址

也就是说，前端不需要再给最终用户解释复杂的运行时覆盖逻辑。

## 4. 运行时行为

由 `POST /agent-builds` 生成的 Agent，当前约束如下：

- 构建时写入地址是唯一生效的服务端地址
- 构建时写入 token 是默认认证值
- 心跳、抖动、重连间隔、命令超时使用 Agent 内置默认值和后端下发配置
- Agent 不支持通过环境变量覆盖这些字段

## 5. Manifest 说明

每次成功构建后，产物旁边都会生成一个 `.manifest.json` 文件。

当前重点字段：

- `listener_id`
  这次构建绑定了哪个 listener。
- `listener_name`
  绑定 listener 的名字。
- `listener_bind`
  listener 的绑定地址。
- `embedded_server_addr`
  实际由 `agent/src/server.rs` 提供并写入产物的地址。
- `server_addr_binding`
  当前固定为 `compile_time_only`。
- `ignored_runtime_overrides`
  明确列出历史遗留的变量名；当前 Agent 会忽略它们。
- `runtime_overrides`
  当前固定为空数组。

这样前端、运维和测试都可以直接读 manifest 判断产物行为，不需要猜。

## 6. 前端推荐做法

前端建议把“生成 Agent”设计成三步：

1. 先查询 listener 列表，让用户选择目标 listener。
2. 调用 `POST /listeners/:listener_id/agent-builds`，只传目标平台、构建模式、可选 token。
3. 构建成功后展示产物路径、manifest 路径、绑定 listener 信息。

不建议前端默认暴露 `server_addr` 自由输入框。

因为当前系统已经把“listener 绑定”和“编译期嵌入地址”收敛成同一件事，继续开放自由地址只会制造歧义。

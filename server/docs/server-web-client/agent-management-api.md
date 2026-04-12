# Agent 管理接口

这份文档定义 Web 管理端使用的 Agent 管理原子接口。

范围包括：

- 在线会话列表
- 历史资产列表
- 单个 Agent 详情
- Beacon 配置更新
- 向单个 Agent 下发任务
- 断开在线会话
- 禁用 / 启用 Agent
- 删除离线 Agent 记录

## 1. 认证

所有接口都要求后端认证。

推荐方式：

```http
Authorization: Bearer <session_token>
```

兼容方式：

```http
Authorization: Bearer <api_token>
```

或者：

```http
x-api-token: <api_token>
```

可选审计头：

```http
x-operator: <operator_name>
```

## 2. 核心数据模型

### 2.1 在线快照 `AgentSnapshot`

用于 `GET /agents`。

典型字段：

- `session_id`
- `agent_id`
- `listener_id`
- `listener_name`
- `hostname`
- `username`
- `os`
- `arch`
- `pid`
- `internal_ip`
- `external_ip`
- `tags`
- `sleep_interval`
- `jitter`
- `peer_addr`
- `connected_at`
- `last_seen`

### 2.2 历史记录 `AgentRecord`

用于 `GET /agents/history` 和 `GET /agents/{agent_id}`。

典型字段包括：

- `agent_id`
- `listener_id`
- `listener_name`
- `hostname`
- `username`
- `os`
- `arch`
- `pid`
- `internal_ip`
- `external_ip`
- `sleep_interval`
- `jitter`
- `is_online`
- `is_disabled`
- `last_seen`
- `connected_at`
- `updated_at`

## 3. 查询接口

### 3.1 获取在线会话

- `GET /agents`

### 3.2 获取历史资产列表

- `GET /agents/history`

常用查询参数：

- `limit`
- `offset`
- `keyword`
- `online`
- `disabled`
- `tag`

### 3.3 获取单个 Agent 详情

- `GET /agents/{agent_id}`

## 4. 管理操作接口

### 4.1 更新 Beacon 配置

- `POST /agents/{agent_id}/beacon-config`

### 4.2 向单个 Agent 下发任务

- `POST /agents/{agent_id}/tasks`

### 4.3 打开命令会话

- `POST /agents/{agent_id}/command-sessions`

### 4.4 断开在线 Agent

- `POST /agents/{agent_id}/disconnect`

### 4.5 禁用 Agent

- `POST /agents/{agent_id}/disable`

### 4.6 启用 Agent

- `POST /agents/{agent_id}/enable`

### 4.7 删除 Agent 记录

- `DELETE /agents/{agent_id}`

约束：

- 仅应对离线 Agent 使用

## 5. 前端状态规则

这几个状态不要混成一个字段：

- 连通性：`is_online`
- 管理状态：`is_disabled`

## 6. 推荐使用方式

- 主表：`GET /agents/history`
- 在线轻量视图：`GET /agents`
- 详情抽屉：`GET /agents/{agent_id}`
- 管理操作完成后刷新当前项，或结合 WebSocket 增量更新

## 7. 设计边界

这份文档只定义“原子接口”。

页面级组合方式请看：

- `docs/server-web-client/agent-page-api.md`

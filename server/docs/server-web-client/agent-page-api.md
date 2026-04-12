# Agent 页面接入指南

这份文档从“页面落地”的角度说明 Agent 管理页该怎么接后端。

它不是原子接口清单，而是页面组合指南。

相关文档：

- `docs/server-web-client/agent-management-api.md`
- `docs/server-web-client/http-api.md`
- `docs/server-web-client/websocket-events-api.md`

## 1. 页面拆分建议

推荐把 Agent 页面拆成四块：

1. 列表页
2. 详情抽屉或详情页
3. 操作区
4. 可选实时刷新层

## 2. 列表页

主接口：

- `GET /agents/history`

常用查询参数：

- `limit`
- `offset`
- `keyword`
- `online`
- `disabled`
- `tag`

推荐表格字段：

- `agent_id`
- `listener_name`
- `hostname`
- `username`
- `os`
- `arch`
- `internal_ip`
- `external_ip`
- `sleep_interval`
- `jitter`
- `is_online`
- `is_disabled`
- `last_seen`

## 3. 状态展示建议

不要把所有状态压成一个字符串。

后端当前提供的是两条正交状态轴：

- 连通状态：`is_online`
- 管理状态：`is_disabled`

推荐前端组合展示：

- 在线 + 已启用
- 在线 + 已禁用
- 离线 + 已启用
- 离线 + 已禁用

## 4. 详情抽屉

推荐流程：

1. 主表使用 `GET /agents/history` 渲染
2. 点击某个 Agent 后，再调用 `GET /agents/{agent_id}`
3. 用单 Agent 详情结果覆盖列表中的缓存数据

## 5. 操作区

常用操作及对应接口：

- 更新 Beacon：`POST /agents/{agent_id}/beacon-config`
- 下发任务：`POST /agents/{agent_id}/tasks`
- 打开命令会话：`POST /agents/{agent_id}/command-sessions`
- 断开连接：`POST /agents/{agent_id}/disconnect`
- 禁用节点：`POST /agents/{agent_id}/disable`
- 启用节点：`POST /agents/{agent_id}/enable`
- 删除记录：`DELETE /agents/{agent_id}`

## 6. 前端禁用逻辑建议

- Agent 不在线时，禁用“断开连接”和“更新 Beacon”
- Agent 已禁用时，不允许再次禁用
- Agent 已禁用时，不建议继续下发任务
- Agent 在线时，不建议允许“删除记录”

## 7. 实时刷新

如果前端接入实时能力，推荐订阅：

- `GET /events/ws`

用途：

- 刷新在线状态
- 刷新 `last_seen`
- 刷新任务和命令会话变化

## 8. 推荐接法

1. 首屏拉 `GET /agents/history`
2. 点开详情时拉 `GET /agents/{agent_id}`
3. 操作成功后刷新当前项
4. 如果接了 WebSocket，用事件做增量更新

## 9. 文档边界

如果你要精确字段和原子接口语义，请看：

- `docs/server-web-client/agent-management-api.md`

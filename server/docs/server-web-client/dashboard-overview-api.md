# 总览页面接口

这份文档定义总览页使用的专用后端接口。

范围固定为三块：

1. `server` 所在主机的运维信息
2. Agent 管理汇总信息
3. Listener 管理汇总信息

## 1. 接口

- `GET /dashboard/stats`

## 2. 认证

该接口为受保护接口，推荐使用：

```http
Authorization: Bearer <session_token>
```

兼容方式：

```http
x-session-token: <session_token>
```

## 3. 返回结构

响应按三个顶层块分组：

- `server`
  服务器主机运行信息
- `agents`
  Agent 数量和状态汇总
- `listeners`
  Listener 数量和运行状态汇总

另外还有：

- `generated_at`
  后端生成这份总览数据的时间戳

## 4. 返回字段说明

### 4.1 `server`

主要用于总览页的主机信息卡片，字段通常包括：

- `hostname`
- `os_name`
- `os_version`
- `kernel_version`
- `architecture`
- `uptime_seconds`
- `cpu_cores`
- `load_average`
- `memory`

### 4.2 `agents`

主要用于总览页的 Agent 汇总卡片，字段通常包括：

- `total`
- `online`
- `offline`
- `disabled`
- `connected_sessions`

### 4.3 `listeners`

主要用于总览页的 Listener 汇总卡片，字段通常包括：

- `total`
- `enabled`
- `disabled`
- `running`
- `stopped`
- `starting`
- `error`
- `by_kind`

## 5. 请求示例

```bash
curl \
  -H 'Authorization: Bearer <session_token>' \
  http://127.0.0.1:3000/dashboard/stats
```

## 6. 响应示例

```json
{
  "generated_at": 1760000000000,
  "server": {
    "hostname": "dev-macbook",
    "os_name": "macOS",
    "os_version": "15.3",
    "kernel_version": "24.3.0",
    "architecture": "aarch64",
    "uptime_seconds": 86400,
    "cpu_cores": 8
  },
  "agents": {
    "total": 12,
    "online": 4,
    "offline": 8,
    "disabled": 1,
    "connected_sessions": 4
  },
  "listeners": {
    "total": 3,
    "enabled": 2,
    "disabled": 1,
    "running": 2,
    "stopped": 1
  }
}
```

## 7. 前端使用建议

- 这份接口适合作为总览页首屏数据源
- 不建议用它替代 Agent 详情页或 Listener 详情页接口
- 如果页面需要实时刷新，可结合 `docs/server-web-client/websocket-events-api.md`

## 8. 设计边界

这份接口强调“总览”，不是“详情列表”。

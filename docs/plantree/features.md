# Feature Requests

## ✅ 已完成

### FEAT-004: 全局 Task 结果后台接收机制

**问题:** 截图、目录浏览等异步操作的结果接收逻辑绑定在页面组件上。用户切换页面时组件卸载 → 事件监听清理 → 操作结果丢失。

**改动:**
- `events.ts` store 新增全局 `taskResults` Map — 所有 `task_result` 事件自动缓存（上限 200 条）
- 新增 `registerPendingTask(taskId, type)` / `getTaskResult(taskId)` / `isTaskCompleted(taskId)` / `clearPendingTask(taskId)` API
- `SessionScreenshotTab.vue`：发起截图时注册 pending task，组件 mount 时检查缓存结果
- 截图超时保护：`catch_unwind` + 30 秒 `recv_timeout` + GDI null 检查

**状态:** ✅ 已完成

---

### FEAT-001: 审计日志清空功能

**改动:**
- Server `DELETE /audits` API（storage + kernel handle + API handler + route）
- 前端 `clearAudits()` + popconfirm 按钮

**状态:** ✅ 已完成

---

### FEAT-002: Token 改为 per-listener 绑定

**改动:**
- **后端:** session 认证优先读取 `listener.config["agent_token"]`，未配置时回退全局
- **前端 A2:** 移除全局"Agent 认证"卡片，改为每行 inline lock/unlock popover
- **前端 A1:** 创建监听器表单新增 token + 认证模式字段
- **前端 A3:** payload 页选择 listener 时自动填入 token（listener 的或全局的）

**状态:** ✅ 已完成

---

### FEAT-003: 文件浏览器目录缓存 + 刷新机制

**改动:**
- `SessionFilesTab.vue` 新增 `browseCache` ref 缓存已访问目录
- `doBrowse` 增加 `forceRefresh` 参数，优先读缓存
- 模板路径栏新增刷新按钮

**状态:** ✅ 已完成

---

## 🔧 附带修复

### 端口冲突校验

**改动:** Server `storage/listeners.rs` 创建/更新监听器时检查 `bind_host:bind_port` 是否已被占用，冲突时报错 `端口冲突: 0.0.0.0:1234 已被监听器 'A' 占用`

---

### apiFetch 错误处理

**改动:** Client `api/request.ts` `apiFetch` 非 2xx 响应时抛出服务端 `detail` 信息，而非静默返回 JSON

---

### bind 错误信息改进

**改动:** Server HTTP API 和 TCP/HTTPS listener bind 失败时附带地址信息，如 `HTTP API 无法绑定到 0.0.0.0:3000: Address already in use`

---

### 截图画廊

**改动:**
- Server `GET /tasks` 新增 `command` 过滤参数
- 前端 `listTasks()` API 函数 + `TaskRecord` / `TaskListResponse` 类型
- `SessionScreenshotTab.vue` 重写：网格画廊 + lightbox 弹窗 + 历史截图持久化

---

### 截图阻塞修复（写入线程解耦）

**改动:**
- Agent `network.rs`: TCP 写入分离到独立线程，`send()` 推入 `mpsc::channel` 不阻塞主循环
- Agent `sys_ops.rs`: 截图缩放至 1280px + PNG `Fastest` 压缩
- Server `console.rs`: 超时日志 ms→s 标签修正

---

## 📋 待实现

（暂无）

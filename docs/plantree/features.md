# Feature Requests

## 待实现

### FEAT-001: 审计日志清空功能

**页面:** `/log` (http://localhost:5173/log)

**现状:** 审计日志只有筛选和分页，没有清空功能。

**需求:**
- 后端：新增 `DELETE /api/audit` 或 `POST /api/audit/clear` 端点
- 前端：在 log 页面加"清空日志"按钮，调用该端点

**优先级:** 中

---

### FEAT-002: Token 改为 per-listener 绑定（架构重构）

**背景:** 当前 token 是全局配置，所有监听器共享。应改为每个监听器独立绑定 token。

#### A1: 数据模型 — Token 从全局改为 per-listener

**现状:** `config.toml` 中 `[auth]` 节的 `agent_token` 和 `agent_auth_mode` 是全局的

**改动:**
- Listener 表新增 `agent_token` 和 `agent_auth_mode` 字段
- 创建监听器时必须指定 token 和认证模式
- 移除全局 token 配置（或降级为默认值/兼容字段）

**涉及文件（预估）:**
- `server/src/kernel/storage/` — listener 表 schema、migration
- `server/src/protocol.rs` — ListenerRecord 结构体
- `server/src/api/listeners/` — 创建/更新 listener 的 request DTO
- `server/src/agent/listeners/` — listener 启动时使用自己的 token

#### A2: UI — 移除全局 "Agent 认证" 卡片

**现状:** 监听器页面顶部有"Agent 认证"卡片，管理全局 token

**改动:**
- 移除监听器页面顶部的"Agent 认证"卡片
- 创建监听器表单中增加 token 和认证模式字段
- 监听器表格中显示每行的 token（脱敏显示）

**涉及文件（预估）:**
- `client/src/views/listener/index.vue` — 移除认证卡片、修改表格列
- `client/src/views/listener/components/CreateListenerModal.vue` — 增加 token 字段
- `client/src/api/listener.ts` — 更新请求参数

#### A3: UI — 构建载荷时自动填充 token

**现状:** 构建页面选择监听器后，agent token 需手动填写

**改动:**
- 选择监听器后，自动填入该监听器绑定的 token（只读/禁用状态）
- 用户可选择不填（不嵌入 token）或手动覆盖

**涉及文件（预估）:**
- `client/src/views/payload/index.vue` — 监听器选择联动 token 字段

**优先级:** 高（核心架构问题）

**状态:** 进行中

**进展 (2026-05-30):**
- 后端已实现 agent 注册认证优先读取 listener `config.agent_token` / `config.agent_auth_mode`
- 若 listener 未配置 token，则保持回退到全局 `[auth]` 配置，兼容现有行为
- 尚未完成 A1 的数据模型收敛（独立字段 / migration）以及 A2/A3 的前端改造

---

### FEAT-003: 文件浏览器目录缓存 + 刷新机制

**页面:** `/agent/:id/session` (终端会话中的文件浏览器)

**现状:**
- 每次进入目录都要等一个心跳周期返回结果
- 从 A → B → A，即使 A 刚访问过，还是要等心跳重新请求
- 没有手动刷新按钮

**需求:**

#### 缓存机制
- 已访问过的目录内容应缓存在前端
- 从 B 返回 A 时，直接显示缓存内容，无需等待心跳
- 缓存 key 为 agent_id + 目录路径

#### 刷新按钮
- 文件浏览器中增加"刷新"按钮
- 点击后强制从 Agent 重新获取当前目录内容，更新缓存
- 快捷键支持（如 F5）

**涉及文件（待确认）:**
- `client/src/views/agent/` — 文件浏览器组件
- `client/src/composables/` — 可能需要新的 composable 管理缓存状态

**优先级:** 中（体验优化）

**状态:** 待实现

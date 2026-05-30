# Bug Tracker

## 🔴 OPEN

### BUG-008: 截图任务导致 Agent 心跳超时断连

**严重性:** 高 — 截图功能完全不可用，且导致 agent 离线

**现象:**
- 截图任务 dispatch 后 agent 既不返回 `task_result`，也不发送心跳
- 约 70 秒后 server 端 heartbeat timeout → agent 被强制清理离线
- `browse` 等其他任务正常完成，仅 `screenshot` 稳定复现

**日志证据:**
```
06:41:23  task-18 (screenshot) created
06:41:34  task-18 dispatched → agent 执行截图...
          （agent 无 task_result 返回，无心跳）
06:42:44  heartbeat timeout → agent 离线
```

**可能根因:**
- `sys_ops::handle_screenshot` 在独立线程执行（`std::thread::spawn`）
- Windows GDI 截屏调用可能阻塞/panic（`SetProcessDPIAware`、`BitBlt`、`GetDIBits`）
- 如果截图线程 panic，`sender.send(TaskResult)` 不会执行 → 无结果回报
- 但 panic 不应影响主线程心跳 — 除非截图操作导致整个进程卡死

**排查方向:**
1. 截图线程是否 panic（加 catch_unwind 或日志）
2. `capture_screen_to_png()` 是否在无桌面/无显示器环境下阻塞
3. base64 编码大图片是否导致 OOM
4. 截图操作是否持有锁导致主线程阻塞

**复现条件:** 稳定复现 — 每次截图必触发

**状态:** 🔴 待排查

---

## ✅ FIXED

### BUG-009: 心跳超时时间过长（实际 55 秒，非预期 15 秒）

**严重性:** 中 — Agent 断连后需 55 秒才能被 server 检测到

**问题:** Agent 心跳间隔 15 秒，但 server 端超时公式为 `sleep_interval × 3 + jitter_extra + grace`：
- `15 × 3 = 45s`（3 倍间隔）
- `+ 10s`（HEARTBEAT_GRACE_MS）
- `= 55 秒` 总超时

即 agent 连续丢失 ~3.67 次心跳才被判定离线。期间 server 不知道 agent 已死。

**关键常量（`connection.rs`）:**
```rust
const UNREGISTERED_SESSION_TIMEOUT_MS: u64 = 10_000;  // 未注册 10s
const HEARTBEAT_GRACE_MS: u64 = 10_000;                // 宽限 10s
const MIN_HEARTBEAT_TIMEOUT_MS: u64 = 5_000;           // 最小 5s
```

**超时公式（`agent_state.rs:186-203`）:**
```
timeout = sleep_interval × 1000 × 3 + sleep_interval × 1000 × jitter/100 + heartbeat_grace_ms
```

**Sweep 频率:** watchdog 每 1 秒触发一次 `SweepHeartbeats`，检测本身不是瓶颈。

**结论:** 设计如此 — 3 倍间隔 + grace 是合理的容错窗口，避免因网络抖动误判 agent 离线。

### BUG-001: 载荷构建完成通知显示 undefined

**现象:** 构建完成后弹窗显示 `Build #undefined — undefined`

**根因:** Server 发送嵌套 `{ build: { build_id, status } }`，客户端直接取扁平字段导致 undefined

**修复:** 前端正确访问 `event.build.status` 和 `event.build.build_id`（已确认生效）

**状态:** ✅ 已修复

---

### BUG-002: 认证失败的 Agent 触发虚假"离线"通知（通知风暴）

**现象:** Agent token 不匹配时，前端疯狂弹出"Agent 离线 — unknown"通知

**修复:**
- Server `session/mod.rs`：只在 `registered = true` 时发送 Disconnected 事件
- Server `connection.rs`：`cleanup_session_disconnect` 只在 `agent_id` 为 Some 时发布 AgentDisconnected

**状态:** ✅ 已修复

---

### BUG-003: 权限列显示多余的 `Admin:` 前缀

**现象:** 权限列显示 `Admin: SeIncreaseQuotaPrivilege, ...`

**修复:** `agent/index.vue` 和 `agent/session.vue` 的 tooltip 通过 `.replace(/^Admin:\s*/, '')` 去除前缀

**状态:** ✅ 已修复

---

### BUG-004: 命令执行失败且错误信息不友好

**现象:** Windows 上 `ls`、`dir` 等命令执行后显示 `exec failed`，退出码 1

**根因:** Windows 的 `Command::new(cmd)` 直接执行程序，shell 内置命令不是独立可执行文件

**修复:** Agent `ops.rs` Windows 使用 `cmd.exe /C <command>` 包装

**状态:** ✅ 已修复

---

### BUG-005: 文件下载任务完成但浏览器无下载响应

**现象:** 任务完成但浏览器没有下载

**根因:** Agent 端文件以 base64 通过 `task_result` 返回，但客户端从未处理 base64 触发浏览器下载

**修复:** `events.ts` 新增 `pendingDownloads` map，WebSocket handler 解码 base64 并通过 Blob URL 触发下载

**状态:** ✅ 已修复

---

### BUG-006: 离线通知显示 UUID 而非 hostname

**现象:** Agent 离线通知显示 `e2785793-b4d7-...` 而非 hostname

**修复:** `events.ts` 新增 `agentDisplayNames` map，从 snapshot/registered/updated 事件收集 hostname，通知使用 `getAgentDisplayName()` 解析

**状态:** ✅ 已修复

---

### BUG-007: 任务通知缺少上下文信息

**现象:** 通知只显示"任务完成 Task task-6"，不知道什么类型

**修复:**
- Server `protocol.rs`：`WebEvent::TaskResult` 新增 `command` 字段
- Server `task_reporting.rs`：从 `TaskSnapshot` 提取 command
- 前端 `notifications.ts`：根据 command 显示具体标签（文件下载完成、命令执行: whoami 等）

**状态:** ✅ 已修复

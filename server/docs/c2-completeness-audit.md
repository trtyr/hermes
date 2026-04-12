# Hermes C2 Server — 功能完成度审计

> 审计日期：2026-04-11  
> 审计对象：hermes/server（C2 控制端）  
> 审计目的：对照传统 C2 框架功能清单，评估本科毕设工作量和完成度

---

## 一、项目概况

| 指标 | 数值 |
|---|---|
| Rust 源码 | ~12,265 行 |
| E2E 测试 | ~2,141 行（14 个测试套件 + 公共模块） |
| 架构文档 | ~4,130 行（含 3 个子目录） |
| 架构风格 | Microkernel（消息总线 + Facade 分层） |
| 技术栈 | Rust 2024 + Tokio + Axum 0.8 + SQLite |
| 三段式结构 | server（控制端）/ client（Web 前端）/ agent（Windows 木马） |

---

## 二、传统 C2 框架功能对照表

以 Cobalt Strike、Sliver、Havoc、Mythic 等 C2 为参考基线，按功能域逐项对比。

### 2.1 Agent 管理（Agent Management）

| # | 功能 | 状态 | 实现说明 |
|---|---|---|---|
| 1 | Agent 注册 | ✅ 完成 | TCP JSON 注册，携带 hostname/username/os/arch/pid/IP/tags |
| 2 | 心跳保活 | ✅ 完成 | Watchdog 每秒扫描，超时标记离线 |
| 3 | Beacon 配置（sleep/jitter） | ✅ 完成 | 运行时动态修改 sleep_interval 和 jitter |
| 4 | Agent 状态管理（在线/离线） | ✅ 完成 | 内存状态 + SQLite 持久化 |
| 5 | Agent 禁用/启用 | ✅ 完成 | 禁用后拒绝注册和任务接收 |
| 6 | Agent 断开/删除 | ✅ 完成 | 主动断开连接，删除已离线 agent |
| 7 | Agent 历史查询 | ✅ 完成 | 含已离线、已删除 agent 的完整历史 |
| 8 | 重复 ID 接管（session 接管） | ✅ 完成 | 同 agent_id 重连自动替换旧 session |
| 9 | Agent 分组/标签 | ⚠️ 部分 | 有 tags 字段，但无按标签过滤/分组的 API |
| 10 | Agent 备注/备注名 | ❌ 未实现 | 无对 agent 添加自定义备注的功能 |

### 2.2 任务/命令系统（Tasking）

| # | 功能 | 状态 | 实现说明 |
|---|---|---|---|
| 11 | 单目标任务下发 | ✅ 完成 | POST /agents/{id}/tasks |
| 12 | 广播任务（多目标） | ✅ 完成 | POST /tasks/broadcast，父-子任务链 |
| 13 | 任务取消 | ✅ 完成 | 向 agent 发送 CancelTask |
| 14 | 任务状态机 | ✅ 完成 | Pending → Dispatched → Running → Succeeded/Failed |
| 15 | 任务结果回收 | ✅ 完成 | TaskResult + TaskUpdate，WebSocket 实时推送 |
| 16 | 任务查询（分页） | ✅ 完成 | 列表 + 详情 API |
| 17 | 交互式命令会话 | ✅ 完成 | 完整的 open/execute/close 生命周期 |
| 18 | 命令会话 cwd 保持 | ✅ 完成 | 支持 cd 命令，cwd 状态服务端跟踪 |
| 19 | 流式输出 | ✅ 完成 | CommandOutputChunk → WebSocket 实时推送 |
| 20 | 命令队列（先入队后执行） | ✅ 完成 | queue + execute 两步操作 |
| 21 | 命令会话超时控制 | ✅ 完成 | open 5s / execute 20s / close 5s |

### 2.3 远程执行命令类型

| # | 功能 | 状态 | 实现说明 |
|---|---|---|---|
| 22 | sysinfo（系统信息） | ✅ 完成 | 协议定义 + E2E 测试覆盖 |
| 23 | whoami | ✅ 完成 | 协议定义 |
| 24 | exec（命令执行） | ✅ 完成 | 带 payload 的命令执行 |
| 25 | shell（交互式 shell） | ✅ 完成 | 通过 command session 实现 |
| 26 | 文件上传（Server→Agent） | ❌ 未实现 | 协议无文件传输相关定义 |
| 27 | 文件下载（Agent→Server） | ❌ 未实现 | 同上 |
| 28 | 截图 | ❌ 未实现 | 协议中无定义 |
| 29 | 键盘记录 | ❌ 未实现 | 协议中无定义 |
| 30 | 进程列表 | ❌ 未实现 | 协议中无定义 |
| 31 | 进程注入 | ❌ 未实现 | 协议中无定义 |
| 32 | 屏幕截图 | ❌ 未实现 | 协议中无定义 |
| 33 | 凭证获取（dump hash/票据） | ❌ 未实现 | — |
| 34 | 权限提升 | ❌ 未实现 | — |
| 35 | 持久化植入（注册表/计划任务等） | ❌ 未实现 | — |

### 2.4 Listener 系统

| # | 功能 | 状态 | 实现说明 |
|---|---|---|---|
| 36 | TCP JSON Listener | ✅ 完成 | 纯 TCP JSON 行协议 |
| 37 | HTTPS JSON Listener | ⚠️ 占位 | ListenerKind 枚举中定义，未实现 |
| 38 | 自定义加密协议 Listener | ⚠️ 占位 | ListenerKind::PrivateProto 定义，未实现 |
| 39 | Listener CRUD | ✅ 完成 | 创建/查询/更新/删除 |
| 40 | Listener 启用/禁用 | ✅ 完成 | Manager 每秒 reconcile |
| 41 | Listener 运行状态跟踪 | ✅ 完成 | Starting/Running/Stopped/Error |
| 42 | 默认 Listener 保护 | ✅ 完成 | 默认 listener 不可删除 |

### 2.5 认证与安全

| # | 功能 | 状态 | 实现说明 |
|---|---|---|---|
| 43 | Web Session 认证 | ✅ 完成 | 用户名密码 + Cookie Session + TTL |
| 44 | API Token 认证 | ✅ 完成 | Bearer / x-api-token 头 |
| 45 | Agent Token（明文） | ✅ 完成 | plain_token 模式 |
| 46 | Agent Token（挑战响应） | ✅ 完成 | HMAC-SHA256 challenge_response |
| 47 | 操作者识别 | ✅ 完成 | x-operator 头，审计日志关联 |
| 48 | RBAC / 权限分级 | ❌ 未实现 | 只有单用户，无角色概念 |
| 49 | 加密通信 | ❌ 未实现 | TCP 通道明文 JSON |

### 2.6 事件与通知

| # | 功能 | 状态 | 实现说明 |
|---|---|---|---|
| 50 | WebSocket 实时事件 | ✅ 完成 | 连接时 Snapshot + 增量推送 |
| 51 | Agent 生命周期事件 | ✅ 完成 | Connected/Registered/Heartbeat/Updated/Disconnected |
| 52 | 任务生命周期事件 | ✅ 完成 | Dispatched/Result/Cancelled/Updated |
| 53 | 命令会话事件 | ✅ 完成 | Opened/Updated/Closed/Result/OutputChunk |
| 54 | WebSocket 重连 + 历史回补 | ❌ 未实现 | 断连后无 event replay |

### 2.7 持久化与恢复

| # | 功能 | 状态 | 实现说明 |
|---|---|---|---|
| 55 | SQLite 持久化 | ✅ 完成 | agents/tasks/audits/listeners/agent_builds |
| 56 | 冷启动恢复 | ✅ 完成 | 重启后恢复 listener 状态，旧 agent 标记离线，运行中任务标记失败 |
| 57 | 数据库一致性 | ✅ 完成 | E2E 专门有 database + database_consistency 测试套件 |
| 58 | 数据库中断恢复 | ✅ 完成 | database_interruptions 测试套件覆盖 |

### 2.8 审计日志

| # | 功能 | 状态 | 实现说明 |
|---|---|---|---|
| 59 | 操作审计记录 | ✅ 完成 | operator/action/target/detail 四要素 |
| 60 | 审计查询（多维度过滤） | ✅ 完成 | 按 operator/action/target_kind/target_id 过滤 |
| 61 | 审计精度验证 | ✅ 完成 | audit_precision 测试套件与 DB 对比 |

### 2.9 Agent 构建

| # | 功能 | 状态 | 实现说明 |
|---|---|---|---|
| 62 | Agent 二进制构建 | ✅ 完成 | 指定 target_triple + profile 跨平台编译 |
| 63 | 构建产物管理 | ✅ 完成 | 状态跟踪（Pending/Succeeded/Failed） |
| 64 | 嵌入服务器地址 | ✅ 完成 | 编译时注入 server_addr + agent_token |
| 65 | 构建 Manifest | ✅ 完成 | JSON manifest 记录构建参数 |
| 66 | 下载构建产物 | ❌ 未实现 | API 可查但无下载端点 |
| 67 | Stager（分阶段加载） | ❌ 未实现 | 无 stager/dropper 生成 |

### 2.10 Web UI 与 API

| # | 功能 | 状态 | 实现说明 |
|---|---|---|---|
| 68 | RESTful API | ✅ 完成 | 完整的 CRUD + 操作 API |
| 69 | WebSocket 事件流 | ✅ 完成 | /events/ws + /web/terminal/ws |
| 70 | OpenAPI 文档 | ✅ 完成 | /openapi.yaml + /docs |
| 71 | Dashboard 统计 | ✅ 完成 | 在线 agent 数量、任务统计 |
| 72 | 健康检查 | ✅ 完成 | /health |
| 73 | Web Terminal | ✅ 完成 | HTTP + WebSocket 双通道终端 |

### 2.11 C2 高级特性

| # | 功能 | 状态 | 实现说明 |
|---|---|---|---|
| 74 | 后渗透模块（Post-Exploitation） | ❌ 未实现 | 无模块化后渗透框架 |
| 75 | 横向移动 | ❌ 未实现 | 无 p2p relay/smb/pipe 等 |
| 76 | 数据暂存（Data Staging） | ❌ 未实现 | 无文件暂存服务 |
| 77 | 多人协作（Multi-user） | ❌ 未实现 | 单用户，无角色/权限 |
| 78 | 插件/扩展系统 | ❌ 未实现 | 无插件加载机制 |
| 79 | C2 Profile（流量伪装） | ❌ 未实现 | 无 malleable C2 profile |
| 80 | P2P 链式通信 | ❌ 未实现 | 无 agent-to-agent relay |
| 81 | DNS 隧道 | ❌ 未实现 | — |
| 82 | Agent 睡眠/唤醒 | ⚠️ 部分 | 有 sleep/jitter 配置，但无主动唤醒机制 |

---

## 三、完成度统计

### 按状态分类

| 状态 | 数量 | 占比 |
|---|---|---|
| ✅ 已完成 | 55 | 67.1% |
| ⚠️ 部分实现 / 占位 | 5 | 6.1% |
| ❌ 未实现 | 22 | 26.8% |
| **合计** | **82** | 100% |

### 按功能域分类

| 功能域 | 完成 | 部分实现 | 未实现 | 小计 | 完成率 |
|---|---|---|---|---|---|
| Agent 管理 | 8 | 1 | 1 | 10 | 80% |
| 任务/命令系统 | 11 | 0 | 0 | 11 | 100% |
| 远程执行命令类型 | 4 | 0 | 10 | 14 | 29% |
| Listener 系统 | 5 | 2 | 0 | 7 | 71% |
| 认证与安全 | 5 | 0 | 2 | 7 | 71% |
| 事件与通知 | 4 | 0 | 1 | 5 | 80% |
| 持久化与恢复 | 4 | 0 | 0 | 4 | 100% |
| 审计日志 | 3 | 0 | 0 | 3 | 100% |
| Agent 构建 | 4 | 0 | 2 | 6 | 67% |
| Web UI 与 API | 6 | 0 | 0 | 6 | 100% |
| C2 高级特性 | 1 | 1 | 7 | 9 | 11% |

---

## 四、工作量评估

### 4.1 当前工作量

作为本科毕设，从**工程量**角度看：

| 维度 | 评价 |
|---|---|
| **代码量** | Server 端 ~12,000 行 Rust，加上 agent 和 client 三端合计估计 20,000+ 行，对于本科毕设**充分** |
| **架构复杂度** | Microkernel 消息总线 + Facade 分层 + 运行时状态机，远超本科课程设计水平，**优秀** |
| **测试覆盖** | 14 个 E2E 套件覆盖并发/中断/一致性/审计精度/边界，还有 kernel 层 unit tests，**扎实** |
| **文档** | 4,000+ 行架构文档，含三端协议说明，**完善** |

### 4.2 毕设答辩角度

**强项（要重点展示的）：**

1. **架构设计**：Microkernel 不是随便套的，是真的消息总线 + Facade 分层 + 依赖方向控制，这在答辩中很有说服力
2. **任务系统**：单目标 + 广播 + 取消 + 父子任务链，状态机完整
3. **交互式 Shell**：命令会话 + cwd 保持 + 流式输出 + WebSocket 推送，链路完整
4. **测试工程**：E2E 测试覆盖并发、中断、一致性，这不是摆设，能扛住答辩老师问
5. **Agent 构建**：编译时注入配置 + manifest，算是比较完整的功能
6. **审计系统**：全链路审计 + 精度测试，说明你对安全运营有理解

**薄弱项（答辩可能被问到的）：**

1. "文件上传下载怎么做？" → 未实现，但架构上可以通过扩展现有任务系统来做
2. "通信加密呢？" → 目前 TCP 明文 JSON，HTTPS Listener 只有占位
3. "后渗透能力呢？" → 截图、键盘记录、进程操作都没实现
4. "横向移动？" → 未实现

---

## 五、可扩展方向（按性价比排序）

以下建议按"实现成本 vs 答辩加分"排序，越前面越值得做：

### 🔥 高性价比（1-3 天）

| # | 方向 | 说明 |
|---|---|---|
| 1 | **文件上传/下载** | C2 核心功能，协议层加 Upload/Download 消息类型，agent 端实现文件读写，通过现有任务系统下发 |
| 2 | **进程列表** | 命令类型扩展，agent 端调用 Windows API 枚举进程，结果作为任务结果返回 |
| 3 | **HTTPS Listener** | 补全 ListenerKind 的第二个变体，用 rustls 做加密通道 |
| 4 | **截图命令** | Agent 端调用 Windows GDI/API 截屏，结果以 base64 编码返回 |
| 5 | **Agent 分组 API** | 已有 tags 字段，补一个按标签过滤的查询接口即可 |

### 🟡 中等性价比（3-7 天）

| # | 方向 | 说明 |
|---|---|---|
| 6 | **键盘记录** | Agent 端 SetWindowsHookEx 全局钩子，缓冲上报 |
| 7 | **凭证获取** | 调用 Windows API 或 mimikatz-style 操作 |
| 8 | **构建产物下载** | 补一个文件下载端点，加 header 处理 |
| 9 | **WebSocket 事件回补** | 断连重连时补发 missed events |
| 10 | **自定义加密协议 Listener** | 第三种 Listener，展示协议设计能力 |

### 🔵 低性价比（仅加分项）

| # | 方向 | 说明 |
|---|---|---|
| 11 | 横向移动 / P2P relay | 复杂度高，本科毕设不要求 |
| 12 | C2 Profile（流量伪装） | 锦上添花 |
| 13 | 多用户 / RBAC | 工程量大，收益有限 |
| 14 | 插件系统 | 设计工作量大 |
| 15 | DNS 隧道 | 实现复杂 |

---

## 六、总结

### 完成度评级

> **核心平台完成度：约 85%**  
> **C2 功能完整度：约 40%**（缺少文件传输、后渗透、加密通信等）

区分这两个维度很重要——**平台本身做得很好**（架构、任务系统、事件系统、测试），但作为 C2 框架，缺少一些"标志性"功能（文件传输、截图、加密通信）。

### 答辩建议

1. **重点讲架构**：Microkernel 分层、消息总线、Facade 模式、状态机设计
2. **重点讲测试**：E2E 测试的全面性说明你对工程质量有追求
3. **提前准备"为什么没做 X"的回答**：例如"文件传输架构已预留（任务系统可扩展），时间有限未实现 agent 端"
4. **如果能补上文件上传下载和 HTTPS Listener**，完成度会大幅提升，答辩时也更站得住脚

### 工作量判定

对于本科毕设：**工作量达标，且有富余**。单就 server 端的架构设计和测试工程就已经超出本科平均水平。三端联动（server + web client + Windows agent）的综合工作量更是足够。

唯一的风险点是如果答辩老师从"功能完整度"角度提问，可能会觉得缺少文件传输和后渗透功能。建议至少补上文件上传下载，这是 C2 最基本的能力之一。

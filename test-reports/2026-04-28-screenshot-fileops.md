# Hermes Agent 截图与文件操作测试报告

> **日期**: 2026-04-28 14:00 ~ 14:15  
> **测试人**: 夏沐  
> **延续**: `2026-04-28-agent-windows-e2e.md` 基础链路验证后的功能测试  
> **环境**: 与前次相同 (macOS 172.18.64.111 + Windows 10 172.18.64.192)

---

## 1. 测试范围

本次测试覆盖 Hermes Agent 的两个内置功能模块：

1. **截图 (screenshot)** — 通过 PowerShell `System.Drawing` 截取 Windows 桌面
2. **文件管理 (browse / download / upload)** — 目录浏览、文件下载、文件上传

两者均为 agent 的内置命令，不走通用 shell 执行，而是有独立的处理函数。

---

## 2. 截图测试

### 2.1 测试命令

```
POST /agents/DESKTOP-46OSH8B/tasks
{"command": "screenshot"}
```

**task_id**: `task-18`

### 2.2 Agent 处理流程

```
Agent 收到 screenshot 命令
  → is_sys_op("screenshot") → true
  → sys_ops::handle_screenshot()
    → powershell -NoProfile -NonInteractive -Command <script>
      Add-Type -AssemblyName System.Windows.Forms
      Add-Type -AssemblyName System.Drawing
      $screen = [System.Windows.Forms.Screen]::PrimaryScreen
      $bounds = $screen.Bounds
      $bmp = New-Object System.Drawing.Bitmap($bounds.Width, $bounds.Height)
      $graphics = [System.Drawing.Graphics]::FromImage($bmp)
      $graphics.CopyFromScreen($bounds.Location, [System.Drawing.Point]::Empty, $bounds.Size)
      $ms = New-Object System.IO.MemoryStream
      $bmp.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
      [Convert]::ToBase64String($ms.ToArray())
    → 返回 base64 编码的 PNG
  → AgentMessage::TaskResult { success: true, output: <base64> }
```

### 2.3 结果

| 项目 | 值 |
|---|---|
| 状态 | `succeeded` |
| success | `true` |
| PNG 尺寸 | 1024 × 768 |
| Bit depth | 8 |
| Color type | 6 (RGBA) |
| 文件大小 | 3,179 bytes (base64: 4,240 chars) |
| PNG 文件头 | `89504e47` ✅ 有效 |
| **画面内容** | **全黑 (纯黑像素)** |

### 2.4 分析

截图技术上成功（生成了有效的 PNG 文件），但画面内容为纯黑。原因：

- Windows 主机通过 SSH 远程管理，**没有活跃的交互式桌面会话**
- `CopyFromScreen` 依赖桌面窗口管理器 (DWM) 渲染的画面
- Session 0 隔离或无桌面登录状态下，屏幕缓冲区为空/黑色

**这不是 Agent 或 Server 的 bug，是 Windows 远程会话的已知限制。**

**改善方向：**
- 可改用 DirectX Desktop Duplication API (`IDXGIOutputDuplication`)，能获取 DWM 合成后的画面
- 或在 Agent 启动前确保有活跃的桌面会话（RDP 登录、auto-logon）

**结论: ⚠️ 技术上通过，功能受限于无桌面会话**

---

## 3. 文件操作测试

### 3.1 目录浏览 (browse)

#### 测试 1: 浏览 `C:\`

```
POST /agents/DESKTOP-46OSH8B/tasks
{"command": "browse", "payload": "{\"path\":\"C:\\\"}"}
```

**task_id**: `task-19`

**结果**: ✅ 成功，返回 22 个条目

```
DIR  $Recycle.Bin                             size=0
DIR  $SysReset                                size=0
DIR  $WinREAgent                              size=0
FILE $WINRE_BACKUP_PARTITION.MARKER           size=0
FILE appverifUI.dll                           size=112,496
FILE Documents and Settings                   size=0
FILE DumpStack.log.tmp                        size=8,192
DIR  inetpub                                  size=0
FILE pagefile.sys                             size=17,179,869,184
DIR  PerfLogs                                 size=0
DIR  Program Files                            size=0
DIR  Program Files (x86)                      size=0
DIR  ProgramData                              size=0
DIR  Recovery                                 size=0
FILE swapfile.sys                             size=16,777,216
DIR  Symbols                                  size=0
DIR  System Volume Information                size=0
DIR  Users                                    size=0
FILE vfcompat.dll                             size=68,128
DIR  Windows                                  size=0
DIR  _titan_bait                              size=0
DIR  项目文档                                     size=0
```

每个条目包含字段：
- `name` — 文件/目录名
- `is_dir` — 是否目录
- `size` — 文件大小 (bytes)
- `modified` — 修改时间 (Unix timestamp)

#### 测试 2: 浏览 `C:\Users\macuser\Desktop`

**task_id**: `task-24`

**结果**: ✅ 成功，返回 2 个条目

```
FILE agent.exe          393,216 bytes
FILE hermes-test.txt     29 bytes      ← 本次测试上传的文件
```

**结论: ✅ 完全通过** — 目录浏览功能正常，元数据完整

---

### 3.2 文件下载 (download)

**测试命令:**

```
POST /agents/DESKTOP-46OSH8B/tasks
{"command": "download", "payload": "C:\\Windows\\win.ini"}
```

**task_id**: `task-22`

**结果**: ✅ 成功

| 项目 | 值 |
|---|---|
| 状态 | `succeeded` |
| 原始文件 | `C:\Windows\win.ini` |
| base64 长度 | 124 chars |
| 解码后大小 | 92 bytes |
| 编码正确 | ✅ 解码成功 |

**文件内容:**

```ini
; for 16-bit app support
[fonts]
[extensions]
[mci extensions]
[files]
[Mail]
MAPI=1
```

文件已保存到 `test-reports/downloaded-win.ini`。

**结论: ✅ 完全通过** — 文件下载、base64 编码、传输、解码全链路正常

---

### 3.3 文件上传 (upload)

**测试命令:**

```
POST /agents/DESKTOP-46OSH8B/tasks
{"command": "upload", "payload": "{\"remote_path\":\"C:\\Users\\macuser\\Desktop\\hermes-test.txt\",\"content_base64\":\"SGVsbG8gZnJvbSBIZXJtZXMgQWdlbnQgdGVzdCE=\"}"}
```

- 上传内容: `"Hello from Hermes Agent test!"` (29 bytes)
- base64: `SGVsbG8gZnJvbSBIZXJtZXMgQWdlbnQgdGVzdCE=`

**task_id**: `task-23`

**结果**: ✅ 成功

| 项目 | 值 |
|---|---|
| 状态 | `succeeded` |
| Server 返回 | `uploaded 29 bytes to C:\Users\macuser\Desktop\hermes-test.txt` |
| SSH 验证 | `type C:\Users\macuser\Desktop\hermes-test.txt` → `Hello from Hermes Agent test!` |

通过 SSH 登录 Windows 直接 `type` 验证了文件内容完全一致。

**结论: ✅ 完全通过** — 文件上传、base64 解码、写入全链路正常

---

## 4. 测试总结

### 4.1 功能覆盖

| 功能 | 命令 | 状态 | 备注 |
|---|---|---|---|
| 截图 | `screenshot` | ⚠️ 受限 | PNG 有效但全黑，需要活跃桌面会话 |
| 进程列表 | `ps` | 未测试 | — |
| 目录浏览 | `browse` | ✅ 通过 | 元数据完整 (name, is_dir, size, modified) |
| 文件下载 | `download` | ✅ 通过 | base64 编码传输，解码正确 |
| 文件上传 | `upload` | ✅ 通过 | base64 解码写入，SSH 验证内容一致 |

### 4.2 数据流路径

```
Operator → HTTP API → Server (task queue)
                          │
                     Agent heartbeat (15s)
                          │
                   DispatchTask ────────► Agent
                                            │
                                    ┌───────┴───────┐
                                    │               │
                               screenshot      browse/download/upload
                                    │               │
                           PowerShell         fs::read / fs::write
                           System.Drawing     base64 encode/decode
                                    │               │
                              TaskResult ◄──── TaskResult
                                    │               │
                              Server ◄──────────────┘
                                    │
                           GET /tasks (API)
                                    │
                              Operator
```

### 4.3 已知问题

1. **截图全黑**: 无活跃桌面会话时 `CopyFromScreen` 返回黑色画面。建议改用 DXGI Desktop Duplication 或检测桌面会话状态。
2. **browse 的 `is_dir` 字段**: `Documents and Settings`（应该是 junction/目录）被标记为 `FILE`，可能需要特殊处理 reparse point。
3. **大文件下载**: 未测试大文件的 base64 传输性能。Agent 有 `max_output_chars = 6000` 的限制，大文件可能被截断。

### 4.4 文件产物

```
test-reports/
├── screenshot-task-18.png          # 截图 (1024x768, 全黑)
├── downloaded-win.ini              # 下载的 win.ini (92 bytes)
├── 2026-04-28-agent-windows-e2e.md # 基础链路测试报告
└── 2026-04-28-screenshot-fileops.md # 本报告
```
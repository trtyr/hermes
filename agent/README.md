# Hermes Agent (Windows Ops)

Windows-oriented remote operations agent for Hermes server.

## Run

```bash
cargo run
```

连接地址和默认 token 由编译期嵌入配置决定，Agent 运行时不依赖环境变量。

默认情况下：

- 二进制名为 `agent` 或 `agent-*` 时，`agent_id` 使用主机名。
- 如果你把二进制重命名成别的文件名，`agent_id` 会使用可执行文件名。

## Metadata reported to server

On connect, agent reports:

- `hostname`
- `internal_ip`
- `external_ip` (derived by server from remote socket)
- `os`
- `arch`
- `username`
- `pid`
- `sleep_interval`
- `jitter`

## Supported operations (from server `/send <uuid> <message>`)

- `help`
- `ping`
- `sysinfo`
- `hostname`
- `whoami`
- `uptime`
- `disk` / `df`
- `ps`
- `ls [path]`
- `cat <path>`
- `exec <shell command>`

Examples:

- `/send <uuid> ping`
- `/send <uuid> sysinfo`
- `/send <uuid> exec uname -a`

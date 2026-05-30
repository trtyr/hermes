# Hermes Agent

[![Rust](https://img.shields.io/badge/rust-2021+-ed8225?style=flat-square&logo=rust&logoColor=white)](https://rust-lang.org)
[![License](https://img.shields.io/badge/license-unlicensed-22C55E?style=flat-square)]()
[![Platform](https://img.shields.io/badge/platform-cross--platform-8B5CF6?style=flat-square)]()

[GitHub](https://github.com/trtyr/hermes) · [Quick Start](#quick-start) · [Protocol](#protocol) · [Building](#building)

**Deployable C2 agent binary.** Runs on target hosts, connects to Hermes Server via TCP listener gateways, reports sysinfo, and executes dispatched tasks and commands. Designed for minimal binary size and zero disk footprint.

## Quick Start

```bash
cargo run
```

Connection address and default token are compile-time embedded. The agent has no runtime environment variables or config files.

## Agent Identity

- Default `agent_id`: hostname (when binary is named `agent` or `agent-*`)
- Renamed binary: `agent_id` uses the executable filename instead

## Metadata Reported to Server

On connect, agent reports:

| Field | Description |
|---|---|
| `hostname` | Machine hostname |
| `internal_ip` | Local network IP |
| `external_ip` | Derived by server from remote socket |
| `os` | Operating system |
| `arch` | CPU architecture |
| `username` | Current user |
| `pid` | Process ID |
| `sleep_interval` | Heartbeat interval |
| `jitter` | Heartbeat jitter range |

## Protocol

JSON over TCP. Core flow:

```
Agent                    Server
  |--- register ------------->|  (agent_id, hostname, os, arch, sysinfo)
  |<-- ack ------------------|  (session_id, beacon_interval, challenge?)
  |--- heartbeat ------------>|  (periodic, with jitter)
  |<-- dispatch_task ---------|  (task_id, command, timeout, work_dir)
  |--- task_result ---------->|  (task_id, exit_code, stdout, stderr, duration)
  |<-- open_session ----------|  (session_id)
  |--- session_output ------->|  (chunked stdout/stderr)
  |<-- close_session ---------|  (session_id)
  |--- proxy_* <------------->|  (TCP tunnel data)
```

### Auth Modes

| Mode | Behavior |
|---|---|
| Empty token | Open registration — any agent on the listener can register |
| `plain_token` | First registration frame must include matching token |
| `challenge_response` | Server sends nonce, agent responds with `HMAC-SHA256(key: agent_token, message: session_nonce + ":" + agent_id)` |

## Supported Operations

Server-initiated commands:

| Command | Description |
|---|---|
| `help` | List available commands |
| `ping` | Connectivity check |
| `sysinfo` | Full system information |
| `hostname` | Machine hostname |
| `whoami` | Current user |
| `uptime` | System uptime |
| `disk` / `df` | Disk usage |
| `ps` | Process list |
| `ls [path]` | Directory listing |
| `cat <path>` | Read file contents |
| `exec <cmd>` | Execute shell command |

## Building

- **Rust** ≥ 1.56 (edition 2021)
- **No C library required** — pure Rust

```bash
cargo run                  # dev run
cargo build --release      # optimized build (LTO + strip + panic=abort)
cargo build --features tls # with TLS support
```

### Cross-Compilation

```bash
cargo build --release --target x86_64-pc-windows-msvc
```

### Release Profile

The release profile is tuned for minimal binary size:

- `opt-level = "z"` — minimize size
- `lto = true` — link-time optimization
- `strip = true` — strip debug symbols
- `panic = "abort"` — no unwinding
- `codegen-units = 1` — maximize optimization

## Architecture

```
agent/src/
├── main.rs              # Entry point: init services, connect loop, command dispatch
├── protocol/            # Protocol layer (messages, config, crypto)
├── kernel/              # Microkernel core (memory, plugin, scheduler)
├── server.rs            # Embedded server connection profile (compile-time replaced)
├── ops.rs               # Local command execution helpers
├── sys/                 # Platform abstraction (hostname, username, pid, os, arch, ip)
└── services/            # Service layer
    ├── network.rs       # TCP/TLS connection with reconnect backoff
    ├── heartbeat.rs     # Jittered heartbeat loop
    ├── task.rs          # Task dispatch, execution, result reporting
    ├── session.rs       # Interactive command sessions
    ├── proxy.rs         # TCP tunnel proxy
    ├── file_ops.rs      # File read/write/list/search
    └── sys_ops.rs       # Process list, screenshot (Windows)
```

---

⭐ Found this useful? Give it a star on [GitHub](https://github.com/trtyr/hermes).

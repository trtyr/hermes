---
timestamp: 2026-05-28T10:25:35Z
commit: 31ffbf4
---

# Agent — Deployable C2 Agent Binary

## OVERVIEW

Standalone Rust binary deployed on target hosts. Connects to Hermes Server via TCP listener gateways, registers with sysinfo, maintains heartbeat, and executes dispatched tasks and commands. 21 source files, edition 2021.

## STRUCTURE

```
agent/
├── src/
│   ├── main.rs              # Entry point: parse args, init services, connect loop
│   ├── protocol.rs           # AgentMessage (18 variants), ServerCommand (15 variants)
│   ├── secure.rs             # SecureServerAddr (XOR-encrypted heap, zeroed on Drop)
│   ├── config.rs             # Compile-time embedded config (server addr, TLS, agent_token)
│   └── services/             # Service layer
│       ├── network.rs        # TCP/TLS connection to server
│       ├── heartbeat.rs      # Jittered heartbeat, register→ack→heartbeat loop
│       ├── task.rs            # Task dispatch, execution, cancel, result reporting
│       ├── command_session.rs # Command session: open→execute→close lifecycle
│       ├── session.rs         # Working directory tracking per command session
│       ├── proxy.rs           # TCP tunnel proxy service
│       ├── file_ops.rs        # File read/write/list/search operations
│       └── sys_ops.rs         # Process list, screenshot (Windows), system info
├── Cargo.toml
└── build.rs                   # Embed server config at compile time
```

## WHERE TO LOOK

| Concern | Location |
|---|---|
| Connection lifecycle | `services/network.rs` — TCP/TLS connect with reconnect backoff |
| Protocol message types | `protocol.rs` — `AgentMessage` (18 variants: register, heartbeat, task_result, sysinfo, etc.), `ServerCommand` (15 variants: ack, dispatch_task, cancel_task, open_session, execute, close_session, proxy, etc.) |
| Heartbeat with jitter | `services/heartbeat.rs` — sends heartbeat at configurable interval with random jitter |
| Task execution | `services/task.rs` — receives dispatch, spawns subprocess, streams output, reports result |
| Command sessions | `services/command_session.rs` — server-initiated interactive shell sessions |
| Proxy tunneling | `services/proxy.rs` — TCP tunnel to target specified by server |
| File operations | `services/file_ops.rs` — read, write, list directory, search files |
| System operations | `services/sys_ops.rs` — process enumeration, screenshot capture |
| Secure config storage | `secure.rs` — XOR-encrypted server address on heap, zeroed on Drop |
| Auth handshake | `protocol.rs` + `config.rs` — HMAC-SHA256 challenge-response (optional) |

## PROTOCOL

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
| `challenge_response` | Server sends nonce, agent responds with `HMAC-SHA256(nonce + token)` |

## COMMANDS

```bash
# Dev run
cargo run

# Release build (minimal binary size)
cargo build --release

# With TLS support
cargo build --features tls

# Cross-compile for Windows
cargo build --release --target x86_64-pc-windows-msvc
```

Release profile (`Cargo.toml` `[profile.release]`):
- `opt-level = "z"` — minimize binary size
- `lto = true` — link-time optimization
- `strip = true` — strip debug symbols
- `panic = "abort"` — no unwinding, smaller binary
- `codegen-units = 1` — maximize optimization

## ANTI-PATTERNS

- **Never** write to disk from the agent — all configuration is compile-time embedded. The agent is meant to be ephemeral and leave no trace.
- **Never** hardcode server addresses in plain string literals — use `SecureServerAddr` from `secure.rs`.
- **Never** add persistent state to the agent — no local databases, no log files, no config files.

## NOTES

- `agent_id` defaults to hostname; renaming the binary file changes the `agent_id` to the filename.
- Unsafe blocks exist for Windows API calls (screenshot, process enumeration) — these are legitimate and platform-essential.
- The agent is designed for minimal binary size (sub-1MB release builds common).
- Optional TLS feature gate: `#[cfg(feature = "tls")]` in network service.
- Agent binary is not independently versioned — follows the server release cycle.
- E2E tests recompile the agent with embedded test config (server address, token).

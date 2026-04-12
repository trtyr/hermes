# Hermes Server

`server` is the control plane in the current three-part architecture:

- `server`: task dispatch, agent session management, API, audit, persistence
- `web_client`: operations UI, calls HTTP API and subscribes to WebSocket events
- `agent client`: runs on office machines, connects to the TCP agent gateway

At this stage the main focus is `server`, so the API contract is treated as the stable integration point for the other two parts.

## Architecture Highlights

- Agent listener management: DB-backed listener registry with runtime enable/disable and future protocol extension points
- HTTP API: the stable management plane for operators and future `web_client`
- WebSocket events: real-time online agent and task state push
- SQLite storage: persisted agent history, task history, audit logs, listener state, and agent build records
- Agent build service: server-side generation of agent binaries for selected listeners and target platforms

Documentation map:

- `docs/README.md`: 文档总入口
- `docs/server-architecture/README.md`: `Server 架构`
- `docs/server-web-client/README.md`: `Server 和 Web Client`
- `docs/server-agent/README.md`: `Server 和 Agent`

## Defaults

- Default agent listener: `0.0.0.0:1234`
- HTTP API: `0.0.0.0:3000`

Config file: `config.toml`

## Configuration

```toml
[server]
host = "0.0.0.0"
port = 1234

[api]
host = "0.0.0.0"
port = 3000

[storage]
sqlite_path = "data/server.db"

[auth]
api_token = "dev-api-token"
agent_token = ""
web_username = "admin"
web_password = "123456"
session_ttl_secs = 28800
```

Auth behavior:

- `auth.web_username` + `auth.web_password`: backend login is enabled
- `POST /auth/login`: frontend exchanges username/password for a backend session
- protected HTTP API and WebSocket routes accept backend session authentication
- `auth.api_token` is kept as a legacy compatibility path for scripted callers
- `auth.agent_token` empty: agent registration does not require token
- `auth.agent_token` set: the first agent frame must include the matching token

## HTTP API

Current routes:

- `GET /health`
- `POST /auth/login`
- `POST /auth/logout`
- `GET /auth/me`
- `GET /dashboard/stats`
- `GET /agents`
- `GET /agents/history`
- `GET /agents/:agent_id`
- `DELETE /agents/:agent_id`
- `POST /agents/:agent_id/disable`
- `POST /agents/:agent_id/enable`
- `POST /agents/:agent_id/beacon-config`
- `POST /agents/:agent_id/command-sessions`
- `POST /agents/:agent_id/disconnect`
- `POST /agents/:agent_id/tasks`
- `GET /command-sessions`
- `GET /command-sessions/:command_session_id`
- `GET /command-sessions/:command_session_id/commands`
- `GET /command-sessions/:command_session_id/commands/:command_id`
- `POST /command-sessions/:command_session_id/commands`
- `POST /command-sessions/:command_session_id/execute`
- `POST /command-sessions/:command_session_id/close`
- `GET /tasks`
- `GET /tasks/:task_id`
- `POST /tasks/broadcast`
- `GET /audits`
- `GET /listeners`
- `GET /listeners/:listener_id`
- `POST /listeners`
- `PATCH /listeners/:listener_id`
- `POST /listeners/:listener_id/enable`
- `POST /listeners/:listener_id/disable`
- `DELETE /listeners/:listener_id`
- `GET /agent-builds`
- `GET /agent-builds/:build_id`
- `POST /agent-builds`
- `GET /events/ws`

Agent lifecycle semantics:

- `POST /agents/:agent_id/disable`: administratively disable an agent asset; blocks new registration and task dispatch
- `POST /agents/:agent_id/enable`: clear the disabled flag and allow the agent to return
- `DELETE /agents/:agent_id`: delete the persisted agent asset record; only allowed when the agent is offline
- `POST /agents/:agent_id/disconnect`: ask an online agent session to exit
- disabled: asset is retained but blocked from reconnecting or receiving tasks
- offline: agent session is gone, but persisted asset/history still exists
- recover: agent reconnects and recreates its live session from persisted identity

Command-session semantics:

- no PTY; every line still runs as a short-lived process
- session only keeps context, especially `cwd`
- `cd` and `pwd` are handled as session-aware built-ins
- command sessions are separate from durable task dispatch

Listener semantics:

- HTTP API remains a single stable management endpoint
- agent ingress listeners are managed separately from the API listener
- listeners are persisted in SQLite and can be enabled, disabled, and queried over API
- current production driver is `tcp_json`; `https_json` and `private_proto` are reserved extension points

Agent build semantics:

- server generates agent binaries matched to a selected listener
- builds are recorded in SQLite for later traceability
- current supported workflows include host builds plus tested Windows and Linux cross-builds
- artifact generation is local; distribution is intentionally out of scope for now

## Agent Protocol

Detailed protocol docs: `docs/server-agent/agent-protocol.md`

Core flow:

1. Agent connects to TCP gateway
2. Agent sends `register`
3. Server returns `ack`
4. Agent sends `heartbeat`
5. Server pushes `dispatch_task`
6. Agent sends `task_result`

## Quick Start

```bash
make run
```

Without `make`:

```bash
cargo run
```

## Common Commands

Unified local commands live in `Makefile`.

| Goal | Command |
| --- | --- |
| start server | `make run` |
| dev compile check | `make check` |
| format | `make fmt` |
| run unit tests | `make test` |
| run default regression bundle | `make e2e` |
| run full regression bundle | `make e2e-all` |
| run one suite | `make e2e-suite SUITE=listeners` |
| release build | `make build-release` |
| local CI bundle | `make ci` |

## Regression Suites

Default regression:

```bash
make e2e
```

Full regression, including agent build coverage:

```bash
make e2e-all
```

Note: `make e2e-all` expects the sibling agent project to exist at `../agent`.

One suite:

```bash
make e2e-suite SUITE=basic
make e2e-suite SUITE=listeners
make e2e-suite SUITE=command_session
```

Available suite names:

- `basic`
- `auth`
- `audit_precision`
- `command_session`
- `concurrent_stress`
- `database`
- `database_consistency`
- `database_interruptions`
- `edge`
- `lifecycle`
- `listeners`
- `agent_builds`

## Agent Binary Generation

The server can build the sibling agent project and inject listener defaults into the binary.

Example request:

```json
{
  "listener_id": 1,
  "profile": "release",
  "target_triple": "x86_64-pc-windows-msvc"
}
```

or:

```json
{
  "listener_id": 1,
  "profile": "release",
  "target_triple": "x86_64-unknown-linux-musl"
}
```

Artifacts are written under `data/agent-builds/`.

## Repository Conventions

- default branch: `main`
- release tags: `server-vMAJOR.MINOR.PATCH`
- release notes/checklist: `docs/server-architecture/release-process.md`

---
timestamp: 2026-05-28T10:25:35Z
commit: 31ffbf4
---

# Hermes Server — Agent Working Guide

## OVERVIEW

Hermes Server is the control plane in a three-part C2 architecture:

- **server** (this repo): task dispatch, agent session management, HTTP API, WebSocket events, audit, persistence, agent binary generation
- **web_client**: browser UI, calls HTTP API + subscribes to WebSocket events
- **agent_client**: deployed on target hosts, connects via listener gateways

The API contract is the stable integration surface between the three parts.

## Architecture: Microkernel Control Plane

This project follows a strict microkernel design. The dependency chain is:

```
external input → facade → KernelMessage → runtime dispatcher → state mutation → effects → storage / event bus
```

### Layer Map (outside → inside)

| Layer | Code | Rule |
|---|---|---|
| Protocol | `src/protocol.rs` | Data contracts only. No business logic. |
| Adapters | `src/api/*`, `src/agent/gateway.rs`, `src/agent/listeners/*` | Handle transport concerns. **Must NOT directly modify `KernelState`.** Go through `KernelHandle` or facades. |
| Service Facades | `src/kernel/service/*` | Stable kernel entry points. New features go here, not in API handlers. |
| Kernel Runtime | `src/kernel/runtime/*`, `src/kernel/message.rs`, `src/kernel/bus.rs` | Routes domain messages, drives state transitions, triggers side effects via `RuntimePorts`. |
| State | `src/kernel/state/*` | Authoritative in-memory state. State machines live here, not scattered elsewhere. |
| Storage | `src/kernel/storage/*` | SQLite persistence and cold-start loading. No orchestration authority. |

### Dependency Direction (mandatory)

```
main → api/gateway/listeners → kernel/service → kernel/{message, state, storage}
kernel/runtime → kernel/state + runtime effects → storage + event bus
```

**Forbidden reversals:**
- `state` depending on `api`
- `storage` depending on `runtime handler`
- `api` directly depending on `storage` for domain decisions

### Domain Facades in `kernel/service/`

- `AgentCommandFacade` — agent lifecycle commands (disable, enable, disconnect, beacon config)
- `AgentQueryFacade` — agent queries
- `TaskFacade` — task dispatch, broadcast, cancel
- `CommandSessionFacade` — open/execute/close command sessions
- `ListenerCommandFacade` / `ListenerQueryFacade` — listener CRUD
- `AuthFacade` — web login sessions
- `AgentBuildFacade` — agent binary builds

### Architecture Docs

Detailed per-domain docs under `docs/server-architecture/`. Read `docs/server-architecture/server-architecture.md` first, then the specific domain doc for whatever you're changing.

## Commands

| Goal | Command |
|---|---|
| Start server | `make run` |
| Compile check | `make check` |
| Format | `make fmt` |
| Unit tests | `make test` |
| Default E2E regression | `make e2e` |
| Full regression (incl. agent builds) | `make e2e-all` |
| Single E2E suite | `make e2e-suite SUITE=listeners` |
| Release build | `make build-release` |
| Local CI (fmt → check → test → e2e) | `make ci` |

### E2E Suites

Python-based integration tests in `scripts/e2e/`. They spawn a real server binary, hit the HTTP API, and assert responses.

Available suite names: `basic`, `auth`, `audit_precision`, `command_session`, `concurrent_stress`, `database`, `database_consistency`, `database_interruptions`, `edge`, `lifecycle`, `listeners`, `agent_builds`.

**Prerequisites:**
- E2E tests require a compiled `target/debug/server` binary (`cargo build` or `cargo test` first).
- `make e2e-all` additionally requires the sibling agent project at `../agent` with a compiled `target/debug/agent` binary.

### Unit Tests

Rust unit tests are `#[cfg(test)]` modules co-located with source:
- `src/kernel/state/tests.rs` — state layer unit tests
- `src/kernel/runtime/tests.rs` — runtime dispatcher tests (creates temp SQLite DB per test)

## Tech Stack & Build Notes

- **Rust edition 2024** — uses modern edition features.
- **Async runtime**: tokio with `full` features + `test-util`.
- **HTTP framework**: axum 0.8 with WebSocket support.
- **Database**: SQLite via `rusqlite` with `bundled` feature (no external SQLite needed).
- **Config**: loaded from `config.toml` at working directory root via `Config::get_config()`.
- **Dev/test profiles**: `debug = 0`, `incremental = true` — faster builds, no debug symbols.
- **CORS**: wide-open (`Any` origin) — by design for development.
- **Default ports**: TCP agent listener `0.0.0.0:1234`, HTTP API `0.0.0.0:3000`.

## Repository Conventions

- Default branch: `main`
- Release tags: `server-vMAJOR.MINOR.PATCH`
- Release checklist: `docs/server-architecture/release-process.md`
- Config file: `config.toml` (root)
- SQLite data dir: `data/` (gitignored)
- Agent build artifacts: `data/agent-builds/`
- Test fixture/temp file: `test1` (gitignored)

## Auth Model

Three auth paths coexist:

1. **Web session auth** (`auth.web_username` + `auth.web_password`): login via `POST /auth/login`, get session cookie, used for API and WebSocket routes.
2. **API token** (`auth.api_token`): legacy compatibility for scripted callers via `x-api-token` header.
3. **Agent token** (`auth.agent_token`): empty = open registration; set = first agent frame must include matching token. Supports `plain_token` and `challenge_response` modes.

## When Adding Features

1. Find the correct domain facade in `src/kernel/service/`.
2. Add the domain message variant to `src/kernel/message.rs` if needed.
3. Handle it in the appropriate runtime handler under `src/kernel/runtime/`.
4. Wire the API handler in `src/api/` to call the facade — **never** reach into state or storage directly.
5. Add state logic in `src/kernel/state/` if new state is needed.
6. Add storage logic in `src/kernel/storage/` if persistence is needed.
7. Route events through `RuntimePorts` for side effects (persistence + WebSocket broadcast).

## NOTES

- `KernelState` is held by the kernel loop, accessed via `Arc<Mutex<KernelState>>` through facades.
- `RuntimePorts` abstracts all I/O: storage writes, event bus publish, agent message sending.
- E2E tests with names like `database_consistency`, `concurrent_stress`, `fault_matrix` exist — consult `scripts/e2e/` before writing new tests.
- Temp SQLite databases are created per test in `kernel/runtime/tests.rs`.
- Server must be compiled before running E2E: `cargo build` then `make e2e`.
- No Docker config, no CI pipeline — local `make ci` only.
- CORS is wide-open (`Any` origin) by design for development.
- There's a documented release checklist at `docs/server-architecture/release-process.md`.

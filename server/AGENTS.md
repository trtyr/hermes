---
timestamp: 2026-06-01T03:18:25Z
commit: bee5081
---

# Hermes Server — Agent Working Guide

## OVERVIEW

Hermes Server is the control plane in a three-part C2 architecture:

- **server** (this repo): task dispatch, agent session management, HTTP API, WebSocket events, audit, persistence, agent binary generation
- **web_client**: browser UI, calls HTTP API + subscribes to WebSocket events
- **agent_client**: deployed on target hosts, connects via listener gateways

Stack: Rust 2024, tokio, axum 0.8, rusqlite (bundled). 163 `.rs` files, ~21k lines.

## STRUCTURE

```
src/
├── main.rs              # Entry: loads config → new_kernel() → try_join!(gateway, http_api)
├── lib.rs               # Re-exports 5 modules for integration tests
├── protocol.rs          # All DTOs, events, message types (548 lines, pure data, no behavior)
├── console.rs           # Custom logging (eprintln!, no tracing/log crate)
├── api/                 # HTTP route handlers (54 routes, 11 sub-APIs)
├── agent/               # Agent gateway + listener adapters (TCP, HTTPS)
└── kernel/              # Microkernel: service, runtime, state, storage
scripts/e2e/             # Python E2E test suites (16 suites)
docs/server-architecture/ # Per-domain architecture docs (11 files)
```

## Architecture: Microkernel Control Plane

Dependency chain:
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

## CODE MAP

**Hot symbols (most connected):**

| Symbol | Kind | File | Degree | Role |
|---|---|---|---|---|
| `KernelState` | struct | `src/kernel/state/types.rs` | 99 | Central in-memory state, 15 HashMap fields |
| `KernelHandle` | struct | `src/kernel/service/handle/types.rs` | 85 | Facade entry point, all kernel access |
| `RuntimePorts` | struct | `src/kernel/runtime/effects.rs` | 76 | Side-effect abstraction (persistence + events) |
| `AppState` | struct | `src/api/common/app_state.rs` | 67 | Axum state wrapping KernelHandle |
| `authorize_api` | fn | `src/api/common/auth.rs` | 57 | Auth middleware (session/token/bearer) |
| `build_router` | fn | `src/api/mod.rs` | 52 | Assembles 54 routes across 11 sub-APIs |
| `ListenerRecord` | struct | `src/protocol.rs` | 51 | Listener data contract |
| `WebEvent` | enum | `src/protocol.rs` | 48 | WebSocket event types |

**Feature clusters (by connectivity):**

| Cluster | Files | Symbols | Description |
|---|---|---|---|
| `kernel/service` | 30 | 208 | Domain facades — the facade layer |
| `kernel/runtime` | 20 | 102 | Message dispatch + domain handlers |
| `kernel/state` | 8 | 227 | In-memory state machines |
| `protocol` | 1 | 248 | All shared data types |
| `api/common` | 8 | 242 | Auth, AppState, paging, responses |
| `kernel/storage` | 9 | 94 | SQLite persistence |

**Startup call graph:**
```
main() → Config::get_config() → new_kernel() → tokio::try_join!(
  run_agent_gateway() → run_listener_manager() → reconcile loop → driver.spawn() → accept loop,
  run_http_api() → axum::serve(build_router()) → handlers → KernelHandle → KernelMessage → kernel_loop
)
```

## HEALTH

- Health score: **60/100**
- Cycles: 5 (all within `kernel/runtime` ↔ `kernel/runtime/agent_lifecycle`)
- God modules: `kernel/state` (227 symbols), `kernel/storage` (94 symbols), `kernel/runtime` (46 symbols), `api/agents` (21 symbols)
- Zero `unsafe` blocks, zero `.unwrap()` in production, zero TODO markers

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

- `KernelState` is held by the kernel loop, accessed via `Arc<RwLock<KernelState>>` through facades.
- `RuntimePorts` abstracts all I/O: storage writes, event bus publish, agent message sending.
- E2E tests with names like `database_consistency`, `concurrent_stress`, `fault_matrix` exist — consult `scripts/e2e/` before writing new tests.
- Temp SQLite databases are created per test in `kernel/runtime/tests.rs`.
- Server must be compiled before running E2E: `cargo build` then `make e2e`.
- No Docker config, no CI pipeline — local `make ci` only.
- CORS is wide-open (`Any` origin) by design for development.
- There's a documented release checklist at `docs/server-architecture/release-process.md`.

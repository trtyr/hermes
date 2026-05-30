# Hermes

[![Rust](https://img.shields.io/badge/rust-2024+-ed8225?style=flat-square&logo=rust&logoColor=white)](https://rust-lang.org)
[![Vue](https://img.shields.io/badge/vue-3-4FC08D?style=flat-square&logo=vue.js&logoColor=white)](https://vuejs.org)
[![License](https://img.shields.io/badge/license-unlicensed-22C55E?style=flat-square)]()
[![Platform](https://img.shields.io/badge/platform-cross--platform-8B5CF6?style=flat-square)]()

[GitHub](https://github.com/trtyr/hermes) · [Quick Start](#quick-start) · [Architecture](#architecture) · [Commands](#commands)

**Three-part C2 control platform.** Server, agent, and web client — each an independent project with its own build and toolchain, unified under a single workspace. Built on Rust (axum + tokio) and Vue 3 + TypeScript.

## Structure

| Component | Dir | Language | Purpose |
| --- | --- | --- | --- |
| **Server** | `server/` | Rust (edition 2024) | Control plane: HTTP API, WebSocket events, agent session management, task dispatch, SQLite persistence, agent binary generation |
| **Agent** | `agent/` | Rust (edition 2021) | Deployed on target hosts; connects to server via listener gateways; reports sysinfo, executes commands |
| **Web Client** | `client/` | Vue 3 + TypeScript + Vite | Browser operations UI; calls server HTTP API + subscribes to WebSocket events |

```
hermes/
├── server/             # Control plane (Rust 2024)
│   ├── src/api/        # HTTP route handlers (54 routes, 11 sub-APIs)
│   ├── src/kernel/     # Microkernel: services, runtime dispatch, state, storage
│   ├── src/agent/      # Agent gateway, listener handling
│   ├── scripts/e2e/    # Python E2E test suites (17 suites)
│   └── config.toml     # Server configuration
├── agent/              # Deployable agent binary (Rust 2021)
│   └── src/services/   # Network, heartbeat, task, proxy, file/sys ops
├── client/             # Browser UI (Vue 3 + TypeScript + Vite)
│   └── src/
│       ├── api/        # HTTP API client modules (10 modules)
│       ├── store/      # Pinia stores (events, connection, app, notifications)
│       ├── views/      # Page components (7 routes)
│       └── composables/# Terminal, WebSocket composables
└── docs/               # Workspace-level docs (plan trees, specs)
```

## Architecture

Each component builds independently — no workspace-level Cargo, no monorepo tooling.

- **Integration surface**: HTTP/WebSocket API contract between server and the other two components
- **Agent–Server protocol**: JSON over TCP — `register → ack → heartbeat ↔ dispatch_task / task_result`
- **Default ports**: TCP listener `0.0.0.0:1234`, HTTP API `0.0.0.0:3000`, Vite dev `5173`
- **Auth**: three coexisting paths — web session cookie, `x-api-token` header (legacy), agent token (configurable)

### Documentation Map

| Need | Go to |
|---|---|
| Server internals (microkernel, dispatch, state, storage) | `server/AGENTS.md`, `server/src/kernel/AGENTS.md` |
| Server API surface (route handlers, auth, DTOs) | `server/src/api/AGENTS.md` |
| Agent internals (services, protocol, build profile) | `agent/AGENTS.md` |
| Web client internals (routes, stores, composables) | `client/AGENTS.md` |
| Server ↔ Agent protocol | `server/docs/server-agent/` |
| Server ↔ Web Client docs | `server/docs/server-web-client/` |
| Server per-domain architecture docs | `server/docs/server-architecture/` |
| E2E test suite docs | `server/docs/e2e-guide.md` |

## Quick Start

### Server

```bash
cd server
make run
```

### Agent

```bash
cd agent
cargo run
```

Release build (minimal binary, LTO + strip + panic=abort):

```bash
cargo build --release
```

### Web Client

```bash
cd client
npm run dev
```

## Commands

### Server (`server/`)

| Goal | Command |
|---|---|
| Run | `make run` |
| Check | `make check` |
| Format | `make fmt` |
| Unit tests | `make test` |
| Default E2E | `make e2e` |
| Full E2E (incl. agent builds) | `make e2e-all` |
| Single E2E suite | `make e2e-suite SUITE=listeners` |
| Release build | `make build-release` |
| Local CI (fmt → check → test → e2e) | `make ci` |

E2E tests are Python scripts in `server/scripts/e2e/`. They spawn a real `target/debug/server` binary — compile first with `cargo build`.

`make e2e-all` requires `../agent/target/debug/agent` to exist.

### Agent (`agent/`)

```bash
cargo run                  # dev run
cargo build --release      # optimized build
cargo build --features tls # with TLS support
cargo build --release --target x86_64-pc-windows-msvc  # cross-compile for Windows
```

### Web Client (`client/`)

```bash
npm run dev    # Vite dev server
npm run build  # vue-tsc + vite build
```

## Building

### Server

- **Rust** ≥ 1.85 (edition 2024)
- **No C library required** — `rusqlite` uses bundled SQLite

### Agent

- **Rust** ≥ 1.56 (edition 2021)
- **No C library required** — pure Rust
- Optional `tls` feature: `cargo build --features tls`

### Web Client

- **Node.js** ≥ 18
- `npm install` to install dependencies
- `npm run dev` to start dev server

## Conventions

- Default branch: `main`
- Server release tags: `server-vMAJOR.MINOR.PATCH`
- Server config: `server/config.toml`
- Runtime data (SQLite DB, agent builds): `server/data/` (gitignored)
- Agent `agent_id` defaults to hostname; renaming the binary changes it to the filename
- Web client uses `@` path alias → `client/src/`

---

⭐ Found this useful? Give it a star on [GitHub](https://github.com/trtyr/hermes).

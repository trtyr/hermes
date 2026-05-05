# Hermes — Workspace Guide

Three-part C2 platform. Each subdirectory is an independent project with its own build and toolchain.

| Component | Dir | Language | Purpose |
|---|---|---|---|
| Server | `server/` | Rust (edition 2024) | Control plane: HTTP API, WebSocket events, agent session management, task dispatch, SQLite persistence, agent binary generation |
| Agent | `agent/` | Rust (edition 2021) | Deployed on target hosts; connects to server via listener gateways; reports sysinfo, executes commands |
| Web Client | `client/` | Vue 3 + TypeScript + Vite | Browser operations UI; calls server HTTP API + subscribes to WebSocket events |

## Key Facts

- **Integration surface**: the HTTP/WebSocket API contract between server and the other two parts. API routes documented in `server/README.md`.
- **No monorepo tooling**: each project builds independently. No workspace-level Cargo, no turborepo, no shared scripts.
- **Agent–Server protocol**: JSON over TCP. Core flow: `register → ack → heartbeat ↔ dispatch_task / task_result`. Docs in `server/docs/server-agent/`.
- **Default ports**: TCP listener `0.0.0.0:1234`, HTTP API `0.0.0.0:3000`, Vite dev `5173`.
- **Auth**: three coexisting paths — web session cookie, `x-api-token` header (legacy), agent token (configurable). See `server/README.md` → Configuration.
- **No CI pipeline**: local verification only (`make ci` in `server/`).

## Commands Quick Reference

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
cargo run          # dev run
cargo build --release  # optimized (LTO + strip + panic=abort)
```

Release profile is tuned for minimal binary size (`opt-level = "z"`, `codegen-units = 1`).

Optional `tls` feature: `cargo build --features tls`.

Cross-compilation: `--target x86_64-pc-windows-msvc`.

### Web Client (`client/`)

```bash
npm run dev    # Vite dev server
npm run build  # vue-tsc + vite build
```

## Architecture Pointers

- **Server internal architecture** → `server/AGENTS.md` (detailed microkernel layer map, dependency rules, domain facades).
- **Server per-domain docs** → `server/docs/server-architecture/`. Start with `server-architecture.md`.
- **Server ↔ Agent protocol** → `server/docs/server-agent/`.
- **Server ↔ Web Client** → `server/docs/server-web-client/`.

When working on server features, follow the microkernel pattern strictly: API handler → service facade → `KernelMessage` → runtime handler → state + storage. **Never** let API or adapter code reach into state/storage directly. See `server/AGENTS.md` for the full dependency rules.

## Conventions

- Default branch: `main`
- Server release tags: `server-vMAJOR.MINOR.PATCH`
- Server release checklist: `server/docs/server-architecture/release-process.md`
- Server config: `server/config.toml`
- Runtime data (SQLite DB, agent builds): `server/data/` (gitignored)
- Agent `agent_id` defaults to hostname; renaming the binary changes it to the filename
- Web Client uses `@` path alias → `client/src/`

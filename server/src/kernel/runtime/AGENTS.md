---
timestamp: 2026-06-01T03:18:25Z
commit: bee5081
---

# Kernel Runtime — Dispatch & Handlers

## OVERVIEW

The runtime layer is the microkernel's message-processing core. `kernel_loop()` in `dispatch.rs` receives `KernelMessage` variants from an mpsc channel and routes them to four domain handler functions. Handlers acquire write locks on `KernelState`, drive state transitions (state machines live in `state/`, not here), and trigger side effects through `RuntimePorts`.

## STRUCTURE

```
runtime/
├── mod.rs                  # Re-exports bootstrap + dispatch
├── dispatch.rs             # kernel_loop(), 4 route functions
├── bootstrap.rs            # new_kernel() — storage, state, handle, loop spawn
├── watchdog.rs             # heartbeat_watchdog() — 1s interval sweep
├── effects.rs              # RuntimePorts: event bus + state persistence
├── proxy.rs                # Proxy session handler
├── task_flow.rs            # Task dispatch + result handling
├── tests.rs                # 5 async tests + helpers
├── agent_lifecycle/        # 6 files — register, heartbeat, disconnect, disable/enable
└── command_sessions/       # 4 files — open, execute, close, result handling
```

## WHERE TO LOOK

| Concern | Location |
|---|---|
| Message routing (the central switch) | `dispatch.rs` — `kernel_loop()` |
| Kernel startup sequence | `bootstrap.rs` — `new_kernel()` |
| Side-effect abstraction | `effects.rs` — `RuntimePorts` struct |
| Agent registration + heartbeat | `agent_lifecycle/` |
| Task creation + result delivery | `task_flow.rs` |
| Command session lifecycle | `command_sessions/` |
| Proxy session forwarding | `proxy.rs` |
| Heartbeat timeout sweep | `watchdog.rs` — `heartbeat_watchdog()` |
| Runtime unit tests | `tests.rs` |

## CONVENTIONS

**Message dispatch**: `dispatch.rs` only routes — `route_agent_message()`, `route_task_message()`, `route_command_session_message()`, `route_proxy_message()`. No business logic in routing functions.

**Handler patterns**: Handlers receive `&Arc<RwLock<KernelState>>` + `&RuntimePorts`. Acquire write lock → call state methods → drop lock → trigger effects. State machines are in `kernel/state/`, not here.

**Effects isolation**: `RuntimePorts::new(events, storage)` wraps `EventPublisher` (broadcast::Sender<String>, JSON-serialized `WebEvent`) and `StatePersistence`. SQLite ops use `tokio::task::spawn_blocking` because `rusqlite::Connection` is not `Send`. Persistence is fire-and-forget via `tokio::spawn` inside `RuntimePorts` methods.

**Bootstrap sequence**: `new_kernel()` → create `Storage` → load persisted state → initialize `KernelState` → build `KernelHandle` → spawn `kernel_loop()` + `heartbeat_watchdog()`.

## ANTI-PATTERNS

- **Never** add business logic to `dispatch.rs` — it routes only, decisions belong in handlers.
- **Never** import or call `storage` directly from handlers — go through `RuntimePorts`.
- **Never** hold a `KernelState` write lock across an `.await` point that triggers I/O.
- **Never** add domain-specific state machines here — they belong in `kernel/state/`.
- **Never** bypass `RuntimePorts` for persistence or event publishing.

## NOTES

- **Cycles**: 5 detected, all within `runtime` ↔ `runtime/agent_lifecycle` — acceptable tight coupling.
- **Watchdog**: `heartbeat_watchdog()` fires `SweepHeartbeats` on a 1-second `tokio::time::interval`; timeout threshold is in agent config.
- **Clippy suppression**: `#[allow(clippy::too_many_arguments)]` on `handle_command_session_result()` (11 params) — only suppressed lint in the codebase.
- **Test patterns**: `tests.rs` uses `test_runtime_ports()` helper (temp SQLite + `RuntimePorts`), `test_db_path()` with `AtomicU64` + PID for parallel safety, direct `KernelState` construction (bypasses kernel startup), and mpsc/oneshot channels for agent communication (no real TCP).

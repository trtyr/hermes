---
timestamp: 2026-05-28T10:25:35Z
commit: 31ffbf4
---

# Kernel — Microkernel Control Plane

## OVERVIEW

The kernel is the heart of Hermes Server — 71 files implementing a strict microkernel architecture. External input flows through service facades as `KernelMessage` variants, dispatched by a central loop to runtime handlers that drive state transitions and trigger side effects (persistence, WebSocket broadcast).

## STRUCTURE

```
src/kernel/
├── message.rs            # 4-domain message enum + sub-messages
├── dispatch.rs           # `kernel_loop()` routes messages to runtime handlers
├── bus.rs                # Event bus for WebSocket broadcast
├── kernel.rs             # KernelHandle, RuntimePorts, bootstrap
├── service/              # Domain facades (26 files, 8 domains)
│   ├── agent_command/    # Agent lifecycle: disable, enable, disconnect, beacon config
│   ├── agent_query/      # Agent queries
│   ├── task/             # Task dispatch, broadcast, cancel, query
│   ├── command_session/  # Open/execute/close command sessions
│   ├── listener/         # Listener CRUD (command + query split)
│   ├── auth/             # Web login session management
│   ├── agent_build/      # Agent binary builds
│   └── proxy/            # Proxy session management
├── runtime/              # Domain runtime handlers (20 files)
│   ├── kernel_loop.rs    # Main event loop
│   ├── agent_lifecycle/  # Registration, connection, task/command reporting, beacon_config
│   ├── task_flow/        # Task dispatch, result handling, broadcast
│   ├── command_sessions/ # Open, execute, close
│   └── proxy/            # Proxy session handlers
├── state/                # In-memory authoritative state (7 files)
│   ├── kernel_state.rs   # `KernelState` with agent, task, command, proxy maps
│   ├── agent_session.rs  # Agent lifecycle state machine
│   ├── task_record.rs    # Parent/child task tree
│   ├── command_session.rs # Command session state machine
│   └── proxy_session.rs  # Proxy session state
└── storage/              # SQLite persistence (9 files)
    ├── schema.sql        # Tables: tasks, agents, audits, listeners, agent_builds, proxy_sessions
    ├── bootstrap/        # Cold-start loading from DB
    └── migrations/       # Schema migration helpers
```

## WHERE TO LOOK

| Concern | Location |
|---|---|
| Message dispatch routing | `dispatch.rs` → `route_agent_message()`, `route_task_message()`, `route_command_message()`, `route_proxy_message()` |
| Domain facades (entry points) | `service/` — each subfolder contains a `mod.rs` with the facade struct |
| Agent registration flow | `runtime/agent_lifecycle/` → handler for `AgentMessage::Registration` |
| Task dispatch flow | `runtime/task_flow/` → handler for `TaskMessage::DispatchTask` |
| Command session lifecycle | `runtime/command_sessions/` → open → execute → close |
| State machines | `state/` — `AgentSession`, `TaskRecord`, `CommandSessionRecord`, `ProxySessionRecord` |
| DB schema | `storage/schema.sql` |
| Cold-start restore | `storage/bootstrap/` |

## MESSAGE FLOW

```
API handler / gateway adapter
    │
    ▼
KernelHandle.facade_method()        # service/ facade
    │
    ▼
KernelMessage enum variant          # message.rs (Agent | Task | CommandSession | Proxy)
    │
    ▼
kernel_loop() dispatcher            # dispatch.rs
    │
    ▼
route_<domain>_message()            # runtime/ domain handler
    │
    ├── state mutation               # state/ → KernelState
    └── effects via RuntimePorts      # storage/ + bus/ (WebSocket broadcast)
```

## DEPENDENCY RULES

```
main → api/gateway/listeners → kernel/service → kernel/{message, state, storage}
kernel/runtime → kernel/state + effects → storage + event bus
```

**Forbidden reversals:**
- `state` depending on `api`
- `storage` depending on `runtime`
- `api` directly depending on `storage` for domain decisions

## ANTI-PATTERNS

- **Never** call `KernelState` methods directly from API handlers — always go through `KernelHandle` / service facades.
- **Never** import `storage` modules in `runtime` handlers directly — use `RuntimePorts` for side effects.
- **Never** scatter state machine logic in runtime handlers — state machines live in `state/`, runtime handlers drive transitions via state methods.
- **Never** add new domain logic to `dispatch.rs` — it only routes, never decides.

## CONVENTIONS

- **Facade pattern**: Each domain facade in `service/` wraps a `KernelHandle` sender. New features add facade methods + message variants + runtime handlers.
- **Message ownership**: `KernelMessage` carries `KernelHandle` as `reply_to` field; runtime handlers use it to send back responses or effects.
- **State singletons**: `KernelState` is held by the kernel loop, accessed via `Arc<Mutex<KernelState>>` in facades, passed to runtime handlers.

## NOTES

- 4 `KernelMessage` variants: `Agent`, `Task`, `CommandSession`, `Proxy`. Each maps to one domain handler file.
- Mermaid generated `KernelMessage` is defined in `message.rs`; sub-messages are domain-specific enums.
- `RuntimePorts` abstracts I/O: storage writes, event bus publish, agent message sending.
- Test helpers create temp SQLite databases per test — see `runtime/tests.rs`.

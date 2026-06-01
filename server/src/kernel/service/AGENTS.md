---
timestamp: 2026-06-01T03:18:25Z
commit: bee5081
---

# Kernel Service — Domain Facades

## OVERVIEW

The service layer is the facade boundary of the Hermes microkernel — 30 files, 208 symbols, degree=1481 (highest connectivity cluster). API handlers and gateway adapters call facade methods here; facades translate those calls into `KernelMessage` variants dispatched to the kernel loop, or read state directly via `Arc<RwLock<KernelState>>`.

## STRUCTURE

```
service/
├── mod.rs                  # Re-exports all facades
├── handle/                 # KernelHandle — the shared entry point
│   ├── types.rs            #   struct definition (bus, state, events, storage, auth, ID counters)
│   ├── audit.rs            #   audit logging helpers
│   ├── capabilities.rs     #   permission/capability checks (deg=51)
│   ├── ids.rs              #   ID generation (AtomicU64 counters)
│   ├── messaging.rs        #   send/reply helpers for KernelMessage
│   └── mod.rs              #   module wiring
├── agent_commands/         # AgentCommandFacade — disable, enable, disconnect, beacon config
├── agent_queries.rs        # AgentQueryFacade — agent state queries
├── tasks/                  # TaskFacade — dispatch, broadcast, cancel, query
│   ├── commands.rs         #   mutation methods
│   └── queries.rs          #   read-only methods
├── command_sessions/       # CommandSessionFacade — open/execute/close sessions
├── listener_commands.rs    # ListenerCommandFacade — listener CRUD
├── listener_queries.rs     # ListenerQueryFacade — listener queries
├── auth.rs                 # AuthFacade — web login sessions (uses AuthService, NOT KernelMessage)
├── agent_builds/           # AgentBuildFacade — agent binary builds
├── proxy/                  # ProxyFacade — proxy session management
├── vuln_alerting/          # Empty placeholder (future)
└── listener_tests.rs       # Listener integration tests
```

## WHERE TO LOOK

| Concern | Location |
|---|---|
| KernelHandle struct & fields | `handle/types.rs` — all fields `pub(in crate::kernel::service)` |
| ID generation prefixes | `handle/ids.rs` — `task-`, `cmdsess-`, `cmdreq-`, `agentreq-` |
| Capability checks | `handle/capabilities.rs` |
| Agent lifecycle commands | `agent_commands/` — `AgentCommandFacade` |
| Task dispatch & broadcast | `tasks/` — `TaskFacade` |
| Command session lifecycle | `command_sessions/` — `CommandSessionFacade` |
| Listener CRUD | `listener_commands.rs` + `listener_queries.rs` |
| Web auth sessions | `auth.rs` — `AuthFacade` (bypasses KernelMessage, uses `AuthService` directly) |
| Agent binary builds | `agent_builds/` — `AgentBuildFacade` |
| Proxy sessions | `proxy/` — `ProxyFacade` |
| Test helpers | `listener_tests.rs`, `command_sessions/tests.rs`, `proxy/tests.rs`, `agent_builds/tests.rs` |

## CONVENTIONS

- **Facade pattern**: Each facade holds a `KernelHandle` clone. Public methods are the stable API — API handlers never touch handle internals.
- **Handle visibility**: All `KernelHandle` fields are `pub(in crate::kernel::service)` — only service submodules can access them. Do not widen this visibility.
- **ID generation**: `AtomicU64::fetch_add(1, Relaxed)` in `handle/ids.rs`. Prefixed strings (`task-0`, `cmdsess-1`, etc.) for human-readable IDs.
- **Snapshot pattern**: Internal state objects (`AgentSession`, `TaskRecord`, etc.) expose `.snapshot()` returning a `Serialize`-only DTO. Facades call `.snapshot()` before sending data to API/event bus.
- **Async request-response**: Facades that need responses from the runtime use `oneshot::channel()` — the sender goes into a `HashMap<String, oneshot::Sender<...>>` in `KernelState`, the runtime replies through it.
- **AuthFacade exception**: `AuthFacade` uses `AuthService` (`Arc<std::sync::RwLock<AuthState>>`) directly, not the `KernelMessage` channel — auth state is isolated from the kernel dispatch loop.
- **Proxy mutex**: `ProxyFacade` uses `std::sync::Mutex<Vec<JoinHandle>>` for tracking spawned proxy tasks — not `tokio::sync::Mutex` (no `.await` needed).

## ANTI-PATTERNS

- **Never** access `KernelHandle` fields from outside `service/` — the `pub(in crate::kernel::service)` visibility exists for a reason.
- **Never** add facade logic in API handlers — call facade methods, don't replicate their logic.
- **Never** use `KernelMessage` for auth operations — `AuthFacade` has its own channel.
- **Never** use `tokio::sync::Mutex` for the proxy task tracker — `std::sync::Mutex` is sufficient and avoids holding a guard across `.await`.

## NOTES

- 5 test files co-located in service/: `listener_tests.rs`, `command_sessions/tests.rs`, `proxy/tests.rs`, `agent_builds/tests.rs`, `auth_tests.rs`.
- All tests use `test_kernel()` helper → `new_kernel()` with a temp SQLite database.
- Agent seeding in tests: `seed_connected_agent()` inserts `AgentSession` + `AgentIdentity` directly into `KernelState`.
- `vuln_alerting/` is an empty placeholder — no implementation yet.
- `KernelHandle` is `Clone` (all fields are `Arc`-wrapped) — safe to pass to spawned tasks.

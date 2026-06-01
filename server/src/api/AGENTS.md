---
timestamp: 2026-06-01T03:18:25Z
commit: bee5081
---

# Server API — HTTP Route Handlers

## OVERVIEW

54 HTTP route handlers across 11 sub-APIs, mounted on axum 0.8. Each handler follows a strict pattern: extract request → authorize → validate → call `KernelHandle` facade → return response. Never reaches into kernel state or storage directly.

## STRUCTURE

```
src/api/
├── mod.rs                  # Router assembly, AppState, CORS, WebSocket upgrade
├── auth/                   # Login/logout/status (3 routes)
├── agents/                 # Agent CRUD + context menu actions (8 routes)
├── listeners/              # Listener CRUD (5 routes)
├── tasks/                  # Task dispatch, broadcast, cancel, query (6 routes)
├── command_sessions/       # Command session lifecycle (6 routes)
├── agent_builds/           # Agent binary builds (5 routes)
├── web_terminal/           # WebSocket terminal proxy (2 routes)
├── dashboard/              # Dashboard aggregation data
├── audits/                 # Audit log queries with paging
├── proxy/                  # Proxy session management
├── settings/               # Server settings read/write
├── system/                 # System info, health checks
├── common.rs               # AppState, authorize_api middleware, paging helpers
└── *.rs                    # Per-domain request/response DTOs
```

## WHERE TO LOOK

| Concern | Location |
|---|---|
| Router assembly & middleware | `mod.rs` → `build_api_router()` |
| Auth middleware | `common.rs` → `authorize_api()` — checks session cookie, `x-api-token`, Bearer token |
| AppState | `common.rs` → `AppState(KernelHandle)` — the only kernel access point for handlers |
| Request/response types | `common.rs` (shared), then `agents.rs`, `tasks.rs`, etc. (domain-specific) |
| Paging | `common.rs` → `PagingParams`, `PageResult<T>` |
| WebSocket terminal | `web_terminal/` — upgrades to WS, proxies I/O to agent via `CommandSessionFacade` |

## CONVENTIONS

### Handler Signature

```rust
#[axum::debug_handler]
async fn handler_name(
    State(state): State<AppState>,
    /* Extractors: Path<T>, Query<T>, Json<T>, headers, etc. */
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)>
```

### Auth Flow

Every protected handler calls `authorize_api()` early:
1. Check session cookie → resolve web user
2. Fallback: check `x-api-token` header (legacy)
3. Fallback: check `Authorization: Bearer <token>` (agent tokens)

### DTO Pattern

Request/response types are defined adjacent to their handler in the same file:
- `CreateListenerRequest`, `ListenerResponse`
- `DispatchTaskRequest`, `TaskSummary`
- `PagingParams` (offset, page_size → validated to > 0, ≤ 500)

### Error Responses

```json
{ "message": "human-readable error", "error_code": "optional_machine_readable" }
```
Returned with appropriate HTTP status codes (400, 401, 404, 409, 500).

## ANTI-PATTERNS

- **Never** import `kernel::state` or `kernel::storage` modules in API handlers. All kernel access goes through `KernelHandle` facade methods.
- **Never** make business decisions in handlers — call the facade and return its result. Handlers are thin translation layers.
- **Never** use blocking I/O in async handlers — use `tokio::task::spawn_blocking` for CPU-heavy or synchronous DB operations if needed.

## NOTES

- 11 sub-APIs, each in its own `mod.rs` file or standalone `.rs` file.
- The API router is assembled in `src/api/mod.rs` via `axum::Router::nest()`.
- CORS is wide-open (`Any` origin) — by design for development convenience.
- WebSocket routes use the same auth middleware before upgrading the connection.
- `Makefile` targets for Hurl API testing exist in the project (check scripts). Hurl is a .hurl file-based API testing tool.

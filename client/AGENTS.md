---
timestamp: 2026-05-28T10:25:35Z
commit: 31ffbf4
---

# Web Client — Browser Operations UI

## OVERVIEW

Vue 3 + TypeScript + Vite SPA for operating the Hermes C2 platform. Connects to the server HTTP API for CRUD operations and subscribes to WebSocket events for real-time updates. Based on vue-vben-admin template. 50 source files, no test files.

## STRUCTURE

```
client/
├── src/
│   ├── main.ts              # App bootstrap, plugin registration
│   ├── App.vue              # Root component
│   ├── router/              # Vue Router config (7 routes)
│   │   └── index.ts
│   ├── api/                 # HTTP API client modules (10 modules)
│   │   ├── request.ts       # Base fetch wrapper (auth headers, error handling)
│   │   ├── agent.ts         # Agent CRUD API
│   │   ├── agentBuild.ts    # Agent binary build API
│   │   ├── audit.ts         # Audit log API
│   │   ├── connection.ts    # Connection profile CRUD API
│   │   ├── dashboard.ts     # Dashboard aggregation API
│   │   ├── listener.ts      # Listener CRUD API
│   │   ├── proxy.ts         # Proxy session API
│   │   ├── settings.ts      # Server settings API
│   │   └── terminal.ts      # Terminal session API
│   ├── store/               # Pinia stores (4 stores)
│   │   ├── events.ts        # WebSocket pub/sub with auto-reconnect
│   │   ├── connection.ts    # Connection profiles (persisted to localStorage)
│   │   ├── app.ts           # Sidebar state, tab management
│   │   └── notifications.ts # Toast notification queue
│   ├── views/               # Page components (7 routes)
│   │   ├── login/           # /login
│   │   ├── dashboard/       # /dashboard (TopStatsGrid, ServerInfoCard, dist charts)
│   │   ├── agent/           # /agent (table, detail drawer, task modal, file ops modal)
│   │   ├── listener/        # /listener (listener management)
│   │   ├── payload/         # /payload (agent binary build/payload generation)
│   │   ├── log/             # /log (audit log viewer)
│   │   └── session/         # /agent/:id/session (interactive terminal + file browser)
│   ├── composables/         # Reusable composition functions
│   │   ├── useTerminal.ts       # Terminal orchestrator (uses core + socket + history)
│   │   ├── useTerminalCore.ts   # xterm.js init and configuration
│   │   ├── useTerminalSocket.ts # WebSocket I/O for terminal sessions
│   │   └── useTerminalHistory.ts# Command history with up/down arrow navigation
│   ├── layouts/             # Layout components (sidebar, header)
│   └── types/               # TypeScript type definitions
├── index.html
├── vite.config.ts           # Vite config with @ path alias
├── package.json
└── tsconfig.json
```

## WHERE TO LOOK

| Concern | Location |
|---|---|
| API client foundation | `api/request.ts` — `get()`, `post()`, `put()`, `del()` helpers with auth header injection |
| WebSocket events | `store/events.ts` — connects to WS endpoint, deserializes events, publishes to subscribers |
| Connection profiles | `store/connection.ts` — CRUD for saved server profiles in localStorage |
| HTTP API modules | `api/` — one module per server API domain, exports typed functions |
| Page routes | `views/` — one folder per route, `views/login/LoginView.vue`, etc. |
| Dashboard | `views/dashboard/` — `TopStatsGrid.vue`, `ServerInfoCard.vue`, `AgentsDistCard.vue`, `ListenersDistCard.vue` |
| Agent management | `views/agent/` — table with context menu, detail drawer (sysinfo tabs), task modal, file ops modal |
| Interactive terminal | `views/session/` — uses `useTerminal` composable chain |
| Router | `router/index.ts` — lazy-loaded routes, circular dependency workarounds |

## COMMANDS

```bash
npm run dev        # Vite dev server (port 5173)
npm run build      # vue-tsc type check + vite build
```

## EVENTS STORE PATTERN

The `events` store provides a publish/subscribe pattern over WebSocket:

```
Server WS event → events store deserializes → publishes to subscribers by event type
  ├── agent_connected → agent table view updates
  ├── agent_disconnected → agent table view updates
  ├── task_completed → task list updates
  ├── command_session_output → terminal composable receives
  └── ...
```

Auto-reconnect with exponential backoff is built in.

## ANTI-PATTERNS

- **Never** add new xterm packages — the deprecated `xterm` and `xterm-addon-fit` packages must be migrated to `@xterm/xterm` and `@xterm/addon-fit`. All new terminal dependencies use the `@xterm/*` namespace.
- **Never** break the lazy import pattern for routes — the router uses dynamic `import()` to work around circular dependency issues. Keep this pattern when adding new routes.
- **Never** add centralized type definitions — types are defined inline in each API module. Maintain this convention for consistency.

## NOTES

- Based on `vue-vben-admin` template — follows its project conventions for layouts, stores, and composables.
- `@` path alias resolves to `client/src/` (configured in `vite.config.ts` and `tsconfig.json`).
- Connection profiles are persisted in `localStorage` — server URL, username, password.
- No test files exist (0 client tests) — manual testing only.
- The terminal chain (`useTerminal` → `useTerminalCore` → `useTerminalSocket` → `useTerminalHistory`) is the most complex client subsystem; understand all 4 composables before modifying any of them.
- Route names map 1:1 to view folders under `views/`.
- All API calls go through `api/request.ts` helpers to ensure consistent auth header injection and error handling.

// API root keeps router composition and cross-domain shared helpers.
// Domain handlers live in `agents`, `tasks`, and `command_sessions`.
mod agent_builds;
mod agents;
mod audits;
mod auth;
mod command_sessions;
mod common;
mod dashboard;
mod listeners;
mod settings;
mod system;
mod tasks;
mod web_terminal;

use axum::{
    Json, Router,
    extract::Request,
    http::{HeaderMap, Method, StatusCode, header},
    middleware::{self, Next},
    response::IntoResponse,
    routing::{get, patch, post},
};
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};

use anyhow::Context as _;
use crate::console;
use crate::kernel::KernelHandle;
use common::*;

pub fn build_router(kernel: KernelHandle) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::HeaderName::from_static("x-api-token"),
            header::HeaderName::from_static("x-session-token"),
            header::HeaderName::from_static("x-operator"),
        ]);

    Router::new()
        .route("/health", get(system::health))
        .route("/auth/login", post(auth::login))
        .route("/auth/logout", post(auth::logout))
        .route("/auth/me", get(auth::me))
        .route(
            "/server/auth-settings",
            get(settings::get_auth_settings).put(settings::update_auth_settings),
        )
        .route("/dashboard/stats", get(dashboard::dashboard_stats))
        .route(
            "/agent-builds",
            get(agent_builds::list_agent_builds).post(agent_builds::create_agent_build),
        )
        .route(
            "/agent-builds/{build_id}",
            get(agent_builds::get_agent_build).delete(agent_builds::delete_agent_build),
        )
        .route(
            "/agent-builds/{build_id}/download",
            get(agent_builds::download_agent_build),
        )
        .route("/openapi.yaml", get(system::openapi_spec))
        .route("/docs", get(system::api_docs))
        .route("/agents", get(agents::list_agents))
        .route("/agents/history", get(agents::list_persisted_agents))
        .route("/agents/{agent_id}", patch(agents::update_agent))
        .route("/agents/{agent_id}/enable", post(agents::enable_agent))
        .route("/agents/{agent_id}/disable", post(agents::disable_agent))
        .route(
            "/agents/{agent_id}/beacon-config",
            post(agents::update_agent_beacon_config),
        )
        .route(
            "/agents/{agent_id}/command-sessions",
            post(command_sessions::create_command_session),
        )
        .route(
            "/agents/{agent_id}/disconnect",
            post(agents::disconnect_agent),
        )
        .route("/agents/{agent_id}/tasks", post(agents::dispatch_task))
        .route("/agents/{agent_id}/upload", post(agents::upload_file))
        .route("/agents/{agent_id}/download", post(agents::download_file))
        .route("/agents/{agent_id}/browse", post(agents::browse_file))
        .route(
            "/agents/{agent_id}/screenshot",
            post(agents::take_screenshot),
        )
        .route(
            "/agents/{agent_id}/proxy",
            get(agents::list_proxy).post(agents::start_proxy),
        )
        .route(
            "/agents/{agent_id}/proxy/{proxy_id}",
            axum::routing::delete(agents::delete_proxy),
        )
        .route(
            "/listeners",
            get(listeners::list_listeners).post(listeners::create_listener),
        )
        .route(
            "/listeners/{listener_id}",
            get(listeners::get_listener)
                .patch(listeners::update_listener)
                .delete(listeners::delete_listener),
        )
        .route(
            "/listeners/{listener_id}/agent-builds",
            post(listeners::create_listener_agent_build),
        )
        .route(
            "/listeners/{listener_id}/enable",
            post(listeners::enable_listener),
        )
        .route(
            "/listeners/{listener_id}/disable",
            post(listeners::disable_listener),
        )
        .route(
            "/agents/{agent_id}",
            get(agents::get_agent).delete(agents::delete_agent),
        )
        .route(
            "/command-sessions",
            get(command_sessions::list_command_sessions),
        )
        .route(
            "/command-sessions/{command_session_id}",
            get(command_sessions::get_command_session),
        )
        .route(
            "/command-sessions/{command_session_id}/commands",
            get(command_sessions::list_command_executions)
                .post(command_sessions::queue_command_execution),
        )
        .route(
            "/command-sessions/{command_session_id}/commands/{command_id}",
            get(command_sessions::get_command_execution),
        )
        .route(
            "/command-sessions/{command_session_id}/execute",
            post(command_sessions::execute_command_session),
        )
        .route(
            "/command-sessions/{command_session_id}/close",
            post(command_sessions::close_command_session),
        )
        .route("/tasks", get(tasks::list_tasks))
        .route(
            "/tasks/{task_id}",
            get(tasks::get_task).delete(tasks::cancel_task),
        )
        .route("/tasks/broadcast", post(tasks::broadcast_task))
        .route("/audits", get(audits::list_audits).delete(audits::clear_audits))
        .route("/events/ws", get(system::ws_events))
        .route(
            "/web/terminal/open",
            post(web_terminal::open_terminal_session),
        )
        .route(
            "/web/terminal/session/{session_id}",
            get(web_terminal::get_terminal_session),
        )
        .route(
            "/web/terminal/command",
            post(web_terminal::queue_terminal_command),
        )
        .route(
            "/web/terminal/close",
            post(web_terminal::close_terminal_session),
        )
        .route("/web/terminal/ws", get(web_terminal::terminal_ws_events))
        .layer(middleware::from_fn(log_http_request))
        .layer(cors)
        .with_state(AppState { kernel })
}

pub async fn run_http_api(
    kernel: KernelHandle,
    api_addr: (std::net::Ipv4Addr, u16),
) -> anyhow::Result<(), anyhow::Error> {
    let app = build_router(kernel);

    let listener = tokio::net::TcpListener::bind(api_addr)
        .await
        .with_context(|| format!("HTTP API 无法绑定到 {}:{}", api_addr.0, api_addr.1))?;
    console::startup_http_api(listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn log_http_request(request: Request, next: Next) -> impl IntoResponse {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let started = std::time::Instant::now();

    let response = next.run(request).await;
    let status = response.status();
    let elapsed_ms = started.elapsed().as_millis();

    console::http_request(&method, &uri, status, elapsed_ms);

    response
}

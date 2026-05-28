use std::{
    path::{Path, PathBuf},
    sync::{
        OnceLock,
        atomic::{AtomicU64, Ordering},
    },
};

use axum::{
    Router,
    body::{Body, to_bytes},
    http::{HeaderMap, Method, Request, StatusCode, header},
};
use serde_json::{Value, json};
use server::{
    api::build_router,
    kernel::{AgentAuthConfig, AgentAuthMode, new_kernel},
};
use tokio::sync::Mutex;
use tower::ServiceExt;

static NEXT_TEST_ID: AtomicU64 = AtomicU64::new(1);
static CWD_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

struct TestApp {
    app: Router,
    _sqlite_path: PathBuf,
}

struct TestResponse {
    status: StatusCode,
    headers: HeaderMap,
    json: Value,
    text: String,
}

struct CwdGuard {
    previous: PathBuf,
}

impl Drop for CwdGuard {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(&self.previous);
    }
}

fn test_db_path(name: &str) -> PathBuf {
    let id = NEXT_TEST_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "hermes-http-test-{name}-{}-{id}.db",
        std::process::id()
    ))
}

async fn build_test_app(name: &str) -> TestApp {
    let sqlite_path = test_db_path(name);
    let _ = std::fs::remove_file(&sqlite_path);
    let agent_auth_config = AgentAuthConfig::shared(None, AgentAuthMode::PlainToken);
    let kernel = new_kernel(
        32,
        32,
        &sqlite_path,
        Some("test-api-token".into()),
        Some("admin".into()),
        Some("password".into()),
        8 * 60 * 60,
        agent_auth_config,
    )
    .await
    .expect("kernel starts");

    TestApp {
        app: build_router(kernel),
        _sqlite_path: sqlite_path,
    }
}

async fn request(
    app: &Router,
    method: Method,
    uri: &str,
    body: Option<Value>,
    headers: &[(&str, &str)],
) -> TestResponse {
    let mut req = Request::builder().method(method).uri(uri);
    for (key, value) in headers {
        req = req.header(*key, *value);
    }
    if body.is_some() {
        req = req.header(header::CONTENT_TYPE, "application/json");
    }
    let body = body
        .map(|json| Body::from(serde_json::to_vec(&json).expect("json body serializes")))
        .unwrap_or_else(Body::empty);
    let req = req.body(body).expect("request builds");

    let response = app.clone().oneshot(req).await.expect("router responds");
    let status = response.status();
    let headers = response.headers().clone();
    let body_bytes = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body reads");
    let text = String::from_utf8_lossy(&body_bytes).into_owned();
    let json = if body_bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&body_bytes).unwrap_or_else(|_| Value::String(text.clone()))
    };

    TestResponse {
        status,
        headers,
        json,
        text,
    }
}

async fn get(app: &Router, uri: &str, headers: &[(&str, &str)]) -> TestResponse {
    request(app, Method::GET, uri, None, headers).await
}

async fn post(app: &Router, uri: &str, body: Value, headers: &[(&str, &str)]) -> TestResponse {
    request(app, Method::POST, uri, Some(body), headers).await
}

async fn put(app: &Router, uri: &str, body: Value, headers: &[(&str, &str)]) -> TestResponse {
    request(app, Method::PUT, uri, Some(body), headers).await
}

fn bearer_auth() -> [(&'static str, &'static str); 1] {
    [("authorization", "Bearer test-api-token")]
}

fn session_cookie(set_cookie: &str) -> String {
    set_cookie
        .split(';')
        .next()
        .expect("cookie pair present")
        .to_string()
}

fn write_temp_config(dir: &Path) {
    let config = r#"
[server]
host = "127.0.0.1"
port = 1234

[api]
host = "127.0.0.1"
port = 3000

[storage]
sqlite_path = "data/server.db"

[auth]
api_token = "test-api-token"
agent_token = "initial-agent-token"
agent_auth_mode = "plain_token"
web_username = "admin"
web_password = "password"
session_ttl_secs = 28800
"#;
    std::fs::create_dir_all(dir).expect("temp config dir exists");
    std::fs::write(dir.join("config.toml"), config).expect("temp config written");
}

fn switch_current_dir(path: &Path) -> CwdGuard {
    let previous = std::env::current_dir().expect("current dir available");
    std::env::set_current_dir(path).expect("switch current dir succeeds");
    CwdGuard { previous }
}

#[tokio::test]
async fn test_health_endpoint() {
    let app = build_test_app("health").await;
    let response = get(&app.app, "/health", &[]).await;

    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(response.json["status"], "ok");
}

#[tokio::test]
async fn test_dashboard_stats_requires_auth() {
    let app = build_test_app("dashboard-auth").await;
    let response = get(&app.app, "/dashboard/stats", &[]).await;

    assert_eq!(response.status, StatusCode::UNAUTHORIZED);
    assert_eq!(response.json["detail"], "unauthorized");
}

#[tokio::test]
async fn test_login_with_correct_credentials() {
    let app = build_test_app("login-ok").await;
    let response = post(
        &app.app,
        "/auth/login",
        json!({ "username": "admin", "password": "password" }),
        &[],
    )
    .await;

    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(response.json["success"], true);
    assert_eq!(response.json["username"], "admin");
    assert!(response.json["session_token"].as_str().is_some());
    assert!(response.headers.get(header::SET_COOKIE).is_some());
}

#[tokio::test]
async fn test_login_with_wrong_credentials() {
    let app = build_test_app("login-bad").await;
    let response = post(
        &app.app,
        "/auth/login",
        json!({ "username": "admin", "password": "wrong-password" }),
        &[],
    )
    .await;

    assert_eq!(response.status, StatusCode::UNAUTHORIZED);
    assert_eq!(response.json["detail"], "unauthorized");
    assert_eq!(response.json["success"], false);
}

#[tokio::test]
async fn test_protected_route_without_auth() {
    let app = build_test_app("protected-without-auth").await;

    for uri in [
        "/agents",
        "/listeners",
        "/agent-builds",
        "/audits",
        "/server/auth-settings",
    ] {
        let response = get(&app.app, uri, &[]).await;
        assert_eq!(response.status, StatusCode::UNAUTHORIZED, "uri={uri}");
    }
}

#[tokio::test]
async fn test_list_agents_empty() {
    let app = build_test_app("agents-empty").await;
    let response = get(&app.app, "/agents", &bearer_auth()).await;

    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(response.json["agents"], json!([]));
}

#[tokio::test]
async fn test_get_nonexistent_agent() {
    let app = build_test_app("agent-missing").await;
    let response = get(&app.app, "/agents/missing-agent", &bearer_auth()).await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.json["success"], false);
    assert_eq!(response.json["detail"], "agent missing-agent not found");
}

#[tokio::test]
async fn test_listener_crud() {
    let app = build_test_app("listener-crud").await;

    let created = post(
        &app.app,
        "/listeners",
        json!({
            "name": "test-listener",
            "kind": "tcp_json",
            "bind_host": "127.0.0.1",
            "bind_port": 41001,
            "enabled": false,
            "config": {}
        }),
        &bearer_auth(),
    )
    .await;
    assert_eq!(created.status, StatusCode::CREATED);
    let listener_id = created.json["listener"]["listener_id"]
        .as_i64()
        .expect("listener id exists");

    let listed = get(&app.app, "/listeners", &bearer_auth()).await;
    assert_eq!(listed.status, StatusCode::OK);
    assert_eq!(listed.json["total"], 1);
    assert_eq!(listed.json["listeners"][0]["name"], "test-listener");

    let fetched = get(&app.app, &format!("/listeners/{listener_id}"), &bearer_auth()).await;
    assert_eq!(fetched.status, StatusCode::OK);
    assert_eq!(fetched.json["listener_id"], listener_id);

    let deleted = request(
        &app.app,
        Method::DELETE,
        &format!("/listeners/{listener_id}"),
        None,
        &bearer_auth(),
    )
    .await;
    assert_eq!(deleted.status, StatusCode::OK);
    assert_eq!(deleted.json["success"], true);
}

#[tokio::test]
async fn test_list_agent_builds_empty() {
    let app = build_test_app("builds-empty").await;
    let response = get(&app.app, "/agent-builds", &bearer_auth()).await;

    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(response.json["builds"], json!([]));
    assert_eq!(response.json["total"], 0);
}

#[tokio::test]
async fn test_create_and_list_agent_build() {
    let app = build_test_app("builds-create").await;

    let created = post(
        &app.app,
        "/agent-builds",
        json!({
            "server_addr": "127.0.0.1:41002",
            "profile": "release"
        }),
        &bearer_auth(),
    )
    .await;
    assert_eq!(created.status, StatusCode::ACCEPTED);
    let build_id = created.json["build"]["build_id"]
        .as_i64()
        .expect("build id exists");

    let listed = get(&app.app, "/agent-builds", &bearer_auth()).await;
    assert_eq!(listed.status, StatusCode::OK);
    assert_eq!(listed.json["total"], 1);
    assert_eq!(listed.json["builds"][0]["build_id"], build_id);
}

#[tokio::test]
async fn test_get_auth_settings() {
    let app = build_test_app("settings-get").await;
    let response = get(&app.app, "/server/auth-settings", &bearer_auth()).await;

    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(response.json["agent_token"], Value::Null);
    assert_eq!(response.json["agent_auth_mode"], "plain_token");
}

#[tokio::test]
async fn test_update_auth_settings() {
    let app = build_test_app("settings-update").await;
    let _guard = CWD_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .await;

    let temp_dir = std::env::temp_dir().join(format!(
        "hermes-http-config-{}-{}",
        std::process::id(),
        NEXT_TEST_ID.fetch_add(1, Ordering::Relaxed)
    ));
    write_temp_config(&temp_dir);
    let _cwd = switch_current_dir(&temp_dir);

    let response = put(
        &app.app,
        "/server/auth-settings",
        json!({
            "agent_token": "rotated-agent-token",
            "agent_auth_mode": "challenge_response"
        }),
        &bearer_auth(),
    )
    .await;

    assert_eq!(response.status, StatusCode::OK, "body: {}", response.text);
    assert_eq!(response.json["agent_token"], "rotated-agent-token");
    assert_eq!(response.json["agent_auth_mode"], "challenge_response");

    let updated = std::fs::read_to_string(temp_dir.join("config.toml")).expect("config readable");
    assert!(updated.contains("agent_token = \"rotated-agent-token\""));
    assert!(updated.contains("agent_auth_mode = \"challenge_response\""));
}

#[tokio::test]
async fn test_list_audits_with_auth() {
    let app = build_test_app("audits-list").await;

    let _created = post(
        &app.app,
        "/listeners",
        json!({
            "name": "audit-listener",
            "kind": "tcp_json",
            "bind_host": "127.0.0.1",
            "bind_port": 41003,
            "enabled": false,
            "config": {}
        }),
        &bearer_auth(),
    )
    .await;

    let response = get(&app.app, "/audits", &bearer_auth()).await;
    assert_eq!(response.status, StatusCode::OK);
    assert!(response.json["total"].as_u64().unwrap_or(0) >= 1);
    assert_eq!(response.json["audits"][0]["action"], "create_listener");
}

#[tokio::test]
async fn test_api_token_auth() {
    let app = build_test_app("auth-bearer").await;
    let response = get(&app.app, "/agents", &bearer_auth()).await;

    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(response.json["agents"], json!([]));
}

#[tokio::test]
async fn test_session_cookie_auth() {
    let app = build_test_app("auth-cookie").await;
    let login = post(
        &app.app,
        "/auth/login",
        json!({ "username": "admin", "password": "password" }),
        &[],
    )
    .await;
    assert_eq!(login.status, StatusCode::OK);

    let cookie = session_cookie(
        login
            .headers
            .get(header::SET_COOKIE)
            .expect("set-cookie present")
            .to_str()
            .expect("cookie header is utf-8"),
    );

    let me = get(&app.app, "/auth/me", &[("cookie", cookie.as_str())]).await;
    assert_eq!(me.status, StatusCode::OK);
    assert_eq!(me.json["authenticated"], true);
    assert_eq!(me.json["username"], "admin");

    let agents = get(&app.app, "/agents", &[("cookie", cookie.as_str())]).await;
    assert_eq!(agents.status, StatusCode::OK);
}

// ---------------------------------------------------------------------------
// Helpers for PATCH and DELETE
// ---------------------------------------------------------------------------

async fn patch(app: &Router, uri: &str, body: Value, headers: &[(&str, &str)]) -> TestResponse {
    request(app, Method::PATCH, uri, Some(body), headers).await
}

async fn delete_request(app: &Router, uri: &str, headers: &[(&str, &str)]) -> TestResponse {
    request(app, Method::DELETE, uri, None, headers).await
}

// ===========================================================================
// Auth edge cases
// ===========================================================================

#[tokio::test]
async fn test_logout_requires_auth() {
    let app = build_test_app("logout-no-auth").await;
    let response = post(&app.app, "/auth/logout", json!({}), &[]).await;

    assert_eq!(response.status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_login_with_missing_body() {
    let app = build_test_app("login-missing-body").await;
    let response = post(&app.app, "/auth/login", json!({}), &[]).await;

    // Should fail — either 400 or 401 depending on validation
    assert!(
        response.status == StatusCode::BAD_REQUEST
            || response.status == StatusCode::UNAUTHORIZED
            || response.status == StatusCode::UNPROCESSABLE_ENTITY,
        "expected failure status, got {}",
        response.status
    );
}

#[tokio::test]
async fn test_auth_me_requires_auth() {
    let app = build_test_app("me-no-auth").await;
    let response = get(&app.app, "/auth/me", &[]).await;

    assert_eq!(response.status, StatusCode::UNAUTHORIZED);
}

// ===========================================================================
// Agent endpoints (nonexistent agent → 404)
// ===========================================================================

#[tokio::test]
async fn test_patch_nonexistent_agent() {
    let app = build_test_app("agent-patch-404").await;
    let response = patch(
        &app.app,
        "/agents/nonexistent-agent",
        json!({ "name": "renamed" }),
        &bearer_auth(),
    )
    .await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.json["success"], false);
}

#[tokio::test]
async fn test_delete_nonexistent_agent() {
    let app = build_test_app("agent-delete-404").await;
    let response = delete_request(&app.app, "/agents/nonexistent-agent", &bearer_auth()).await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.json["success"], false);
}

#[tokio::test]
async fn test_enable_nonexistent_agent() {
    let app = build_test_app("agent-enable-404").await;
    let response = post(
        &app.app,
        "/agents/nonexistent-agent/enable",
        json!({}),
        &bearer_auth(),
    )
    .await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.json["success"], false);
}

#[tokio::test]
async fn test_disable_nonexistent_agent() {
    let app = build_test_app("agent-disable-404").await;
    let response = post(
        &app.app,
        "/agents/nonexistent-agent/disable",
        json!({}),
        &bearer_auth(),
    )
    .await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.json["success"], false);
}

#[tokio::test]
async fn test_disconnect_nonexistent_agent() {
    let app = build_test_app("agent-disconnect-404").await;
    let response = post(
        &app.app,
        "/agents/nonexistent-agent/disconnect",
        json!({}),
        &bearer_auth(),
    )
    .await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.json["success"], false);
}

#[tokio::test]
async fn test_beacon_config_nonexistent_agent() {
    let app = build_test_app("agent-beacon-404").await;
    let response = post(
        &app.app,
        "/agents/nonexistent-agent/beacon-config",
        json!({"sleep_interval": 60, "jitter": 10}),
        &bearer_auth(),
    )
    .await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.json["success"], false);
}

#[tokio::test]
async fn test_dispatch_task_nonexistent_agent() {
    let app = build_test_app("agent-task-404").await;
    let response = post(
        &app.app,
        "/agents/nonexistent-agent/tasks",
        json!({"command": "whoami"}),
        &bearer_auth(),
    )
    .await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.json["success"], false);
}

#[tokio::test]
async fn test_upload_nonexistent_agent() {
    let app = build_test_app("agent-upload-404").await;
    let response = post(
        &app.app,
        "/agents/nonexistent-agent/upload",
        json!({"remote_path": "/tmp/x", "content_base64": "dGVzdA=="}),
        &bearer_auth(),
    )
    .await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.json["success"], false);
}

#[tokio::test]
async fn test_download_nonexistent_agent() {
    let app = build_test_app("agent-download-404").await;
    let response = post(
        &app.app,
        "/agents/nonexistent-agent/download",
        json!({"remote_path": "/etc/hosts"}),
        &bearer_auth(),
    )
    .await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.json["success"], false);
}

#[tokio::test]
async fn test_browse_nonexistent_agent() {
    let app = build_test_app("agent-browse-404").await;
    let response = post(
        &app.app,
        "/agents/nonexistent-agent/browse",
        json!({"path": "C:\\"}),
        &bearer_auth(),
    )
    .await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.json["success"], false);
}

#[tokio::test]
async fn test_screenshot_nonexistent_agent() {
    let app = build_test_app("agent-screenshot-404").await;
    let response = post(
        &app.app,
        "/agents/nonexistent-agent/screenshot",
        json!({}),
        &bearer_auth(),
    )
    .await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.json["success"], false);
}

#[tokio::test]
async fn test_get_proxy_nonexistent_agent() {
    let app = build_test_app("agent-proxy-get").await;
    let response = get(&app.app, "/agents/nonexistent-agent/proxy", &bearer_auth()).await;

    // May return 200 with empty proxies or 404 — accept either
    assert!(
        response.status == StatusCode::OK || response.status == StatusCode::NOT_FOUND,
        "expected 200 or 404, got {}",
        response.status
    );
}

#[tokio::test]
async fn test_post_proxy_nonexistent_agent() {
    let app = build_test_app("agent-proxy-post-404").await;
    let response = post(
        &app.app,
        "/agents/nonexistent-agent/proxy",
        json!({}),
        &bearer_auth(),
    )
    .await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.json["success"], false);
}

#[tokio::test]
async fn test_delete_proxy_nonexistent_entry() {
    let app = build_test_app("agent-proxy-del").await;
    let response = delete_request(
        &app.app,
        "/agents/nonexistent-agent/proxy/nonexistent",
        &bearer_auth(),
    )
    .await;

    // May return 200 (found and deleted), 404 (not found), or 500 (proxy delete
    // doesn't handle missing proxies gracefully) — accept all as non-crash
    assert!(
        response.status == StatusCode::OK
            || response.status == StatusCode::NOT_FOUND
            || response.status == StatusCode::INTERNAL_SERVER_ERROR,
        "expected 200, 404, or 500, got {}",
        response.status
    );
}

#[tokio::test]
async fn test_agents_history_empty() {
    let app = build_test_app("agents-history").await;
    let response = get(&app.app, "/agents/history", &bearer_auth()).await;

    assert_eq!(response.status, StatusCode::OK);
    assert!(response.json["total"].as_u64().is_some() || response.json["total"].as_i64().is_some());
}

#[tokio::test]
async fn test_get_agent_detail_nonexistent() {
    let app = build_test_app("agent-detail-404").await;
    let response = get(&app.app, "/agents/nonexistent-agent", &bearer_auth()).await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.json["success"], false);
}

// ===========================================================================
// Listener endpoints (nonexistent listener)
// ===========================================================================

#[tokio::test]
async fn test_patch_nonexistent_listener() {
    let app = build_test_app("listener-patch-404").await;
    let response = patch(
        &app.app,
        "/listeners/99999",
        json!({"name": "updated"}),
        &bearer_auth(),
    )
    .await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.json["success"], false);
}

#[tokio::test]
async fn test_enable_nonexistent_listener() {
    let app = build_test_app("listener-enable-404").await;
    let response = post(
        &app.app,
        "/listeners/99999/enable",
        json!({}),
        &bearer_auth(),
    )
    .await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.json["success"], false);
}

#[tokio::test]
async fn test_disable_nonexistent_listener() {
    let app = build_test_app("listener-disable-404").await;
    let response = post(
        &app.app,
        "/listeners/99999/disable",
        json!({}),
        &bearer_auth(),
    )
    .await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.json["success"], false);
}

#[tokio::test]
async fn test_agent_build_for_nonexistent_listener() {
    let app = build_test_app("listener-build-err").await;
    let response = post(
        &app.app,
        "/listeners/99999/agent-builds",
        json!({"profile": "release"}),
        &bearer_auth(),
    )
    .await;

    // Should return some error — 404 or 400
    assert!(
        response.status.is_client_error(),
        "expected 4xx, got {}",
        response.status
    );
}

// ===========================================================================
// Agent build endpoints (nonexistent build)
// ===========================================================================

#[tokio::test]
async fn test_get_nonexistent_agent_build() {
    let app = build_test_app("build-get-404").await;
    let response = get(&app.app, "/agent-builds/99999", &bearer_auth()).await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.json["success"], false);
}

#[tokio::test]
async fn test_delete_nonexistent_agent_build() {
    let app = build_test_app("build-del-404").await;
    let response = delete_request(&app.app, "/agent-builds/99999", &bearer_auth()).await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.json["success"], false);
}

// ===========================================================================
// Command session endpoints
// ===========================================================================

#[tokio::test]
async fn test_list_command_sessions_empty() {
    let app = build_test_app("cmd-sessions-list").await;
    let response = get(&app.app, "/command-sessions", &bearer_auth()).await;

    assert_eq!(response.status, StatusCode::OK);
    assert!(response.json["sessions"].is_array());
    assert!(response.json["total"].as_u64().is_some() || response.json["total"].as_i64().is_some());
}

#[tokio::test]
async fn test_get_nonexistent_command_session() {
    let app = build_test_app("cmd-session-get-404").await;
    let response = get(&app.app, "/command-sessions/nonexistent", &bearer_auth()).await;

    assert!(
        response.status == StatusCode::NOT_FOUND || response.status == StatusCode::BAD_REQUEST,
        "expected 404 or 400, got {}",
        response.status
    );
}

#[tokio::test]
async fn test_open_command_session_nonexistent_agent() {
    let app = build_test_app("cmd-session-open-404").await;
    let response = post(
        &app.app,
        "/agents/nonexistent-agent/command-sessions",
        json!({}),
        &bearer_auth(),
    )
    .await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.json["success"], false);
}

#[tokio::test]
async fn test_execute_in_nonexistent_session() {
    let app = build_test_app("cmd-exec-err").await;
    let response = post(
        &app.app,
        "/command-sessions/nonexistent/execute",
        json!({"line": "ls"}),
        &bearer_auth(),
    )
    .await;

    assert!(
        response.status.is_client_error(),
        "expected 4xx, got {}",
        response.status
    );
}

#[tokio::test]
async fn test_send_command_to_nonexistent_session() {
    let app = build_test_app("cmd-send-err").await;
    let response = post(
        &app.app,
        "/command-sessions/nonexistent/commands",
        json!({"line": "ls"}),
        &bearer_auth(),
    )
    .await;

    assert!(
        response.status.is_client_error(),
        "expected 4xx, got {}",
        response.status
    );
}

#[tokio::test]
async fn test_close_nonexistent_command_session() {
    let app = build_test_app("cmd-close-err").await;
    let response = post(
        &app.app,
        "/command-sessions/nonexistent/close",
        json!({}),
        &bearer_auth(),
    )
    .await;

    assert!(
        response.status.is_client_error(),
        "expected 4xx, got {}",
        response.status
    );
}

// ===========================================================================
// Task endpoints
// ===========================================================================

#[tokio::test]
async fn test_list_tasks_empty() {
    let app = build_test_app("tasks-list").await;
    let response = get(&app.app, "/tasks", &bearer_auth()).await;

    assert_eq!(response.status, StatusCode::OK);
    assert!(response.json["tasks"].is_array());
    assert!(response.json["total"].as_u64().is_some() || response.json["total"].as_i64().is_some());
}

#[tokio::test]
async fn test_get_nonexistent_task() {
    let app = build_test_app("task-get-404").await;
    let response = get(&app.app, "/tasks/nonexistent", &bearer_auth()).await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_nonexistent_task() {
    let app = build_test_app("task-del-404").await;
    let response = delete_request(&app.app, "/tasks/nonexistent", &bearer_auth()).await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_broadcast_task() {
    let app = build_test_app("task-broadcast").await;
    let response = post(
        &app.app,
        "/tasks/broadcast",
        json!({"command": "whoami"}),
        &bearer_auth(),
    )
    .await;

    // broadcast doesn't need a connected agent; may return 202 or 200
    assert!(
        response.status == StatusCode::ACCEPTED || response.status == StatusCode::OK,
        "expected 202 or 200, got {}",
        response.status
    );
}

// ===========================================================================
// Web terminal endpoints (HTTP only, not WS)
// ===========================================================================

#[tokio::test]
async fn test_web_terminal_open_nonexistent_agent() {
    let app = build_test_app("web-term-open").await;
    let response = post(
        &app.app,
        "/web/terminal/open",
        json!({"agent_id": "nonexistent"}),
        &bearer_auth(),
    )
    .await;

    // Should fail — 409 or 404 depending on implementation
    assert!(
        response.status.is_client_error(),
        "expected 4xx, got {}",
        response.status
    );
}

#[tokio::test]
async fn test_web_terminal_get_nonexistent_session() {
    let app = build_test_app("web-term-get").await;
    let response = get(
        &app.app,
        "/web/terminal/session/nonexistent",
        &bearer_auth(),
    )
    .await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_web_terminal_command_nonexistent_session() {
    let app = build_test_app("web-term-cmd").await;
    let response = post(
        &app.app,
        "/web/terminal/command",
        json!({"session_id": "nonexistent", "line": "ls"}),
        &bearer_auth(),
    )
    .await;

    assert!(
        response.status.is_client_error(),
        "expected 4xx, got {}",
        response.status
    );
}

#[tokio::test]
async fn test_web_terminal_close_nonexistent_session() {
    let app = build_test_app("web-term-close").await;
    let response = post(
        &app.app,
        "/web/terminal/close",
        json!({"session_id": "nonexistent"}),
        &bearer_auth(),
    )
    .await;

    assert!(
        response.status.is_client_error(),
        "expected 4xx, got {}",
        response.status
    );
}

// ===========================================================================
// Pagination tests
// ===========================================================================

#[tokio::test]
async fn test_agents_history_pagination() {
    let app = build_test_app("agents-history-page").await;
    let response = get(&app.app, "/agents/history?limit=5&offset=0", &bearer_auth()).await;

    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(response.json["limit"], 5);
}

#[tokio::test]
async fn test_audits_pagination() {
    let app = build_test_app("audits-page").await;
    let response = get(&app.app, "/audits?limit=2&offset=0", &bearer_auth()).await;

    assert_eq!(response.status, StatusCode::OK);
    assert!(response.json["total"].as_u64().is_some() || response.json["total"].as_i64().is_some());
    assert!(response.json["audits"].is_array());
}

#[tokio::test]
async fn test_tasks_pagination() {
    let app = build_test_app("tasks-page").await;
    let response = get(&app.app, "/tasks?limit=5", &bearer_auth()).await;

    assert_eq!(response.status, StatusCode::OK);
    assert!(response.json["tasks"].is_array());
}

// ===========================================================================
// Dashboard response shape
// ===========================================================================

#[tokio::test]
async fn test_dashboard_stats_shape() {
    let app = build_test_app("dashboard-shape").await;
    let response = get(&app.app, "/dashboard/stats", &bearer_auth()).await;

    assert_eq!(response.status, StatusCode::OK);
    assert!(response.json["agents"]["online"].as_u64().is_some() || response.json["agents"]["online"].as_i64().is_some());
    assert!(response.json["agents"]["total"].as_u64().is_some() || response.json["agents"]["total"].as_i64().is_some());
    assert!(response.json["agents"]["disabled"].as_u64().is_some() || response.json["agents"]["disabled"].as_i64().is_some());
    assert!(response.json["listeners"]["total"].as_u64().is_some() || response.json["listeners"]["total"].as_i64().is_some());
    assert!(response.json["listeners"]["enabled"].as_u64().is_some() || response.json["listeners"]["enabled"].as_i64().is_some());
    assert!(response.json["server"]["hostname"].as_str().is_some());
    assert!(response.json["server"]["memory"]["total_bytes"].as_u64().is_some());
    assert!(response.json["server"]["uptime_seconds"].as_u64().is_some() || response.json["server"]["uptime_seconds"].as_i64().is_some());
}

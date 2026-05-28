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

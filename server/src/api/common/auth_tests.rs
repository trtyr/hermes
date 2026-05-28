use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use crate::kernel::{
    AgentAuthConfig, AgentAuthMode, AuthIdentity, KernelHandle, WebSession, new_kernel,
};

static NEXT_TEST_DB_ID: AtomicU64 = AtomicU64::new(1);

fn test_db_path(name: &str) -> PathBuf {
    let id = NEXT_TEST_DB_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "hermes-server-auth-{name}-{}-{id}.db",
        std::process::id()
    ))
}

async fn test_kernel(name: &str) -> KernelHandle {
    let sqlite_path = test_db_path(name);
    let _ = std::fs::remove_file(&sqlite_path);
    let agent_auth_config = AgentAuthConfig::shared(None, AgentAuthMode::PlainToken);
    new_kernel(
        32,
        32,
        sqlite_path,
        None,
        Some("user".to_string()),
        Some("pass".to_string()),
        8 * 60 * 60,
        agent_auth_config,
    )
    .await
    .expect("kernel starts")
}

#[tokio::test]
async fn test_login_and_session_check() {
    let kernel = test_kernel("auth-test").await;

    let username = kernel
        .auth()
        .validate_web_credentials("user", "pass")
        .expect("credentials validate");
    assert!(kernel.auth().validate_web_credentials("user", "wrong").is_none());

    let session: WebSession = kernel
        .auth()
        .create_session(&username)
        .expect("session created");
    assert_eq!(session.username, "user");

    let valid = kernel
        .auth()
        .lookup_session_token(&session.session_token)
        .is_some();
    assert!(valid);

    let identity: AuthIdentity = kernel
        .auth()
        .resolve_token(&session.session_token)
        .expect("session token resolves");
    assert_eq!(identity.username.as_deref(), Some("user"));
    assert_eq!(identity.session_token.as_deref(), Some(session.session_token.as_str()));
    assert!(!identity.via_legacy_api_token);
}

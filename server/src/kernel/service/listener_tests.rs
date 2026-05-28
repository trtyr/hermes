use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use serde_json::json;

use crate::{
    kernel::{AgentAuthConfig, AgentAuthMode, KernelHandle, new_kernel},
    protocol::ListenerKind,
};

static NEXT_TEST_DB_ID: AtomicU64 = AtomicU64::new(1);

fn test_db_path(name: &str) -> PathBuf {
    let id = NEXT_TEST_DB_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "hermes-server-listeners-{name}-{}-{id}.db",
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
        None,
        None,
        8 * 60 * 60,
        agent_auth_config,
    )
    .await
    .expect("kernel starts")
}

#[tokio::test]
async fn test_listener_crud() {
    let kernel = test_kernel("listener-crud").await;

    let listener = kernel
        .listener_commands()
        .create(
            "test-listener".to_string(),
            ListenerKind::TcpJson,
            "127.0.0.1".to_string(),
            40123,
            true,
            json!({"transport": "tcp"}),
        )
        .await
        .expect("listener created");

    let stored = kernel
        .listener_queries()
        .record(listener.listener_id)
        .await
        .expect("listener lookup succeeds")
        .expect("listener exists");
    assert_eq!(stored.name, "test-listener");
    assert_eq!(stored.kind, ListenerKind::TcpJson);

    let listeners = kernel
        .listener_queries()
        .filtered_records(None, None, None)
        .await
        .expect("listener list succeeds");
    assert!(listeners.iter().any(|item| item.listener_id == listener.listener_id));

    let deleted = kernel
        .listener_commands()
        .delete(listener.listener_id)
        .await
        .expect("listener delete succeeds");
    assert!(deleted);

    let after_delete = kernel
        .listener_queries()
        .record(listener.listener_id)
        .await
        .expect("listener lookup after delete succeeds");
    assert!(after_delete.is_none());
}

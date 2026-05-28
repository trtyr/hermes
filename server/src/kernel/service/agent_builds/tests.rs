use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use crate::{
    kernel::{AgentAuthConfig, AgentAuthMode, KernelHandle, new_kernel},
    protocol::AgentBuildStatus,
};

static NEXT_TEST_DB_ID: AtomicU64 = AtomicU64::new(1);

fn test_db_path(name: &str) -> PathBuf {
    let id = NEXT_TEST_DB_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "hermes-server-agent-builds-{name}-{}-{id}.db",
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
async fn test_create_and_list_builds() {
    let kernel = test_kernel("builds-list").await;

    let build = kernel
        .storage()
        .create_agent_build_record(
            "x86_64-pc-windows-msvc".to_string(),
            "debug".to_string(),
            None,
            "127.0.0.1:1234".to_string(),
            false,
        )
        .await
        .expect("build record created");

    assert_eq!(build.status, AgentBuildStatus::Pending);

    let builds = kernel
        .agent_builds()
        .filtered_records(None, None)
        .await
        .expect("build records listed");

    assert!(!builds.is_empty());
    assert!(builds.iter().any(|item| item.build_id == build.build_id));
}

#[tokio::test]
async fn test_delete_build() {
    let kernel = test_kernel("builds-delete").await;

    let build = kernel
        .storage()
        .create_agent_build_record(
            "x86_64-pc-windows-msvc".to_string(),
            "debug".to_string(),
            None,
            "127.0.0.1:2234".to_string(),
            false,
        )
        .await
        .expect("build record created");

    kernel
        .storage()
        .update_agent_build_record(
            build.build_id,
            AgentBuildStatus::Succeeded,
            Some("/tmp/agent.exe".to_string()),
            Some("agent.exe".to_string()),
            Some("completed for delete test".to_string()),
        )
        .await
        .expect("build record updated")
        .expect("build record still exists");

    let deleted = kernel
        .agent_builds()
        .delete_build(build.build_id)
        .await
        .expect("delete succeeds");

    assert!(deleted);
    assert!(kernel
        .agent_builds()
        .record(build.build_id)
        .await
        .expect("record lookup succeeds")
        .is_none());
}

#[tokio::test]
async fn test_build_idempotent() {
    let kernel = test_kernel("builds-idempotent").await;

    let first = kernel
        .storage()
        .create_agent_build_record(
            "x86_64-pc-windows-msvc".to_string(),
            "debug".to_string(),
            None,
            "127.0.0.1:3234".to_string(),
            false,
        )
        .await
        .expect("first build record created");

    let second = kernel
        .storage()
        .create_agent_build_record(
            "x86_64-pc-windows-msvc".to_string(),
            "debug".to_string(),
            None,
            "127.0.0.1:3234".to_string(),
            false,
        )
        .await
        .expect("second build record created");

    assert_ne!(first.build_id, second.build_id);

    let builds = kernel
        .agent_builds()
        .filtered_records(None, Some("x86_64-pc-windows-msvc".to_string()))
        .await
        .expect("build records filtered");

    assert!(builds.iter().any(|item| item.build_id == first.build_id));
    assert!(builds.iter().any(|item| item.build_id == second.build_id));
}

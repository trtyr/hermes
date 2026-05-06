use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use tokio::{
    sync::{mpsc, oneshot},
    task::yield_now,
    time::advance,
};

use super::{
    COMMAND_SESSION_CLOSE_TIMEOUT, COMMAND_SESSION_EXECUTE_TIMEOUT, COMMAND_SESSION_OPEN_TIMEOUT,
    is_command_session_timeout,
};
use crate::{
    kernel::{
        AgentAuthConfig, AgentAuthMode, KernelHandle, new_kernel,
        state::{AgentIdentity, AgentSession},
    },
    protocol::ServerCommand,
};

static NEXT_TEST_DB_ID: AtomicU64 = AtomicU64::new(1);

fn test_db_path(name: &str) -> PathBuf {
    let id = NEXT_TEST_DB_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "hermes-server-{name}-{}-{id}.db",
        std::process::id()
    ))
}

fn wall_clock_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock after epoch")
        .as_millis() as u64
}

async fn test_kernel(name: &str) -> KernelHandle {
    let sqlite_path = test_db_path(name);
    let _ = std::fs::remove_file(&sqlite_path);
    let agent_auth_config = AgentAuthConfig::shared(None, AgentAuthMode::PlainToken);
    new_kernel(32, 32, sqlite_path, None, None, None, 8 * 60 * 60, agent_auth_config)
        .await
        .expect("kernel starts")
}

async fn seed_connected_agent(kernel: &KernelHandle, agent_id: &str) {
    let (sender, receiver) = mpsc::unbounded_channel::<ServerCommand>();
    std::mem::forget(receiver);
    seed_agent_with_sender(kernel, agent_id, sender).await;
}

async fn seed_agent_with_closed_sender(kernel: &KernelHandle, agent_id: &str) {
    let (sender, receiver) = mpsc::unbounded_channel::<ServerCommand>();
    drop(receiver);
    seed_agent_with_sender(kernel, agent_id, sender).await;
}

async fn seed_agent_with_sender(
    kernel: &KernelHandle,
    agent_id: &str,
    sender: mpsc::UnboundedSender<ServerCommand>,
) {
    let now = wall_clock_now();
    let mut state = kernel.state.write().await;
    state.insert_session(AgentSession {
        session_id: 1,
        agent_id: None,
        listener_id: Some(1),
        listener_name: Some("default-agent-tcp".to_string()),
        hostname: None,
        username: None,
        os: None,
        arch: None,
        pid: None,
        internal_ip: None,
        tags: Vec::new(),
        sleep_interval: 60,
        jitter: 0,
        peer_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 4321)),
        connected_at: now,
        last_seen: now,
        sender,
        privilege: String::new(),
    });
    let snapshot = state.upsert_agent_identity(
        1,
        AgentIdentity {
            agent_id: agent_id.to_string(),
            hostname: "host".to_string(),
            username: None,
            os: None,
            arch: None,
            pid: None,
            internal_ip: None,
            tags: Vec::new(),
            sleep_interval: 60,
            jitter: 0,
            last_seen: now,
            privilege: String::new(),
        },
    );
    assert!(snapshot.is_some());
}

async fn seed_open_command_session(
    kernel: &KernelHandle,
    command_session_id: &str,
    agent_id: &str,
) {
    let (sender, _receiver) = oneshot::channel();
    let now = wall_clock_now();
    let mut state = kernel.state.write().await;
    state.insert_command_session(
        command_session_id.to_string(),
        agent_id.to_string(),
        "tester".to_string(),
        now,
        sender,
    );
    let snapshot = state.activate_command_session(command_session_id, "/tmp".to_string(), now);
    assert!(snapshot.is_some());
}

#[tokio::test(start_paused = true)]
async fn open_times_out_and_removes_placeholder_session() {
    let kernel = test_kernel("cmd-open-timeout").await;
    seed_connected_agent(&kernel, "agent-open").await;

    let facade = kernel.command_sessions();
    let future = tokio::spawn(async move {
        facade
            .open("agent-open".to_string(), "tester".to_string())
            .await
    });

    yield_now().await;
    advance(COMMAND_SESSION_OPEN_TIMEOUT + std::time::Duration::from_millis(1)).await;

    let error = future
        .await
        .expect("join succeeds")
        .expect_err("open should time out");
    assert!(is_command_session_timeout(&error));
    assert!(
        kernel
            .command_sessions()
            .snapshot("cmdsess-1")
            .await
            .is_none()
    );
}

#[tokio::test(start_paused = true)]
async fn execute_times_out_and_cleans_pending_request() {
    let kernel = test_kernel("cmd-execute-timeout").await;
    seed_connected_agent(&kernel, "agent-exec").await;
    seed_open_command_session(&kernel, "cmdsess-1", "agent-exec").await;

    let facade = kernel.command_sessions();
    let future = tokio::spawn(async move {
        facade
            .execute("cmdsess-1".to_string(), "pwd".to_string())
            .await
    });

    yield_now().await;
    advance(COMMAND_SESSION_EXECUTE_TIMEOUT + std::time::Duration::from_millis(1)).await;

    let error = future
        .await
        .expect("join succeeds")
        .expect_err("execute should time out");
    assert!(is_command_session_timeout(&error));

    let mut state = kernel.state.write().await;
    assert!(!state.abort_pending_command_execute("cmdreq-1"));
}

#[tokio::test(start_paused = true)]
async fn close_times_out_and_leaves_session_open() {
    let kernel = test_kernel("cmd-close-timeout").await;
    seed_connected_agent(&kernel, "agent-close").await;
    seed_open_command_session(&kernel, "cmdsess-1", "agent-close").await;

    let facade = kernel.command_sessions();
    let future = tokio::spawn(async move { facade.close("cmdsess-1".to_string()).await });

    yield_now().await;
    advance(COMMAND_SESSION_CLOSE_TIMEOUT + std::time::Duration::from_millis(1)).await;

    let error = future
        .await
        .expect("join succeeds")
        .expect_err("close should time out");
    assert!(is_command_session_timeout(&error));
    assert!(
        !kernel
            .state
            .write()
            .await
            .abort_pending_close_command_session("cmdsess-1")
    );
    assert_eq!(
        kernel
            .command_sessions()
            .snapshot("cmdsess-1")
            .await
            .expect("session still exists")
            .status,
        crate::protocol::CommandSessionStatus::Open
    );
}

#[tokio::test]
async fn open_fails_fast_when_agent_sender_is_closed() {
    let kernel = test_kernel("cmd-open-sender-closed").await;
    seed_agent_with_closed_sender(&kernel, "agent-open-closed").await;

    let error = kernel
        .command_sessions()
        .open("agent-open-closed".to_string(), "tester".to_string())
        .await
        .expect_err("open should fail when agent sender is closed");

    assert!(error.to_string().contains("sender closed"));
    assert!(
        !kernel
            .agent_queries()
            .is_connected("agent-open-closed")
            .await
    );
}

#[tokio::test]
async fn execute_fails_fast_when_agent_sender_is_closed() {
    let kernel = test_kernel("cmd-execute-sender-closed").await;
    seed_agent_with_closed_sender(&kernel, "agent-exec-closed").await;
    seed_open_command_session(&kernel, "cmdsess-1", "agent-exec-closed").await;

    let error = kernel
        .command_sessions()
        .execute("cmdsess-1".to_string(), "pwd".to_string())
        .await
        .expect_err("execute should fail when agent sender is closed");

    assert!(error.to_string().contains("sender closed"));
    assert!(
        !kernel
            .agent_queries()
            .is_connected("agent-exec-closed")
            .await
    );
    assert_eq!(
        kernel
            .command_sessions()
            .snapshot("cmdsess-1")
            .await
            .expect("command session snapshot still exists")
            .status,
        crate::protocol::CommandSessionStatus::Open
    );
}

#[tokio::test]
async fn close_fails_fast_when_agent_sender_is_closed() {
    let kernel = test_kernel("cmd-close-sender-closed").await;
    seed_agent_with_closed_sender(&kernel, "agent-close-closed").await;
    seed_open_command_session(&kernel, "cmdsess-1", "agent-close-closed").await;

    let error = kernel
        .command_sessions()
        .close("cmdsess-1".to_string())
        .await
        .expect_err("close should fail when agent sender is closed");

    assert!(error.to_string().contains("sender closed"));
    assert_eq!(
        kernel
            .command_sessions()
            .snapshot("cmdsess-1")
            .await
            .expect("command session snapshot still exists")
            .status,
        crate::protocol::CommandSessionStatus::Open
    );
    assert!(
        !kernel
            .agent_queries()
            .is_connected("agent-close-closed")
            .await
    );
}

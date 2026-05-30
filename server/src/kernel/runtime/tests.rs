use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use tokio::sync::{RwLock, broadcast, mpsc, oneshot};

use super::{agent_lifecycle, effects::RuntimePorts, task_flow};
use crate::{
    kernel::{
        state::{AgentIdentity, AgentSession, KernelState, NewTask},
        storage::Storage,
    },
    protocol::{
        AgentMessage, CommandExecutionStatus, CommandSessionStatus, ServerCommand, TaskStatus,
    },
};

static NEXT_TEST_DB_ID: AtomicU64 = AtomicU64::new(1);

fn test_db_path(name: &str) -> PathBuf {
    let id = NEXT_TEST_DB_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "hermes-runtime-{name}-{}-{id}.db",
        std::process::id()
    ))
}

fn wall_clock_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock after epoch")
        .as_millis() as u64
}

async fn test_runtime_ports(name: &str) -> RuntimePorts {
    let sqlite_path = test_db_path(name);
    let _ = std::fs::remove_file(&sqlite_path);
    let storage = Storage::new(sqlite_path).await.expect("storage starts");
    let (events, _) = broadcast::channel(32);
    RuntimePorts::new(events, storage)
}

fn open_sender() -> mpsc::UnboundedSender<crate::protocol::ServerCommand> {
    let (sender, receiver) = mpsc::unbounded_channel();
    std::mem::forget(receiver);
    sender
}

#[tokio::test]
async fn late_register_does_not_evict_existing_agent_session() {
    let state = Arc::new(RwLock::new(KernelState::new()));
    let effects = test_runtime_ports("late-register").await;
    let now = wall_clock_now();

    {
        let mut state = state.write().await;
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
            peer_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 4101)),
            connected_at: now,
            last_seen: now,
            sender: open_sender(),
            privilege: String::new(),
        });
        state.upsert_agent_identity(
            1,
            AgentIdentity {
                agent_id: "agent-1".to_string(),
                hostname: "old-host".to_string(),
                username: Some("tester".to_string()),
                os: Some("windows".to_string()),
                arch: Some("amd64".to_string()),
                pid: Some(1001),
                internal_ip: Some("10.0.0.8".to_string()),
                tags: vec!["prod".to_string()],
                sleep_interval: 60,
                jitter: 0,
                last_seen: now,
                privilege: String::new(),
            },
        );

        state.create_task(NewTask {
            task_id: "task-1".to_string(),
            parent_task_id: None,
            target_agent_id: Some("agent-1".to_string()),
            command: "whoami".to_string(),
            payload: None,
            created_at: now,
        });
        state.mark_task_running("task-1", now + 1);

        let (open_tx, _open_rx) = oneshot::channel();
        state.insert_command_session(
            "cmdsess-1".to_string(),
            "agent-1".to_string(),
            "tester".to_string(),
            now,
            open_tx,
        );
        state.activate_command_session("cmdsess-1", "/tmp".to_string(), now);
        state.queue_command_execution("cmd-1".to_string(), "cmdsess-1", "pwd".to_string(), now);
        state.mark_command_dispatched("cmdsess-1", "cmd-1", now + 1);
        state.mark_command_running("cmdsess-1", "cmd-1", now + 2);
    }

    agent_lifecycle::handle_agent_frame(
        &state,
        &effects,
        2,
        AgentMessage::Register {
            agent_id: "agent-1".to_string(),
            hostname: "new-host".to_string(),
            username: Some("tester".to_string()),
            protocol_version: 2,
            os: Some("windows".to_string()),
            arch: Some("amd64".to_string()),
            pid: Some(2002),
            internal_ip: Some("10.0.0.9".to_string()),
            tags: vec!["prod".to_string()],
            sleep_interval: 60,
            jitter: 0,
            token: None,
            session_nonce: None,
            auth_response: None,
            privilege: String::new(),
        },
    )
    .await;

    let state = state.read().await;
    let session = state
        .session_by_agent_id("agent-1")
        .expect("old session should remain registered");
    assert_eq!(session.session_id, 1);
    assert_eq!(session.hostname.as_deref(), Some("old-host"));

    let task = state.task_snapshot("task-1").expect("task remains");
    assert_eq!(task.status, TaskStatus::Running);

    let command_session = state
        .command_session_snapshot("cmdsess-1")
        .expect("command session remains");
    assert_eq!(command_session.status, CommandSessionStatus::Open);

    let command = state
        .command_execution_snapshot("cmd-1")
        .expect("command remains");
    assert_eq!(command.status, CommandExecutionStatus::Running);
}

#[tokio::test]
async fn heartbeat_dispatches_pending_tasks_for_registered_agent() {
    let state = Arc::new(RwLock::new(KernelState::new()));
    let effects = test_runtime_ports("heartbeat-dispatch").await;
    let now = wall_clock_now();
    let (sender, mut receiver) = mpsc::unbounded_channel();

    {
        let mut state = state.write().await;
        state.insert_session(AgentSession {
            session_id: 7,
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
            sleep_interval: 15,
            jitter: 10,
            peer_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 4107)),
            connected_at: now,
            last_seen: now,
            sender,
            privilege: String::new(),
        });
        state.upsert_agent_identity(
            7,
            AgentIdentity {
                agent_id: "agent-heartbeat".to_string(),
                hostname: "host-1".to_string(),
                username: Some("tester".to_string()),
                os: Some("windows".to_string()),
                arch: Some("amd64".to_string()),
                pid: Some(7001),
                internal_ip: Some("10.0.0.7".to_string()),
                tags: vec!["qa".to_string()],
                sleep_interval: 15,
                jitter: 10,
                last_seen: now,
                privilege: String::new(),
            },
        );
        state.create_task(NewTask {
            task_id: "task-pending-1".to_string(),
            parent_task_id: None,
            target_agent_id: Some("agent-heartbeat".to_string()),
            command: "whoami".to_string(),
            payload: None,
            created_at: now + 1,
        });
    }

    agent_lifecycle::handle_agent_frame(
        &state,
        &effects,
        7,
        AgentMessage::Heartbeat {
            agent_id: Some("agent-heartbeat".to_string()),
        },
    )
    .await;

    let command = receiver
        .try_recv()
        .expect("heartbeat should return ack with pending task");
    match command {
        ServerCommand::Ack { message, tasks } => {
            assert_eq!(message, "ok");
            let tasks = tasks.expect("ack should include pending tasks");
            assert_eq!(tasks.len(), 1);
            let task = &tasks[0];
            assert_eq!(task.task_id, "task-pending-1");
            assert_eq!(task.command, "whoami");
            assert_eq!(task.payload, None);
        }
        other => panic!("unexpected command: {:?}", other),
    }

    let state = state.read().await;
    let task = state
        .task_snapshot("task-pending-1")
        .expect("task should remain visible");
    assert_eq!(task.status, TaskStatus::Dispatched);
}

#[tokio::test]
async fn tcp_disconnect_preserves_agent_state_for_reconnect() {
    let state = Arc::new(RwLock::new(KernelState::new()));
    let effects = test_runtime_ports("tcp-disconnect-preserves-state").await;
    let now = wall_clock_now();

    let (active_session_sender, _active_session_receiver) = oneshot::channel();
    let (pending_session_sender, mut pending_session_receiver) = oneshot::channel();
    let (command_sender, mut command_receiver) = oneshot::channel();
    let (beacon_sender, mut beacon_receiver) = oneshot::channel();

    {
        let mut state = state.write().await;
        state.insert_session(AgentSession {
            session_id: 9,
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
            sleep_interval: 30,
            jitter: 20,
            peer_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 4109)),
            connected_at: now,
            last_seen: now,
            sender: open_sender(),
            privilege: String::new(),
        });
        state.upsert_agent_identity(
            9,
            AgentIdentity {
                agent_id: "agent-reconnect".to_string(),
                hostname: "host-9".to_string(),
                username: Some("tester".to_string()),
                os: Some("windows".to_string()),
                arch: Some("amd64".to_string()),
                pid: Some(9001),
                internal_ip: Some("10.0.0.9".to_string()),
                tags: vec!["prod".to_string()],
                sleep_interval: 30,
                jitter: 20,
                last_seen: now,
                privilege: String::new(),
            },
        );

        state.create_task(NewTask {
            task_id: "task-running-9".to_string(),
            parent_task_id: None,
            target_agent_id: Some("agent-reconnect".to_string()),
            command: "sleep".to_string(),
            payload: Some("30".to_string()),
            created_at: now,
        });
        state.mark_task_dispatched("task-running-9", now + 1);
        state.mark_task_running("task-running-9", now + 2);

        state.create_task(NewTask {
            task_id: "task-pending-9".to_string(),
            parent_task_id: None,
            target_agent_id: Some("agent-reconnect".to_string()),
            command: "hostname".to_string(),
            payload: None,
            created_at: now + 3,
        });

        state.insert_command_session(
            "cmdsess-9".to_string(),
            "agent-reconnect".to_string(),
            "tester".to_string(),
            now,
            active_session_sender,
        );
        state.activate_command_session("cmdsess-9", "/tmp".to_string(), now + 1);
        state.insert_command_session(
            "cmdsess-pending-9".to_string(),
            "agent-reconnect".to_string(),
            "tester".to_string(),
            now + 1,
            pending_session_sender,
        );
        state.queue_command_execution("cmd-9".to_string(), "cmdsess-9", "pwd".to_string(), now + 2);
        state.mark_command_dispatched("cmdsess-9", "cmd-9", now + 3);
        state.mark_command_running("cmdsess-9", "cmd-9", now + 4);
        state.register_pending_command_execute("cmd-9".to_string(), command_sender);
        state.register_pending_agent_beacon_update(
            "beacon-9".to_string(),
            "agent-reconnect".to_string(),
            beacon_sender,
        );
    }

    agent_lifecycle::handle_agent_disconnected(&state, &effects, 9).await;

    let state = state.read().await;
    assert_eq!(state.existing_session_for_agent("agent-reconnect"), Some(9));
    assert!(state.session_by_agent_id("agent-reconnect").is_none());
    assert_eq!(
        state
            .task_snapshot("task-running-9")
            .expect("running task remains")
            .status,
        TaskStatus::Running
    );
    assert_eq!(
        state
            .task_snapshot("task-pending-9")
            .expect("pending task remains")
            .status,
        TaskStatus::Pending
    );
    assert_eq!(
        state
            .command_session_snapshot("cmdsess-9")
            .expect("command session remains")
            .status,
        CommandSessionStatus::Open
    );
    assert_eq!(
        state
            .command_execution_snapshot("cmd-9")
            .expect("command remains")
            .status,
        CommandExecutionStatus::Running
    );
    drop(state);

    assert!(matches!(
        pending_session_receiver.try_recv(),
        Err(tokio::sync::oneshot::error::TryRecvError::Empty)
    ));
    assert!(matches!(
        command_receiver.try_recv(),
        Err(tokio::sync::oneshot::error::TryRecvError::Empty)
    ));
    assert!(matches!(
        beacon_receiver.try_recv(),
        Err(tokio::sync::oneshot::error::TryRecvError::Empty)
    ));
}

#[tokio::test]
async fn heartbeat_timeout_fully_cleans_agent_state() {
    let state = Arc::new(RwLock::new(KernelState::new()));
    let effects = test_runtime_ports("heartbeat-timeout-cleans-state").await;
    let now = wall_clock_now();

    let (active_session_sender, _active_session_receiver) = oneshot::channel();
    let (pending_session_sender, mut pending_session_receiver) = oneshot::channel();
    let (command_sender, mut command_receiver) = oneshot::channel();
    let (beacon_sender, mut beacon_receiver) = oneshot::channel();

    {
        let mut state = state.write().await;
        state.insert_session(AgentSession {
            session_id: 10,
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
            sleep_interval: 1,
            jitter: 0,
            peer_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 4110)),
            connected_at: now.saturating_sub(20_000),
            last_seen: now.saturating_sub(20_000),
            sender: open_sender(),
            privilege: String::new(),
        });
        state.upsert_agent_identity(
            10,
            AgentIdentity {
                agent_id: "agent-expired".to_string(),
                hostname: "host-10".to_string(),
                username: Some("tester".to_string()),
                os: Some("windows".to_string()),
                arch: Some("amd64".to_string()),
                pid: Some(10001),
                internal_ip: Some("10.0.0.10".to_string()),
                tags: vec!["prod".to_string()],
                sleep_interval: 1,
                jitter: 0,
                last_seen: now.saturating_sub(20_000),
                privilege: String::new(),
            },
        );

        state.create_task(NewTask {
            task_id: "task-running-10".to_string(),
            parent_task_id: None,
            target_agent_id: Some("agent-expired".to_string()),
            command: "sleep".to_string(),
            payload: Some("30".to_string()),
            created_at: now,
        });
        state.mark_task_dispatched("task-running-10", now + 1);
        state.mark_task_running("task-running-10", now + 2);

        state.create_task(NewTask {
            task_id: "task-pending-10".to_string(),
            parent_task_id: None,
            target_agent_id: Some("agent-expired".to_string()),
            command: "hostname".to_string(),
            payload: None,
            created_at: now + 3,
        });

        state.insert_command_session(
            "cmdsess-10".to_string(),
            "agent-expired".to_string(),
            "tester".to_string(),
            now,
            active_session_sender,
        );
        state.activate_command_session("cmdsess-10", "/tmp".to_string(), now + 1);
        state.insert_command_session(
            "cmdsess-pending-10".to_string(),
            "agent-expired".to_string(),
            "tester".to_string(),
            now + 1,
            pending_session_sender,
        );
        state.queue_command_execution(
            "cmd-10".to_string(),
            "cmdsess-10",
            "pwd".to_string(),
            now + 2,
        );
        state.mark_command_dispatched("cmdsess-10", "cmd-10", now + 3);
        state.mark_command_running("cmdsess-10", "cmd-10", now + 4);
        state.register_pending_command_execute("cmd-10".to_string(), command_sender);
        state.register_pending_agent_beacon_update(
            "beacon-10".to_string(),
            "agent-expired".to_string(),
            beacon_sender,
        );
    }

    agent_lifecycle::sweep_heartbeats(&state, &effects).await;

    let state = state.read().await;
    assert!(state.existing_session_for_agent("agent-expired").is_none());
    assert_eq!(
        state
            .command_execution_snapshot("cmd-10")
            .expect("command record remains visible")
            .status,
        CommandExecutionStatus::Dropped
    );
    assert_eq!(
        state
            .command_session_snapshot("cmdsess-10")
            .expect("command session record remains closed")
            .status,
        CommandSessionStatus::Closed
    );
    assert_eq!(
        state
            .task_snapshot("task-running-10")
            .expect("running task still recorded")
            .status,
        TaskStatus::Failed
    );
    assert_eq!(
        state
            .task_snapshot("task-pending-10")
            .expect("pending task still recorded")
            .status,
        TaskStatus::Failed
    );
    drop(state);

    assert!(
        pending_session_receiver
            .try_recv()
            .expect("pending session open should complete")
            .expect_err("pending session open should fail")
            .to_string()
            .contains("heartbeat timed out")
    );
    assert!(
        command_receiver
            .try_recv()
            .expect("pending command should complete")
            .expect_err("pending command should fail")
            .to_string()
            .contains("heartbeat timed out")
    );
    assert!(
        beacon_receiver
            .try_recv()
            .expect("pending beacon update should complete")
            .expect_err("pending beacon update should fail")
            .to_string()
            .contains("heartbeat timed out")
    );
}

#[tokio::test]
async fn cancel_active_task_marks_cancel_requested_and_sends_command() {
    let state = Arc::new(RwLock::new(KernelState::new()));
    let effects = test_runtime_ports("cancel-active-task").await;
    let now = wall_clock_now();
    let (sender, mut receiver) = mpsc::unbounded_channel();

    {
        let mut state = state.write().await;
        state.insert_session(AgentSession {
            session_id: 8,
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
            sleep_interval: 15,
            jitter: 10,
            peer_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 4108)),
            connected_at: now,
            last_seen: now,
            sender,
            privilege: String::new(),
        });
        state.upsert_agent_identity(
            8,
            AgentIdentity {
                agent_id: "agent-cancel".to_string(),
                hostname: "host-8".to_string(),
                username: Some("tester".to_string()),
                os: Some("windows".to_string()),
                arch: Some("amd64".to_string()),
                pid: Some(8001),
                internal_ip: Some("10.0.0.8".to_string()),
                tags: Vec::new(),
                sleep_interval: 15,
                jitter: 10,
                last_seen: now,
                privilege: String::new(),
            },
        );
        state.create_task(NewTask {
            task_id: "task-running-1".to_string(),
            parent_task_id: None,
            target_agent_id: Some("agent-cancel".to_string()),
            command: "sleep".to_string(),
            payload: Some("10".to_string()),
            created_at: now,
        });
        state.mark_task_dispatched("task-running-1", now + 1);
        state.mark_task_running("task-running-1", now + 2);
    }

    task_flow::cancel_task(&state, &effects, "task-running-1".to_string()).await;

    let command = receiver
        .try_recv()
        .expect("cancel should be sent to active agent task");
    match command {
        ServerCommand::CancelTask { task_id } => assert_eq!(task_id, "task-running-1"),
        other => panic!("unexpected command: {:?}", other),
    }

    let state = state.read().await;
    let task = state
        .task_snapshot("task-running-1")
        .expect("task should remain visible");
    assert_eq!(task.status, TaskStatus::CancelRequested);
    assert_eq!(task.output.as_deref(), Some("cancel requested by operator"));
}

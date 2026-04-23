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
                os: Some("linux".to_string()),
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
            os: Some("linux".to_string()),
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
                os: Some("linux".to_string()),
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
        .expect("task should be dispatched on heartbeat");
    match command {
        ServerCommand::DispatchTask {
            task_id,
            command,
            payload,
        } => {
            assert_eq!(task_id, "task-pending-1");
            assert_eq!(command, "whoami");
            assert_eq!(payload, None);
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
                os: Some("linux".to_string()),
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

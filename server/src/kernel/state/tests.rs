use std::{
    collections::VecDeque,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
};

use tokio::sync::{mpsc, oneshot};

use super::{
    AgentIdentity, AgentSession, CommandExecutionRecord, CommandExecutionStatus,
    CommandSessionRecord, CommandSessionStatus, KernelState, NewTask,
};
use crate::protocol::{CommandOutputStream, TaskStatus};

#[test]
fn abort_pending_open_command_session_removes_placeholder_record() {
    let mut state = KernelState::new();
    let (tx, mut rx) = oneshot::channel();

    state.insert_command_session(
        "cmdsess-1".to_string(),
        "agent-1".to_string(),
        "tester".to_string(),
        1,
        tx,
    );

    assert!(state.command_session_snapshot("cmdsess-1").is_some());
    assert!(state.abort_pending_open_command_session("cmdsess-1"));
    assert!(state.command_session_snapshot("cmdsess-1").is_none());
    assert!(matches!(
        rx.try_recv(),
        Err(tokio::sync::oneshot::error::TryRecvError::Closed)
    ));
}

#[test]
fn disconnect_only_fails_pending_executes_for_same_agent() {
    let mut state = KernelState::new();
    state.command_sessions.insert(
        "cmdsess-a".to_string(),
        CommandSessionRecord {
            command_session_id: "cmdsess-a".to_string(),
            agent_id: "agent-a".to_string(),
            cwd: "/tmp".to_string(),
            status: CommandSessionStatus::Open,
            created_by: "tester".to_string(),
            created_at: 1,
            last_active_at: 1,
            active_command_id: None,
            queued_command_ids: VecDeque::new(),
        },
    );
    state.command_sessions.insert(
        "cmdsess-b".to_string(),
        CommandSessionRecord {
            command_session_id: "cmdsess-b".to_string(),
            agent_id: "agent-b".to_string(),
            cwd: "/tmp".to_string(),
            status: CommandSessionStatus::Open,
            created_by: "tester".to_string(),
            created_at: 1,
            last_active_at: 1,
            active_command_id: None,
            queued_command_ids: VecDeque::new(),
        },
    );
    state.command_executions.insert(
        "req-a".to_string(),
        CommandExecutionRecord {
            command_id: "req-a".to_string(),
            command_session_id: "cmdsess-a".to_string(),
            agent_id: "agent-a".to_string(),
            line: "whoami".to_string(),
            status: CommandExecutionStatus::Queued,
            queued_at: 1,
            updated_at: 1,
            dispatched_at: None,
            started_at: None,
            finished_at: None,
            cwd_before: None,
            cwd_after: None,
            exit_code: None,
            stdout: None,
            stderr: None,
            success: None,
        },
    );
    state.command_executions.insert(
        "req-b".to_string(),
        CommandExecutionRecord {
            command_id: "req-b".to_string(),
            command_session_id: "cmdsess-b".to_string(),
            agent_id: "agent-b".to_string(),
            line: "hostname".to_string(),
            status: CommandExecutionStatus::Queued,
            queued_at: 1,
            updated_at: 1,
            dispatched_at: None,
            started_at: None,
            finished_at: None,
            cwd_before: None,
            cwd_after: None,
            exit_code: None,
            stdout: None,
            stderr: None,
            success: None,
        },
    );

    let (tx_a, mut rx_a) = oneshot::channel();
    let (tx_b, mut rx_b) = oneshot::channel();
    state.register_pending_command_execute("req-a".to_string(), tx_a);
    state.register_pending_command_execute("req-b".to_string(), tx_b);

    state.fail_pending_command_sessions_for_agent("agent-a", "agent-a disconnected");

    let err = rx_a
        .try_recv()
        .expect("agent-a request should be completed")
        .expect_err("agent-a request should fail");
    assert!(err.to_string().contains("agent-a disconnected"));
    assert!(matches!(
        rx_b.try_recv(),
        Err(tokio::sync::oneshot::error::TryRecvError::Empty)
    ));
    assert!(state.abort_pending_command_execute("req-b"));
    assert!(matches!(
        rx_b.try_recv(),
        Err(tokio::sync::oneshot::error::TryRecvError::Closed)
    ));
}

#[test]
fn broadcast_parent_stays_active_until_any_child_reaches_terminal_state() {
    let mut state = KernelState::new();

    let parent = state.create_task(NewTask {
        task_id: "parent".to_string(),
        parent_task_id: None,
        target_agent_id: None,
        command: "whoami".to_string(),
        payload: None,
        created_at: 1,
    });
    assert_eq!(parent.status, TaskStatus::Pending);

    state.create_task(NewTask {
        task_id: "child-1".to_string(),
        parent_task_id: Some("parent".to_string()),
        target_agent_id: Some("agent-1".to_string()),
        command: "whoami".to_string(),
        payload: None,
        created_at: 2,
    });
    assert_eq!(
        state.task_snapshot("parent").expect("parent exists").status,
        TaskStatus::Pending
    );

    state.create_task(NewTask {
        task_id: "child-2".to_string(),
        parent_task_id: Some("parent".to_string()),
        target_agent_id: Some("agent-2".to_string()),
        command: "whoami".to_string(),
        payload: None,
        created_at: 2,
    });
    assert_eq!(
        state.task_snapshot("parent").expect("parent exists").status,
        TaskStatus::Pending
    );

    state.mark_task_dispatched("child-1", 3);
    assert_eq!(
        state.task_snapshot("parent").expect("parent exists").status,
        TaskStatus::Dispatched
    );

    state.mark_task_running("child-2", 4);
    assert_eq!(
        state.task_snapshot("parent").expect("parent exists").status,
        TaskStatus::Running
    );

    state.complete_task("child-1", true, "ok".to_string(), 5);
    assert_eq!(
        state.task_snapshot("parent").expect("parent exists").status,
        TaskStatus::Partial
    );
}

#[test]
fn broadcast_parent_uses_terminal_status_when_children_converge() {
    let mut state = KernelState::new();
    state.create_task(NewTask {
        task_id: "failed-parent".to_string(),
        parent_task_id: None,
        target_agent_id: None,
        command: "hostname".to_string(),
        payload: None,
        created_at: 1,
    });

    for child_id in ["child-1", "child-2"] {
        state.create_task(NewTask {
            task_id: child_id.to_string(),
            parent_task_id: Some("failed-parent".to_string()),
            target_agent_id: Some(format!("agent-{child_id}")),
            command: "hostname".to_string(),
            payload: None,
            created_at: 2,
        });
    }

    state.complete_task("child-1", false, "fail".to_string(), 3);
    state.complete_task("child-2", false, "fail".to_string(), 4);
    assert_eq!(
        state
            .task_snapshot("failed-parent")
            .expect("failed parent exists")
            .status,
        TaskStatus::Failed
    );

    state.create_task(NewTask {
        task_id: "cancelled-parent".to_string(),
        parent_task_id: None,
        target_agent_id: None,
        command: "hostname".to_string(),
        payload: None,
        created_at: 5,
    });
    for child_id in ["cancel-child-1", "cancel-child-2"] {
        state.create_task(NewTask {
            task_id: child_id.to_string(),
            parent_task_id: Some("cancelled-parent".to_string()),
            target_agent_id: Some(format!("agent-{child_id}")),
            command: "hostname".to_string(),
            payload: None,
            created_at: 6,
        });
    }

    state.mark_task_cancelled("cancel-child-1", Some("cancel".to_string()), 7);
    state.mark_task_cancelled("cancel-child-2", Some("cancel".to_string()), 8);
    assert_eq!(
        state
            .task_snapshot("cancelled-parent")
            .expect("cancelled parent exists")
            .status,
        TaskStatus::Cancelled
    );
}

#[test]
fn snapshots_only_include_registered_agents() {
    let mut state = KernelState::new();
    let (sender, _receiver) = mpsc::unbounded_channel();

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
        peer_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 4444)),
        connected_at: 1,
        last_seen: 1,
        sender,
        privilege: String::new(),
    });

    assert!(state.snapshots().is_empty());

    state.upsert_agent_identity(
        1,
        AgentIdentity {
            agent_id: "agent-1".to_string(),
            hostname: "host".to_string(),
            username: None,
            os: None,
            arch: None,
            pid: None,
            internal_ip: None,
            tags: Vec::new(),
            sleep_interval: 60,
            jitter: 0,
            last_seen: 2,
            privilege: String::new(),
        },
    );

    let snapshots = state.snapshots();
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0].agent_id.as_deref(), Some("agent-1"));
}

#[test]
fn cancelled_task_ignores_late_result_and_running_update() {
    let mut state = KernelState::new();
    state.create_task(NewTask {
        task_id: "task-1".to_string(),
        parent_task_id: None,
        target_agent_id: Some("agent-1".to_string()),
        command: "sleep".to_string(),
        payload: Some("10".to_string()),
        created_at: 1,
    });

    state.mark_task_running("task-1", 2);
    state.mark_task_cancelled("task-1", Some("cancelled".to_string()), 3);

    assert!(state.mark_task_running("task-1", 4).is_none());
    assert!(
        state
            .complete_task("task-1", true, "late ok".to_string(), 5)
            .is_none()
    );

    let snapshot = state.task_snapshot("task-1").expect("task exists");
    assert_eq!(snapshot.status, TaskStatus::Cancelled);
    assert_eq!(snapshot.output.as_deref(), Some("cancelled"));
    assert_eq!(snapshot.success, Some(false));
}

#[test]
fn cancel_requested_task_accepts_terminal_result() {
    let mut state = KernelState::new();
    state.create_task(NewTask {
        task_id: "task-1".to_string(),
        parent_task_id: None,
        target_agent_id: Some("agent-1".to_string()),
        command: "sleep".to_string(),
        payload: Some("2".to_string()),
        created_at: 1,
    });

    state.mark_task_dispatched("task-1", 2);
    state.mark_task_running("task-1", 3);
    state.mark_task_cancel_requested("task-1", Some("cancel requested".to_string()), 4);

    let snapshot = state
        .complete_task("task-1", true, "finished before kill".to_string(), 5)
        .expect("terminal result should still be accepted");
    assert_eq!(snapshot.status, TaskStatus::Succeeded);
    assert_eq!(snapshot.output.as_deref(), Some("finished before kill"));
    assert_eq!(snapshot.success, Some(true));
}

#[test]
fn cancel_requested_is_counted_as_active_for_agent() {
    let mut state = KernelState::new();
    state.create_task(NewTask {
        task_id: "task-1".to_string(),
        parent_task_id: None,
        target_agent_id: Some("agent-1".to_string()),
        command: "sleep".to_string(),
        payload: Some("5".to_string()),
        created_at: 1,
    });
    state.mark_task_dispatched("task-1", 2);
    state.mark_task_cancel_requested("task-1", Some("cancel requested".to_string()), 3);

    assert_eq!(
        state.active_task_ids_for_agent("agent-1"),
        vec!["task-1".to_string()]
    );
}

#[test]
fn pending_tasks_survive_restart_recovery_and_disconnect_accounting() {
    let mut state = KernelState::new();
    state.create_task(NewTask {
        task_id: "queued-task".to_string(),
        parent_task_id: None,
        target_agent_id: Some("agent-1".to_string()),
        command: "whoami".to_string(),
        payload: None,
        created_at: 1,
    });
    state.create_task(NewTask {
        task_id: "running-task".to_string(),
        parent_task_id: None,
        target_agent_id: Some("agent-1".to_string()),
        command: "hostname".to_string(),
        payload: None,
        created_at: 2,
    });
    state.mark_task_dispatched("running-task", 3);
    state.mark_task_running("running-task", 4);

    let repaired =
        state.recover_interrupted_tasks(10, "server restarted before task reached terminal state");

    assert_eq!(
        state
            .task_snapshot("queued-task")
            .expect("queued task exists")
            .status,
        TaskStatus::Pending
    );
    assert_eq!(
        state
            .task_snapshot("running-task")
            .expect("running task exists")
            .status,
        TaskStatus::Failed
    );
    assert_eq!(
        state.active_task_ids_for_agent("agent-1"),
        Vec::<String>::new()
    );
    assert_eq!(
        state.pending_task_ids_for_agent("agent-1"),
        vec!["queued-task".to_string()]
    );
    assert_eq!(repaired.len(), 1);
    assert_eq!(repaired[0].task_id, "running-task");
}

#[test]
fn cancelled_command_ignores_late_result() {
    let mut state = KernelState::new();
    state.command_sessions.insert(
        "cmdsess-1".to_string(),
        CommandSessionRecord {
            command_session_id: "cmdsess-1".to_string(),
            agent_id: "agent-1".to_string(),
            cwd: "/tmp".to_string(),
            status: CommandSessionStatus::Open,
            created_by: "tester".to_string(),
            created_at: 1,
            last_active_at: 1,
            active_command_id: Some("cmd-1".to_string()),
            queued_command_ids: VecDeque::new(),
        },
    );
    state.command_executions.insert(
        "cmd-1".to_string(),
        CommandExecutionRecord {
            command_id: "cmd-1".to_string(),
            command_session_id: "cmdsess-1".to_string(),
            agent_id: "agent-1".to_string(),
            line: "pwd".to_string(),
            status: CommandExecutionStatus::Running,
            queued_at: 1,
            updated_at: 2,
            dispatched_at: Some(2),
            started_at: Some(3),
            finished_at: None,
            cwd_before: None,
            cwd_after: None,
            exit_code: None,
            stdout: None,
            stderr: None,
            success: None,
        },
    );

    let cancelled = state
        .cancel_command_execution("cmd-1", "closed".to_string(), 4)
        .expect("command cancelled");
    assert_eq!(cancelled.status, CommandExecutionStatus::Cancelled);

    let result = state.finish_command_execute(
        "cmdsess-1",
        "cmd-1",
        "pwd".to_string(),
        "/tmp".to_string(),
        "/tmp".to_string(),
        0,
        "/tmp".to_string(),
        "".to_string(),
        true,
        5,
    );
    assert!(result.is_none());

    let snapshot = state
        .command_execution_snapshot("cmd-1")
        .expect("command exists");
    assert_eq!(snapshot.status, CommandExecutionStatus::Cancelled);
    assert_eq!(snapshot.success, Some(false));
}

#[test]
fn command_output_chunks_are_appended_to_running_command() {
    let mut state = KernelState::new();
    state.command_sessions.insert(
        "cmdsess-1".to_string(),
        CommandSessionRecord {
            command_session_id: "cmdsess-1".to_string(),
            agent_id: "agent-1".to_string(),
            cwd: "/tmp".to_string(),
            status: CommandSessionStatus::Open,
            created_by: "tester".to_string(),
            created_at: 1,
            last_active_at: 1,
            active_command_id: Some("cmd-1".to_string()),
            queued_command_ids: VecDeque::new(),
        },
    );
    state.command_executions.insert(
        "cmd-1".to_string(),
        CommandExecutionRecord {
            command_id: "cmd-1".to_string(),
            command_session_id: "cmdsess-1".to_string(),
            agent_id: "agent-1".to_string(),
            line: "echo hi".to_string(),
            status: CommandExecutionStatus::Running,
            queued_at: 1,
            updated_at: 2,
            dispatched_at: Some(2),
            started_at: Some(3),
            finished_at: None,
            cwd_before: None,
            cwd_after: None,
            exit_code: None,
            stdout: None,
            stderr: None,
            success: None,
        },
    );

    let first = state
        .append_command_output_chunk(
            "cmdsess-1",
            "cmd-1",
            &CommandOutputStream::Stdout,
            "hello ",
            4,
        )
        .expect("first chunk appended");
    assert_eq!(first.stdout.as_deref(), Some("hello "));

    let second = state
        .append_command_output_chunk(
            "cmdsess-1",
            "cmd-1",
            &CommandOutputStream::Stdout,
            "world",
            5,
        )
        .expect("second chunk appended");
    assert_eq!(second.stdout.as_deref(), Some("hello world"));

    let stderr = state
        .append_command_output_chunk(
            "cmdsess-1",
            "cmd-1",
            &CommandOutputStream::Stderr,
            "warn",
            6,
        )
        .expect("stderr chunk appended");
    assert_eq!(stderr.stderr.as_deref(), Some("warn"));
}

#[test]
fn registered_agent_heartbeat_timeout_uses_triple_interval_plus_jitter_and_grace() {
    let (_sender, receiver) = mpsc::unbounded_channel();
    let session = AgentSession {
        session_id: 1,
        agent_id: Some("agent-1".to_string()),
        listener_id: Some(1),
        listener_name: Some("default-agent-tcp".to_string()),
        hostname: Some("host-1".to_string()),
        username: Some("tester".to_string()),
        os: Some("windows".to_string()),
        arch: Some("amd64".to_string()),
        pid: Some(1001),
        internal_ip: Some("10.0.0.1".to_string()),
        tags: Vec::new(),
        sleep_interval: 30,
        jitter: 20,
        peer_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 4001)),
        connected_at: 1,
        last_seen: 1,
        sender: _sender,
        privilege: String::new(),
    };
    std::mem::forget(receiver);

    assert_eq!(session.heartbeat_timeout_ms(10_000, 10_000, 5_000), 106_000);
}

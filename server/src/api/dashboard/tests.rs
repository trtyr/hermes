use super::*;
use crate::protocol::{AgentRecord, ListenerKind, ListenerRecord, ListenerRuntimeStatus};
use serde_json::json;

#[test]
fn summarize_agents_counts_online_and_disabled() {
    let records = vec![
        AgentRecord {
            agent_id: "agent-1".to_string(),
            session_id: Some(1),
            listener_id: Some(1),
            listener_name: Some("default-agent-tcp".to_string()),
            hostname: Some("host-1".to_string()),
            username: Some("alice".to_string()),
            os: Some("macos".to_string()),
            arch: Some("aarch64".to_string()),
            pid: Some(1001),
            internal_ip: Some("10.0.0.11".to_string()),
            external_ip: Some("203.0.113.11".to_string()),
            tags: vec![],
            sleep_interval: 15,
            jitter: 0,
            peer_addr: "127.0.0.1:9001".to_string(),
            connected_at: 1,
            last_seen: 2,
            is_online: true,
            is_disabled: false,
            updated_at: 2,
            privilege: String::new(),
        },
        AgentRecord {
            agent_id: "agent-2".to_string(),
            session_id: None,
            listener_id: Some(2),
            listener_name: Some("office-tcp".to_string()),
            hostname: Some("host-2".to_string()),
            username: Some("bob".to_string()),
            os: Some("windows".to_string()),
            arch: Some("x86_64".to_string()),
            pid: Some(1002),
            internal_ip: Some("10.0.0.12".to_string()),
            external_ip: Some("203.0.113.12".to_string()),
            tags: vec![],
            sleep_interval: 30,
            jitter: 10,
            peer_addr: "127.0.0.1:9002".to_string(),
            connected_at: 3,
            last_seen: 4,
            is_online: false,
            is_disabled: true,
            updated_at: 4,
            privilege: String::new(),
        },
    ];

    let summary = summarize_agents(&records, 3);

    assert_eq!(summary.total, 2);
    assert_eq!(summary.online, 1);
    assert_eq!(summary.offline, 1);
    assert_eq!(summary.disabled, 1);
    assert_eq!(summary.connected_sessions, 3);
}

#[test]
fn summarize_listeners_counts_runtime_and_kind() {
    let listeners = vec![
        ListenerRecord {
            listener_id: 1,
            name: "tcp".to_string(),
            kind: ListenerKind::TcpJson,
            bind_host: "0.0.0.0".to_string(),
            bind_port: 1234,
            enabled: true,
            config: json!({}),
            runtime_status: ListenerRuntimeStatus::Running,
            last_error: None,
            created_at: 1,
            updated_at: 1,
        },
        ListenerRecord {
            listener_id: 2,
            name: "https".to_string(),
            kind: ListenerKind::HttpsJson,
            bind_host: "0.0.0.0".to_string(),
            bind_port: 443,
            enabled: false,
            config: json!({}),
            runtime_status: ListenerRuntimeStatus::Error,
            last_error: Some("bind failed".to_string()),
            created_at: 2,
            updated_at: 2,
        },
    ];

    let summary = summarize_listeners(&listeners);

    assert_eq!(summary.total, 2);
    assert_eq!(summary.enabled, 1);
    assert_eq!(summary.disabled, 1);
    assert_eq!(summary.running, 1);
    assert_eq!(summary.error, 1);
    assert_eq!(summary.by_kind.tcp_json, 1);
    assert_eq!(summary.by_kind.https_json, 1);
    assert_eq!(summary.by_kind.private_proto, 0);
}

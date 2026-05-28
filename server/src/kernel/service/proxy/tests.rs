use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use tokio::sync::mpsc;

use crate::{
    kernel::{
        AgentAuthConfig, AgentAuthMode, AgentKernelMessage, KernelHandle, new_kernel,
        state::{AgentIdentity, AgentSession},
    },
    protocol::{AgentMessage, ProxySessionStatus, ServerCommand},
};

static NEXT_TEST_DB_ID: AtomicU64 = AtomicU64::new(1);

fn test_db_path(name: &str) -> PathBuf {
    let id = NEXT_TEST_DB_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "hermes-server-proxy-{name}-{}-{id}.db",
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

async fn seed_connected_agent(
    kernel: &KernelHandle,
    agent_id: &str,
) -> mpsc::UnboundedReceiver<ServerCommand> {
    let now = wall_clock_now();
    let (sender, receiver) = mpsc::unbounded_channel();
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
    receiver
}

#[tokio::test]
async fn test_start_and_delete_proxy() {
    let kernel = test_kernel("proxy-lifecycle").await;
    let mut receiver = seed_connected_agent(&kernel, "agent-proxy").await;

    let facade = kernel.proxy();
    let start = tokio::spawn(async move { facade.start("agent-proxy".to_string()).await });

    let command = tokio::time::timeout(Duration::from_secs(5), receiver.recv())
        .await
        .expect("open proxy command arrives before timeout")
        .expect("agent command exists");

    let (proxy_id, bind_addr) = match command {
        ServerCommand::OpenProxy { proxy_id, bind_addr } => (proxy_id, bind_addr),
        other => panic!("unexpected command: {other:?}"),
    };

    kernel
        .send_agent_message(AgentKernelMessage::Frame {
            session_id: 1,
            frame: AgentMessage::ProxyOpened {
                proxy_id: proxy_id.clone(),
                bind_addr: bind_addr.clone(),
            },
        })
        .await
        .expect("proxy opened frame sent");

    let started = start
        .await
        .expect("start task joins")
        .expect("proxy start succeeds");
    assert_eq!(started.proxy_id, proxy_id);
    assert_eq!(started.agent_id, "agent-proxy");
    assert_eq!(started.bind_addr, bind_addr);
    assert_eq!(started.status, ProxySessionStatus::Open);

    let proxies = kernel.proxy().list_for_agent("agent-proxy").await;
    assert_eq!(proxies.len(), 1);
    assert_eq!(proxies[0].proxy_id, proxy_id);

    let deleted = kernel
        .proxy()
        .delete(proxy_id.clone())
        .await
        .expect("proxy delete succeeds");
    assert_eq!(deleted.proxy_id, proxy_id);

    let proxies = kernel.proxy().list_for_agent("agent-proxy").await;
    assert!(proxies.is_empty());
}

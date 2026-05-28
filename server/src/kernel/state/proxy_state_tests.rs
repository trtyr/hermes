use tokio::sync::{mpsc, oneshot};

use super::*;
use crate::protocol::ProxySessionStatus;

#[test]
fn proxy_session_transitions_update_snapshots_and_channels() {
    let mut state = KernelState::new();
    let (start_tx, start_rx) = oneshot::channel();
    let (stop_tx, stop_rx) = oneshot::channel();
    let (client_tx, _client_rx) = mpsc::unbounded_channel();

    state.insert_proxy_session(
        "proxy-1".to_string(),
        "agent-1".to_string(),
        "127.0.0.1:9001".to_string(),
        10,
        start_tx,
    );

    let opening = state
        .proxy_session_snapshot("proxy-1")
        .expect("opening snapshot exists");
    assert_eq!(opening.status, ProxySessionStatus::Opening);
    assert_eq!(opening.active_streams, 0);

    let open = state
        .activate_proxy_session("proxy-1", 11)
        .expect("proxy activates");
    assert_eq!(open.status, ProxySessionStatus::Open);

    let started = start_rx.blocking_recv().expect("start reply arrives");
    assert!(started.is_ok());

    state.attach_proxy_stream(
        "proxy-1",
        "stream-1".to_string(),
        "example.com".to_string(),
        443,
        client_tx,
    )
    .expect("stream attaches");

    let updated = state
        .confirm_proxy_stream_open("proxy-1", "stream-1")
        .expect("stream open confirmed");
    assert_eq!(updated.active_streams, 1);

    let after_remove = state
        .remove_proxy_stream("proxy-1", "stream-1", 12)
        .expect("stream removed");
    assert_eq!(after_remove.active_streams, 0);

    state.register_pending_proxy_stop("proxy-1".to_string(), stop_tx);
    let closed = state
        .close_proxy_session("proxy-1", 13)
        .expect("proxy closes");
    assert_eq!(closed.status, ProxySessionStatus::Closed);
    assert_eq!(closed.active_streams, 0);

    let stopped = stop_rx.blocking_recv().expect("stop reply arrives");
    assert!(stopped.is_ok());
}

#[test]
fn remove_proxy_session_cleans_streams_and_pending_channels() {
    let mut state = KernelState::new();
    let (start_tx, start_rx) = oneshot::channel::<anyhow::Result<crate::protocol::ProxySessionSnapshot>>();
    let (stop_tx, stop_rx) = oneshot::channel::<anyhow::Result<crate::protocol::ProxySessionSnapshot>>();
    let (client_tx, _client_rx) = mpsc::unbounded_channel();

    state.insert_proxy_session(
        "proxy-2".to_string(),
        "agent-2".to_string(),
        "127.0.0.1:9002".to_string(),
        20,
        start_tx,
    );
    state.register_pending_proxy_stop("proxy-2".to_string(), stop_tx);
    state
        .attach_proxy_stream(
            "proxy-2",
            "stream-2".to_string(),
            "example.org".to_string(),
            8443,
            client_tx,
        )
        .expect("stream attaches");

    let removed = state
        .remove_proxy_session("proxy-2")
        .expect("proxy removed");
    assert_eq!(removed.proxy_id, "proxy-2");
    assert!(state.proxy_session_snapshot("proxy-2").is_none());
    assert!(state.proxy_stream_sender("stream-2").is_none());

    assert!(start_rx.blocking_recv().is_err());
    assert!(stop_rx.blocking_recv().is_err());
}

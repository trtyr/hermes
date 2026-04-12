use super::*;

pub(super) async fn heartbeat_watchdog(handle: KernelHandle) {
    let mut ticker = interval(Duration::from_secs(1));
    loop {
        ticker.tick().await;
        if handle
            .send_agent_message(AgentKernelMessage::SweepHeartbeats)
            .await
            .is_err()
        {
            break;
        }
    }
}

use super::*;

pub(super) fn summarize_agents(
    agent_records: &[crate::protocol::AgentRecord],
    live_sessions: usize,
) -> AgentOverviewSummary {
    let total = agent_records.len();
    let online = agent_records
        .iter()
        .filter(|record| record.is_online)
        .count();
    let disabled = agent_records
        .iter()
        .filter(|record| record.is_disabled)
        .count();

    AgentOverviewSummary {
        total,
        online,
        offline: total.saturating_sub(online),
        disabled,
        connected_sessions: live_sessions,
    }
}

pub(super) fn summarize_listeners(
    listeners: &[crate::protocol::ListenerRecord],
) -> ListenerOverviewSummary {
    use crate::protocol::{ListenerKind, ListenerRuntimeStatus};

    let mut by_kind = ListenerKindSummary::default();
    let mut running = 0;
    let mut stopped = 0;
    let mut starting = 0;
    let mut error = 0;

    for listener in listeners {
        match listener.kind {
            ListenerKind::TcpJson => by_kind.tcp_json += 1,
            ListenerKind::HttpsJson => by_kind.https_json += 1,
            ListenerKind::PrivateProto => by_kind.private_proto += 1,
        }

        match listener.runtime_status {
            ListenerRuntimeStatus::Running => running += 1,
            ListenerRuntimeStatus::Stopped => stopped += 1,
            ListenerRuntimeStatus::Starting => starting += 1,
            ListenerRuntimeStatus::Error => error += 1,
        }
    }

    let total = listeners.len();
    let enabled = listeners.iter().filter(|listener| listener.enabled).count();

    ListenerOverviewSummary {
        total,
        enabled,
        disabled: total.saturating_sub(enabled),
        running,
        stopped,
        starting,
        error,
        by_kind,
    }
}

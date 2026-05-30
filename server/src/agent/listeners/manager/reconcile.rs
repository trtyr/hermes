use std::{collections::HashMap, time::Instant};

use crate::{
    console,
    kernel::{AgentAuthMode, KernelHandle},
    protocol::{ListenerRecord, ListenerRuntimeStatus},
};

use super::{super::registry::driver_for, ActiveListener};

pub(super) async fn reconcile_listeners(
    kernel: &KernelHandle,
    agent_token: &Option<String>,
    agent_auth_mode: AgentAuthMode,
    active: &mut HashMap<i64, ActiveListener>,
) {
    let listeners = match kernel
        .listener_queries()
        .filtered_records(None, None, None)
        .await
    {
        Ok(listeners) => listeners,
        Err(error) => {
            crate::console::storage_error("load listeners", &error);
            return;
        }
    };

    let desired = listeners
        .iter()
        .map(|listener| {
            let fingerprint = driver_for(listener.kind)
                .map(|driver| driver.fingerprint(listener))
                .unwrap_or_else(|| fallback_fingerprint(listener));
            (listener.listener_id, (fingerprint, listener.enabled))
        })
        .collect::<HashMap<_, _>>();

    stop_stale_listeners(kernel, active, &desired);

    for listener in listeners {
        start_listener_if_needed(kernel, agent_token, agent_auth_mode, active, listener);
    }
}

fn stop_stale_listeners(
    kernel: &KernelHandle,
    active: &mut HashMap<i64, ActiveListener>,
    desired: &HashMap<i64, (String, bool)>,
) {
    let active_ids = active.keys().copied().collect::<Vec<_>>();
    for listener_id in active_ids {
        let should_stop = match desired.get(&listener_id) {
            Some((fingerprint, enabled)) => {
                if let Some(current) = active.get(&listener_id) {
                    !enabled || current.fingerprint != *fingerprint
                } else {
                    false
                }
            }
            None => true,
        };
        if should_stop {
            if let Some(current) = active.remove(&listener_id) {
                current.handle.abort();
            }
            kernel.listener_commands().update_runtime_state(
                listener_id,
                ListenerRuntimeStatus::Stopped,
                None,
            );
        }
    }
}

fn start_listener_if_needed(
    kernel: &KernelHandle,
    agent_token: &Option<String>,
    agent_auth_mode: AgentAuthMode,
    active: &mut HashMap<i64, ActiveListener>,
    listener: ListenerRecord,
) {
    if !listener.enabled {
        kernel.listener_commands().update_runtime_state(
            listener.listener_id,
            ListenerRuntimeStatus::Stopped,
            None,
        );
        return;
    }

    let Some(driver) = driver_for(listener.kind) else {
        kernel.listener_commands().update_runtime_state(
            listener.listener_id,
            ListenerRuntimeStatus::Error,
            Some(format!(
                "listener kind {:?} is not implemented yet",
                listener.kind
            )),
        );
        return;
    };

    let fingerprint = driver.fingerprint(&listener);
    if let Some(current) = active.get_mut(&listener.listener_id) {
        if current.fingerprint != fingerprint {
            return;
        }

        if !current.handle.is_finished() {
            current.maybe_reset_restart_backoff();
            return;
        }

        if listener.runtime_status != ListenerRuntimeStatus::Error {
            kernel.listener_commands().update_runtime_state(
                listener.listener_id,
                ListenerRuntimeStatus::Error,
                Some("listener runtime exited unexpectedly".to_string()),
            );
        }

        if Instant::now() < current.restart_not_before {
            return;
        }

        current.note_restart();
        console::listener_restarting(listener.listener_id, &listener.name);
        kernel.listener_commands().update_runtime_state(
            listener.listener_id,
            ListenerRuntimeStatus::Starting,
            None,
        );
        current.handle = driver.spawn(
            kernel.clone(),
            listener.clone(),
            agent_token.clone(),
            agent_auth_mode,
        );
        return;
    }

    kernel.listener_commands().update_runtime_state(
        listener.listener_id,
        ListenerRuntimeStatus::Starting,
        None,
    );
    let handle = driver.spawn(
        kernel.clone(),
        listener.clone(),
        agent_token.clone(),
        agent_auth_mode,
    );
    active.insert(
        listener.listener_id,
        ActiveListener::new(fingerprint, handle),
    );
}

fn fallback_fingerprint(listener: &ListenerRecord) -> String {
    format!(
        "{:?}|{}|{}|{}|{}",
        listener.kind, listener.bind_host, listener.bind_port, listener.enabled, listener.config
    )
}

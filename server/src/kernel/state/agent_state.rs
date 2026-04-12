use super::*;

impl KernelState {
    pub fn insert_session(&mut self, session: AgentSession) {
        self.sessions.insert(session.session_id, session);
    }

    pub fn remove_session(&mut self, session_id: u64) -> Option<AgentSession> {
        let session = self.sessions.remove(&session_id)?;
        if let Some(agent_id) = &session.agent_id {
            self.agent_index.remove(agent_id);
        }
        Some(session)
    }

    pub fn existing_session_for_agent(&self, agent_id: &str) -> Option<u64> {
        self.agent_index.get(agent_id).copied()
    }

    pub fn remove_existing_session_for_agent(&mut self, agent_id: &str) -> Option<AgentSession> {
        let session_id = self.agent_index.remove(agent_id)?;
        self.sessions.remove(&session_id)
    }

    pub fn session_mut(&mut self, session_id: u64) -> Option<&mut AgentSession> {
        self.sessions.get_mut(&session_id)
    }

    pub fn session_by_agent_id(&self, agent_id: &str) -> Option<&AgentSession> {
        let session_id = self.agent_index.get(agent_id)?;
        self.sessions.get(session_id)
    }

    pub fn sessions(&self) -> impl Iterator<Item = &AgentSession> {
        self.sessions.values()
    }

    pub fn timed_out_session_ids(
        &self,
        now: u64,
        unregistered_timeout_ms: u64,
        heartbeat_grace_ms: u64,
        min_heartbeat_timeout_ms: u64,
    ) -> Vec<u64> {
        self.sessions
            .values()
            .filter(|session| {
                let timeout_ms = session.heartbeat_timeout_ms(
                    unregistered_timeout_ms,
                    heartbeat_grace_ms,
                    min_heartbeat_timeout_ms,
                );
                now.saturating_sub(session.last_seen) > timeout_ms
            })
            .map(|session| session.session_id)
            .collect()
    }

    pub fn upsert_agent_identity(
        &mut self,
        session_id: u64,
        identity: AgentIdentity,
    ) -> Option<AgentSnapshot> {
        let session = self.sessions.get_mut(&session_id)?;
        session.agent_id = Some(identity.agent_id.clone());
        session.hostname = Some(identity.hostname);
        session.username = identity.username;
        session.os = identity.os;
        session.arch = identity.arch;
        session.pid = identity.pid;
        session.internal_ip = identity.internal_ip;
        session.tags = identity.tags;
        session.sleep_interval = identity.sleep_interval;
        session.jitter = identity.jitter;
        session.last_seen = identity.last_seen;
        self.agent_index.insert(identity.agent_id, session_id);
        Some(session.snapshot())
    }

    pub fn update_last_seen(&mut self, session_id: u64, last_seen: u64) -> Option<Option<String>> {
        let session = self.sessions.get_mut(&session_id)?;
        session.last_seen = last_seen;
        Some(session.agent_id.clone())
    }

    pub fn snapshots(&self) -> Vec<AgentSnapshot> {
        self.sessions
            .values()
            .filter(|session| session.agent_id.is_some())
            .map(AgentSession::snapshot)
            .collect()
    }

    pub fn update_agent_beacon_config(
        &mut self,
        session_id: u64,
        sleep_interval: u64,
        jitter: u32,
    ) -> Option<AgentSnapshot> {
        let session = self.sessions.get_mut(&session_id)?;
        session.sleep_interval = sleep_interval;
        session.jitter = jitter;
        Some(session.snapshot())
    }

    pub fn register_pending_agent_beacon_update(
        &mut self,
        request_id: String,
        agent_id: String,
        sender: oneshot::Sender<anyhow::Result<AgentSnapshot>>,
    ) {
        self.pending_agent_beacon_updates
            .insert(request_id, PendingAgentBeaconUpdate { agent_id, sender });
    }

    pub fn complete_agent_beacon_update(
        &mut self,
        session_id: u64,
        request_id: &str,
        sleep_interval: u64,
        jitter: u32,
    ) -> Option<AgentSnapshot> {
        let snapshot = self.update_agent_beacon_config(session_id, sleep_interval, jitter)?;
        if let Some(pending) = self.pending_agent_beacon_updates.remove(request_id) {
            let _ = pending.sender.send(Ok(snapshot.clone()));
        }
        Some(snapshot)
    }

    pub fn abort_pending_agent_beacon_update(&mut self, request_id: &str) -> bool {
        self.pending_agent_beacon_updates
            .remove(request_id)
            .is_some()
    }

    pub fn fail_pending_agent_beacon_updates_for_agent(&mut self, agent_id: &str, reason: &str) {
        let pending_ids = self
            .pending_agent_beacon_updates
            .iter()
            .filter(|(_, pending)| pending.agent_id == agent_id)
            .map(|(request_id, _)| request_id.clone())
            .collect::<Vec<_>>();

        for request_id in pending_ids {
            if let Some(pending) = self.pending_agent_beacon_updates.remove(&request_id) {
                let _ = pending
                    .sender
                    .send(Err(anyhow::anyhow!(reason.to_string())));
            }
        }
    }
}

impl AgentSession {
    pub fn heartbeat_timeout_ms(
        &self,
        unregistered_timeout_ms: u64,
        heartbeat_grace_ms: u64,
        min_heartbeat_timeout_ms: u64,
    ) -> u64 {
        if self.agent_id.is_none() || self.sleep_interval == 0 {
            return unregistered_timeout_ms.max(min_heartbeat_timeout_ms);
        }

        let base_ms = self.sleep_interval.saturating_mul(1000);
        let jitter_factor = 100_u64.saturating_add(self.jitter as u64);
        let max_expected_ms = base_ms.saturating_mul(jitter_factor) / 100;
        max_expected_ms
            .saturating_add(heartbeat_grace_ms)
            .max(min_heartbeat_timeout_ms)
    }

    pub fn snapshot(&self) -> AgentSnapshot {
        AgentSnapshot {
            session_id: self.session_id,
            agent_id: self.agent_id.clone(),
            listener_id: self.listener_id,
            listener_name: self.listener_name.clone(),
            hostname: self.hostname.clone(),
            username: self.username.clone(),
            os: self.os.clone(),
            arch: self.arch.clone(),
            pid: self.pid,
            internal_ip: self.internal_ip.clone(),
            external_ip: Some(self.peer_addr.ip().to_string()),
            tags: self.tags.clone(),
            sleep_interval: self.sleep_interval,
            jitter: self.jitter,
            peer_addr: self.peer_addr.to_string(),
            connected_at: self.connected_at,
            last_seen: self.last_seen,
        }
    }
}

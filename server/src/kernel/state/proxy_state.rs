use super::*;

impl KernelState {
    pub fn proxy_session_snapshot(&self, proxy_id: &str) -> Option<ProxySessionSnapshot> {
        self.proxy_sessions.get(proxy_id).map(|record| ProxySessionSnapshot {
            proxy_id: record.proxy_id.clone(),
            agent_id: record.agent_id.clone(),
            bind_addr: record.bind_addr.clone(),
            status: record.status,
            active_streams: record.active_stream_ids.len(),
            created_at: record.created_at,
            updated_at: record.updated_at,
            last_error: record.last_error.clone(),
        })
    }

    pub fn proxy_session_snapshots_for_agent(&self, agent_id: &str) -> Vec<ProxySessionSnapshot> {
        let mut items = self
            .proxy_sessions
            .values()
            .filter(|record| record.agent_id == agent_id)
            .map(|record| ProxySessionSnapshot {
                proxy_id: record.proxy_id.clone(),
                agent_id: record.agent_id.clone(),
                bind_addr: record.bind_addr.clone(),
                status: record.status,
                active_streams: record.active_stream_ids.len(),
                created_at: record.created_at,
                updated_at: record.updated_at,
                last_error: record.last_error.clone(),
            })
            .collect::<Vec<_>>();
        items.sort_by(|a, b| b.created_at.cmp(&a.created_at).then_with(|| a.proxy_id.cmp(&b.proxy_id)));
        items
    }

    pub fn insert_proxy_session(
        &mut self,
        proxy_id: String,
        agent_id: String,
        bind_addr: String,
        created_at: u64,
        sender: oneshot::Sender<anyhow::Result<ProxySessionSnapshot>>,
    ) {
        self.pending_proxy_session_starts.insert(proxy_id.clone(), sender);
        self.proxy_sessions.insert(
            proxy_id.clone(),
            ProxySessionRecord {
                proxy_id,
                agent_id,
                bind_addr,
                status: ProxySessionStatus::Opening,
                active_stream_ids: HashSet::new(),
                created_at,
                updated_at: created_at,
                last_error: None,
            },
        );
    }

    pub fn activate_proxy_session(&mut self, proxy_id: &str, updated_at: u64) -> Option<ProxySessionSnapshot> {
        let record = self.proxy_sessions.get_mut(proxy_id)?;
        record.status = ProxySessionStatus::Open;
        record.updated_at = updated_at;
        let snapshot = self.proxy_session_snapshot(proxy_id)?;
        if let Some(sender) = self.pending_proxy_session_starts.remove(proxy_id) {
            let _ = sender.send(Ok(snapshot.clone()));
        }
        Some(snapshot)
    }

    pub fn register_pending_proxy_stop(
        &mut self,
        proxy_id: String,
        sender: oneshot::Sender<anyhow::Result<ProxySessionSnapshot>>,
    ) {
        self.pending_proxy_session_stops.insert(proxy_id, sender);
    }

    pub fn close_proxy_session(&mut self, proxy_id: &str, updated_at: u64) -> Option<ProxySessionSnapshot> {
        let record = self.proxy_sessions.get_mut(proxy_id)?;
        record.status = ProxySessionStatus::Closed;
        record.updated_at = updated_at;
        let stream_ids = record.active_stream_ids.clone().into_iter().collect::<Vec<_>>();
        for stream_id in stream_ids {
            self.proxy_streams.remove(&stream_id);
            record.active_stream_ids.remove(&stream_id);
        }
        let snapshot = self.proxy_session_snapshot(proxy_id)?;
        if let Some(sender) = self.pending_proxy_session_stops.remove(proxy_id) {
            let _ = sender.send(Ok(snapshot.clone()));
        }
        Some(snapshot)
    }

    pub fn fail_pending_proxy_session_starts_for_agent(&mut self, agent_id: &str, reason: &str) {
        let proxy_ids = self
            .proxy_sessions
            .values()
            .filter(|record| record.agent_id == agent_id)
            .map(|record| record.proxy_id.clone())
            .collect::<Vec<_>>();
        for proxy_id in proxy_ids {
            if let Some(sender) = self.pending_proxy_session_starts.remove(&proxy_id) {
                let _ = sender.send(Err(anyhow::anyhow!(reason.to_string())));
            }
            if let Some(sender) = self.pending_proxy_session_stops.remove(&proxy_id) {
                let _ = sender.send(Err(anyhow::anyhow!(reason.to_string())));
            }
        }
    }

    pub fn register_pending_proxy_stream_open(
        &mut self,
        stream_id: String,
        sender: oneshot::Sender<anyhow::Result<()>>,
    ) {
        self.pending_proxy_stream_opens.insert(stream_id, sender);
    }

    pub fn attach_proxy_stream(
        &mut self,
        proxy_id: &str,
        stream_id: String,
        target_host: String,
        target_port: u16,
        client_sender: mpsc::UnboundedSender<Option<Vec<u8>>>,
    ) -> anyhow::Result<()> {
        let record = self
            .proxy_sessions
            .get_mut(proxy_id)
            .ok_or_else(|| anyhow::anyhow!("proxy session not found"))?;
        record.active_stream_ids.insert(stream_id.clone());
        self.proxy_streams.insert(
            stream_id.clone(),
            ProxyStreamRecord {
                stream_id,
                proxy_id: proxy_id.to_string(),
                target_host,
                target_port,
                client_sender,
            },
        );
        Ok(())
    }

    pub fn confirm_proxy_stream_open(&mut self, proxy_id: &str, stream_id: &str) -> Option<ProxySessionSnapshot> {
        if let Some(sender) = self.pending_proxy_stream_opens.remove(stream_id) {
            let _ = sender.send(Ok(()));
        }
        let record = self.proxy_sessions.get_mut(proxy_id)?;
        record.updated_at = std::cmp::max(record.updated_at, 1);
        self.proxy_session_snapshot(proxy_id)
    }

    pub fn fail_proxy_stream_open(&mut self, proxy_id: &str, stream_id: &str, detail: String, updated_at: u64) -> Option<ProxySessionSnapshot> {
        self.proxy_streams.remove(stream_id);
        if let Some(sender) = self.pending_proxy_stream_opens.remove(stream_id) {
            let _ = sender.send(Err(anyhow::anyhow!(detail.clone())));
        }
        let record = self.proxy_sessions.get_mut(proxy_id)?;
        record.active_stream_ids.remove(stream_id);
        record.updated_at = updated_at;
        record.last_error = Some(detail);
        self.proxy_session_snapshot(proxy_id)
    }

    pub fn proxy_stream_sender(&self, stream_id: &str) -> Option<mpsc::UnboundedSender<Option<Vec<u8>>>> {
        self.proxy_streams.get(stream_id).map(|record| record.client_sender.clone())
    }

    pub fn remove_proxy_stream(&mut self, proxy_id: &str, stream_id: &str, updated_at: u64) -> Option<ProxySessionSnapshot> {
        self.proxy_streams.remove(stream_id);
        let record = self.proxy_sessions.get_mut(proxy_id)?;
        record.active_stream_ids.remove(stream_id);
        record.updated_at = updated_at;
        self.proxy_session_snapshot(proxy_id)
    }

    pub fn proxy_agent_id(&self, proxy_id: &str) -> Option<String> {
        self.proxy_sessions.get(proxy_id).map(|record| record.agent_id.clone())
    }

    pub fn proxy_session_ids_for_agent(&self, agent_id: &str) -> Vec<String> {
        self.proxy_sessions
            .values()
            .filter(|record| record.agent_id == agent_id)
            .map(|record| record.proxy_id.clone())
            .collect()
    }
}

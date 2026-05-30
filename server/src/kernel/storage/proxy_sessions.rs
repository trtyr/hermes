use super::*;

use crate::protocol::ProxySessionStatus;

impl Storage {
    pub fn persist_proxy_session(
        &self,
        proxy_id: &str,
        agent_id: &str,
        bind_addr: &str,
        status: &ProxySessionStatus,
        active_streams: usize,
        last_error: Option<&str>,
        created_at: u64,
        updated_at: u64,
    ) {
        let result = (|| -> anyhow::Result<()> {
            let connection = open_connection(&self.sqlite_path)?;
            connection.execute(
                "INSERT OR REPLACE INTO agent_proxy_sessions \
                 (proxy_id, agent_id, bind_addr, status, active_streams, last_error, created_at, updated_at) \
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    proxy_id,
                    agent_id,
                    bind_addr,
                    encode_proxy_status(status),
                    active_streams as i64,
                    last_error,
                    created_at,
                    updated_at,
                ],
            )?;
            Ok(())
        })();

        if let Err(error) = result {
            crate::console::storage_error("persist proxy session", &error);
        }
    }

    pub fn delete_proxy_session(&self, proxy_id: &str) {
        let result = (|| -> anyhow::Result<()> {
            let connection = open_connection(&self.sqlite_path)?;
            connection.execute(
                "DELETE FROM agent_proxy_sessions WHERE proxy_id = ?1",
                params![proxy_id],
            )?;
            Ok(())
        })();

        if let Err(error) = result {
            crate::console::storage_error("delete proxy session", &error);
        }
    }
}

pub struct DbProxySession {
    pub proxy_id: String,
    pub agent_id: String,
    pub bind_addr: String,
    pub status: String,
    pub active_streams: u32,
    pub last_error: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

pub(super) fn encode_proxy_status(status: &ProxySessionStatus) -> &'static str {
    match status {
        ProxySessionStatus::Opening => "opening",
        ProxySessionStatus::Open => "open",
        ProxySessionStatus::Closed => "closed",
        ProxySessionStatus::Error => "error",
    }
}

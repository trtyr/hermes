use super::*;

impl Storage {
    pub fn persist_agent_snapshot(&self, agent: AgentSnapshot, is_online: bool) {
        let result = (|| -> anyhow::Result<()> {
            let Some(agent_id) = agent.agent_id.clone() else {
                return Ok(());
            };

            let connection = open_connection(&self.sqlite_path)?;
            connection.execute(
                "INSERT INTO agents (
                    agent_id, session_id, listener_id, listener_name, hostname, username, os, arch, pid, internal_ip,
                    tags_json, sleep_interval, jitter, peer_addr, connected_at, last_seen,
                    is_online, is_disabled, elevated, updated_at
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, 0, ?18, ?19)
                 ON CONFLICT(agent_id) DO UPDATE SET
                    session_id = excluded.session_id,
                    listener_id = excluded.listener_id,
                    listener_name = excluded.listener_name,
                    hostname = excluded.hostname,
                    username = excluded.username,
                    os = excluded.os,
                    arch = excluded.arch,
                    pid = excluded.pid,
                    internal_ip = excluded.internal_ip,
                    tags_json = excluded.tags_json,
                    sleep_interval = excluded.sleep_interval,
                    jitter = excluded.jitter,
                    peer_addr = excluded.peer_addr,
                    connected_at = excluded.connected_at,
                    last_seen = excluded.last_seen,
                    is_online = excluded.is_online,
                    elevated = excluded.elevated,
                    updated_at = excluded.updated_at",
                params![
                    agent_id,
                    agent.session_id,
                    agent.listener_id,
                    agent.listener_name,
                    agent.hostname,
                    agent.username,
                    agent.os,
                    agent.arch,
                    agent.pid,
                    agent.internal_ip,
                    serde_json::to_string(&agent.tags)?,
                    agent.sleep_interval,
                    agent.jitter,
                    agent.peer_addr,
                    agent.connected_at,
                    agent.last_seen,
                    if is_online { 1 } else { 0 },
                    if agent.elevated { 1 } else { 0 },
                    agent.last_seen,
                ],
            )?;
            Ok(())
        })();

        if let Err(error) = result {
            eprintln!("Failed to persist agent snapshot: {}", error);
        }
    }

    pub fn mark_agent_offline(&self, agent_id: String, updated_at: u64) {
        let result = (|| -> anyhow::Result<()> {
            let connection = open_connection(&self.sqlite_path)?;
            connection.execute(
                "UPDATE agents
                 SET is_online = 0, updated_at = ?2
                 WHERE agent_id = ?1",
                params![agent_id, updated_at],
            )?;
            Ok(())
        })();

        if let Err(error) = result {
            eprintln!("Failed to mark agent offline: {}", error);
        }
    }

    pub async fn filtered_agent_records(
        &self,
        filter: AgentRecordFilter,
    ) -> anyhow::Result<Vec<AgentRecord>> {
        let path = self.sqlite_path.clone();
        tokio::task::spawn_blocking(move || {
            let connection = open_connection(&path)?;
            let mut records = load_agent_records(&connection)?;

            if let Some(online) = filter.online {
                records.retain(|record| record.is_online == online);
            }

            if let Some(disabled) = filter.disabled {
                records.retain(|record| record.is_disabled == disabled);
            }

            if let Some(keyword) = filter.keyword {
                let keyword = keyword.to_lowercase();
                records.retain(|record| {
                    record.agent_id.to_lowercase().contains(&keyword)
                        || record
                            .hostname
                            .as_deref()
                            .unwrap_or_default()
                            .to_lowercase()
                            .contains(&keyword)
                        || record
                            .username
                            .as_deref()
                            .unwrap_or_default()
                            .to_lowercase()
                            .contains(&keyword)
                        || record
                            .os
                            .as_deref()
                            .unwrap_or_default()
                            .to_lowercase()
                            .contains(&keyword)
                        || record
                            .arch
                            .as_deref()
                            .unwrap_or_default()
                            .to_lowercase()
                            .contains(&keyword)
                        || record
                            .internal_ip
                            .as_deref()
                            .unwrap_or_default()
                            .to_lowercase()
                            .contains(&keyword)
                        || record
                            .external_ip
                            .as_deref()
                            .unwrap_or_default()
                            .to_lowercase()
                            .contains(&keyword)
                        || record.peer_addr.to_lowercase().contains(&keyword)
                });
            }

            if let Some(tag) = filter.tag {
                let tag = tag.to_lowercase();
                records.retain(|record| record.tags.iter().any(|item| item.to_lowercase() == tag));
            }

            Ok(records)
        })
        .await
        .context("sqlite filtered agent records join error")?
    }

    pub async fn agent_record(&self, agent_id: &str) -> anyhow::Result<Option<AgentRecord>> {
        let path = self.sqlite_path.clone();
        let agent_id = agent_id.to_string();
        tokio::task::spawn_blocking(move || {
            let connection = open_connection(&path)?;
            let mut statement = connection.prepare(
                "SELECT agent_id, session_id, listener_id, listener_name, hostname, username, os, arch, pid, internal_ip,
                        tags_json, sleep_interval, jitter, peer_addr, connected_at, last_seen,
                        is_online, is_disabled, elevated, updated_at
                 FROM agents
                 WHERE agent_id = ?1",
            )?;

            let mut rows = statement.query(params![agent_id])?;
            let Some(row) = rows.next()? else {
                return Ok(None);
            };

            let tags_json: String = row.get(10)?;
            let peer_addr: String = row.get(13)?;
            Ok(Some(AgentRecord {
                agent_id: row.get(0)?,
                session_id: row.get(1)?,
                listener_id: row.get(2)?,
                listener_name: row.get(3)?,
                hostname: row.get(4)?,
                username: row.get(5)?,
                os: row.get(6)?,
                arch: row.get(7)?,
                pid: row.get(8)?,
                internal_ip: row.get(9)?,
                external_ip: derive_external_ip(&peer_addr),
                tags: serde_json::from_str(&tags_json).unwrap_or_default(),
                sleep_interval: row.get(11)?,
                jitter: row.get(12)?,
                peer_addr,
                connected_at: row.get(14)?,
                last_seen: row.get(15)?,
                is_online: row.get::<_, i64>(16)? != 0,
                is_disabled: row.get::<_, i64>(17)? != 0,
                elevated: row.get::<_, i64>(18)? != 0,
                updated_at: row.get(19)?,
            }))
        })
        .await
        .context("sqlite agent record join error")?
    }

    pub async fn set_agent_disabled(&self, agent_id: &str, disabled: bool) -> anyhow::Result<bool> {
        let path = self.sqlite_path.clone();
        let agent_id = agent_id.to_string();
        tokio::task::spawn_blocking(move || {
            let connection = open_connection(&path)?;
            let rows = connection.execute(
                "UPDATE agents
                 SET is_disabled = ?2, updated_at = ?3
                 WHERE agent_id = ?1",
                params![agent_id, if disabled { 1 } else { 0 }, now_ts()],
            )?;
            Ok(rows > 0)
        })
        .await
        .context("sqlite set agent disabled join error")?
    }

    pub async fn delete_agent_record(&self, agent_id: &str) -> anyhow::Result<bool> {
        let path = self.sqlite_path.clone();
        let agent_id = agent_id.to_string();
        tokio::task::spawn_blocking(move || {
            let connection = open_connection(&path)?;
            let rows = connection.execute(
                "DELETE FROM agents
                 WHERE agent_id = ?1",
                params![agent_id],
            )?;
            Ok(rows > 0)
        })
        .await
        .context("sqlite delete agent record join error")?
    }

    pub async fn update_agent_tags(&self, agent_id: &str, tags: &[String]) -> anyhow::Result<bool> {
        let path = self.sqlite_path.clone();
        let agent_id = agent_id.to_string();
        let tags_json = serde_json::to_string(tags)?;
        tokio::task::spawn_blocking(move || {
            let connection = open_connection(&path)?;
            let rows = connection.execute(
                "UPDATE agents SET tags_json = ?2, updated_at = ?3 WHERE agent_id = ?1",
                params![agent_id, tags_json, now_ts()],
            )?;
            Ok(rows > 0)
        })
        .await
        .context("sqlite update agent tags join error")?
    }
}

fn load_agent_records(connection: &Connection) -> anyhow::Result<Vec<AgentRecord>> {
    let mut statement = connection.prepare(
        "SELECT agent_id, session_id, listener_id, listener_name, hostname, username, os, arch, pid, internal_ip,
                tags_json, sleep_interval, jitter, peer_addr, connected_at, last_seen,
                is_online, is_disabled, elevated, updated_at
         FROM agents
         ORDER BY updated_at DESC, agent_id ASC",
    )?;

    let records = statement
        .query_map([], |row| {
            let tags_json: String = row.get(10)?;
            let peer_addr: String = row.get(13)?;
            Ok(AgentRecord {
                agent_id: row.get(0)?,
                session_id: row.get(1)?,
                listener_id: row.get(2)?,
                listener_name: row.get(3)?,
                hostname: row.get(4)?,
                username: row.get(5)?,
                os: row.get(6)?,
                arch: row.get(7)?,
                pid: row.get(8)?,
                internal_ip: row.get(9)?,
                external_ip: derive_external_ip(&peer_addr),
                tags: serde_json::from_str(&tags_json).unwrap_or_default(),
                sleep_interval: row.get(11)?,
                jitter: row.get(12)?,
                peer_addr,
                connected_at: row.get(14)?,
                last_seen: row.get(15)?,
                is_online: row.get::<_, i64>(16)? != 0,
                is_disabled: row.get::<_, i64>(17)? != 0,
                elevated: row.get::<_, i64>(18)? != 0,
                updated_at: row.get(19)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(records)
}

fn derive_external_ip(peer_addr: &str) -> Option<String> {
    peer_addr
        .parse::<std::net::SocketAddr>()
        .ok()
        .map(|addr| addr.ip().to_string())
}

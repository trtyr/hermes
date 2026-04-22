use super::*;

impl Storage {
    pub async fn filtered_agent_build_records(
        &self,
        filter: AgentBuildRecordFilter,
    ) -> anyhow::Result<Vec<AgentBuildRecord>> {
        let path = self.sqlite_path.clone();
        tokio::task::spawn_blocking(move || {
            let connection = open_connection(&path)?;
            let mut records = load_agent_build_records(&connection)?;

            if let Some(status) = filter.status {
                records.retain(|record| record.status == status);
            }

            if let Some(target_triple) = filter.target_triple {
                let target_triple = target_triple.to_lowercase();
                records.retain(|record| record.target_triple.to_lowercase() == target_triple);
            }

            Ok(records)
        })
        .await
        .context("sqlite filtered agent build records join error")?
    }

    pub async fn agent_build_record(
        &self,
        build_id: i64,
    ) -> anyhow::Result<Option<AgentBuildRecord>> {
        let path = self.sqlite_path.clone();
        tokio::task::spawn_blocking(move || -> anyhow::Result<Option<AgentBuildRecord>> {
            let connection = open_connection(&path)?;
            load_agent_build_record_by_id(&connection, build_id)
        })
        .await
        .context("sqlite agent build record join error")?
    }

    pub async fn create_agent_build_record(
        &self,
        target_triple: String,
        profile: String,
        listener_id: Option<i64>,
        server_addr: String,
        embedded_agent_token: bool,
    ) -> anyhow::Result<AgentBuildRecord> {
        let path = self.sqlite_path.clone();
        tokio::task::spawn_blocking(move || -> anyhow::Result<AgentBuildRecord> {
            let connection = open_connection(&path)?;
            let now = now_ts();
            connection.execute(
                "INSERT INTO agent_builds (
                    target_triple, profile, listener_id, server_addr, embedded_agent_token,
                    artifact_path, artifact_name, status, detail, created_at, updated_at
                 ) VALUES (?1, ?2, ?3, ?4, ?5, NULL, NULL, ?6, NULL, ?7, ?7)",
                params![
                    target_triple,
                    profile,
                    listener_id,
                    server_addr,
                    if embedded_agent_token { 1 } else { 0 },
                    encode_agent_build_status(AgentBuildStatus::Pending),
                    now,
                ],
            )?;
            let build_id = connection.last_insert_rowid();
            load_agent_build_record_by_id(&connection, build_id)?
                .ok_or_else(|| anyhow::anyhow!("created agent build {} missing", build_id))
        })
        .await
        .context("sqlite create agent build record join error")?
    }

    pub async fn delete_agent_build_record(&self, build_id: i64) -> anyhow::Result<bool> {
        let path = self.sqlite_path.clone();
        tokio::task::spawn_blocking(move || {
            let connection = open_connection(&path)?;
            let rows = connection.execute(
                "DELETE FROM agent_builds WHERE build_id = ?1",
                params![build_id],
            )?;
            Ok(rows > 0)
        })
        .await
        .context("sqlite delete agent build record join error")?
    }

    pub async fn update_agent_build_record(
        &self,
        build_id: i64,
        status: AgentBuildStatus,
        artifact_path: Option<String>,
        artifact_name: Option<String>,
        detail: Option<String>,
    ) -> anyhow::Result<Option<AgentBuildRecord>> {
        let path = self.sqlite_path.clone();
        tokio::task::spawn_blocking(move || -> anyhow::Result<Option<AgentBuildRecord>> {
            let connection = open_connection(&path)?;
            let rows = connection.execute(
                "UPDATE agent_builds
                 SET status = ?2,
                     artifact_path = COALESCE(?3, artifact_path),
                     artifact_name = COALESCE(?4, artifact_name),
                     detail = ?5,
                     updated_at = ?6
                 WHERE build_id = ?1",
                params![
                    build_id,
                    encode_agent_build_status(status),
                    artifact_path,
                    artifact_name,
                    detail,
                    now_ts(),
                ],
            )?;
            if rows == 0 {
                return Ok(None);
            }
            load_agent_build_record_by_id(&connection, build_id)
        })
        .await
        .context("sqlite update agent build record join error")?
    }
}

fn load_agent_build_records(connection: &Connection) -> anyhow::Result<Vec<AgentBuildRecord>> {
    let mut statement = connection.prepare(
        "SELECT build_id, target_triple, profile, listener_id, server_addr, embedded_agent_token,
                artifact_path, artifact_name, status, detail, created_at, updated_at
         FROM agent_builds
         ORDER BY build_id DESC",
    )?;

    let records = statement
        .query_map([], decode_agent_build_row)?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(records)
}

fn load_agent_build_record_by_id(
    connection: &Connection,
    build_id: i64,
) -> anyhow::Result<Option<AgentBuildRecord>> {
    let mut statement = connection.prepare(
        "SELECT build_id, target_triple, profile, listener_id, server_addr, embedded_agent_token,
                artifact_path, artifact_name, status, detail, created_at, updated_at
         FROM agent_builds
         WHERE build_id = ?1",
    )?;
    let mut rows = statement.query(params![build_id])?;
    let Some(row) = rows.next()? else {
        return Ok(None);
    };
    Ok(Some(decode_agent_build_row(row)?))
}

fn decode_agent_build_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<AgentBuildRecord> {
    let status_raw: String = row.get(8)?;
    Ok(AgentBuildRecord {
        build_id: row.get(0)?,
        target_triple: row.get(1)?,
        profile: row.get(2)?,
        listener_id: row.get(3)?,
        server_addr: row.get(4)?,
        embedded_agent_token: row.get::<_, i64>(5)? != 0,
        artifact_path: row.get(6)?,
        artifact_name: row.get(7)?,
        status: decode_agent_build_status(&status_raw),
        detail: row.get(9)?,
        created_at: row.get(10)?,
        updated_at: row.get(11)?,
    })
}

fn encode_agent_build_status(status: AgentBuildStatus) -> &'static str {
    match status {
        AgentBuildStatus::Pending => "pending",
        AgentBuildStatus::Succeeded => "succeeded",
        AgentBuildStatus::Failed => "failed",
    }
}

fn decode_agent_build_status(raw: &str) -> AgentBuildStatus {
    match raw {
        "pending" => AgentBuildStatus::Pending,
        "succeeded" => AgentBuildStatus::Succeeded,
        "failed" => AgentBuildStatus::Failed,
        _ => AgentBuildStatus::Failed,
    }
}

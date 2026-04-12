use super::*;

impl Storage {
    pub async fn bootstrap_default_listener(
        &self,
        name: &str,
        bind_host: &str,
        bind_port: u16,
    ) -> anyhow::Result<()> {
        let path = self.sqlite_path.clone();
        let name = name.to_string();
        let bind_host = bind_host.to_string();
        tokio::task::spawn_blocking(move || {
            let connection = open_connection(&path)?;
            let count: i64 =
                connection.query_row("SELECT COUNT(*) FROM listeners", [], |row| row.get(0))?;
            if count > 0 {
                return Ok(());
            }

            let now = now_ts();
            connection.execute(
                "INSERT INTO listeners (
                    name, kind, bind_host, bind_port, enabled, config_json,
                    runtime_status, last_error, created_at, updated_at
                 ) VALUES (?1, ?2, ?3, ?4, 1, ?5, ?6, NULL, ?7, ?7)",
                params![
                    name,
                    encode_listener_kind(ListenerKind::TcpJson),
                    bind_host,
                    bind_port,
                    "{}",
                    encode_listener_runtime_status(ListenerRuntimeStatus::Stopped),
                    now,
                ],
            )?;
            Ok(())
        })
        .await
        .context("sqlite bootstrap default listener join error")?
    }

    pub async fn filtered_listener_records(
        &self,
        filter: ListenerRecordFilter,
    ) -> anyhow::Result<Vec<ListenerRecord>> {
        let path = self.sqlite_path.clone();
        tokio::task::spawn_blocking(move || {
            let connection = open_connection(&path)?;
            let mut records = load_listener_records(&connection)?;

            if let Some(enabled) = filter.enabled {
                records.retain(|record| record.enabled == enabled);
            }

            if let Some(kind) = filter.kind {
                records.retain(|record| record.kind == kind);
            }

            if let Some(keyword) = filter.keyword {
                let keyword = keyword.to_lowercase();
                records.retain(|record| {
                    record.name.to_lowercase().contains(&keyword)
                        || record.bind_host.to_lowercase().contains(&keyword)
                        || format!("{}:{}", record.bind_host, record.bind_port)
                            .to_lowercase()
                            .contains(&keyword)
                });
            }

            Ok(records)
        })
        .await
        .context("sqlite filtered listener records join error")?
    }

    pub async fn listener_record(
        &self,
        listener_id: i64,
    ) -> anyhow::Result<Option<ListenerRecord>> {
        let path = self.sqlite_path.clone();
        tokio::task::spawn_blocking(move || -> anyhow::Result<Option<ListenerRecord>> {
            let connection = open_connection(&path)?;
            let mut statement = connection.prepare(
                "SELECT listener_id, name, kind, bind_host, bind_port, enabled, config_json,
                        runtime_status, last_error, created_at, updated_at
                 FROM listeners
                 WHERE listener_id = ?1",
            )?;
            let mut rows = statement.query(params![listener_id])?;
            let Some(row) = rows.next()? else {
                return Ok(None);
            };
            Ok(Some(decode_listener_row(row)?))
        })
        .await
        .context("sqlite listener record join error")?
    }

    pub async fn create_listener_record(
        &self,
        name: String,
        kind: ListenerKind,
        bind_host: String,
        bind_port: u16,
        enabled: bool,
        config: serde_json::Value,
    ) -> anyhow::Result<ListenerRecord> {
        let path = self.sqlite_path.clone();
        tokio::task::spawn_blocking(move || {
            let connection = open_connection(&path)?;
            let now = now_ts();
            connection.execute(
                "INSERT INTO listeners (
                    name, kind, bind_host, bind_port, enabled, config_json,
                    runtime_status, last_error, created_at, updated_at
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, NULL, ?8, ?8)",
                params![
                    name,
                    encode_listener_kind(kind),
                    bind_host,
                    bind_port,
                    if enabled { 1 } else { 0 },
                    serde_json::to_string(&config)?,
                    encode_listener_runtime_status(ListenerRuntimeStatus::Stopped),
                    now,
                ],
            )?;
            let listener_id = connection.last_insert_rowid();
            load_listener_record_by_id(&connection, listener_id)?
                .ok_or_else(|| anyhow::anyhow!("created listener {} missing", listener_id))
        })
        .await
        .context("sqlite create listener record join error")?
    }

    pub async fn update_listener_record(
        &self,
        listener_id: i64,
        name: Option<String>,
        bind_host: Option<String>,
        bind_port: Option<u16>,
        config: Option<serde_json::Value>,
    ) -> anyhow::Result<Option<ListenerRecord>> {
        let path = self.sqlite_path.clone();
        tokio::task::spawn_blocking(move || {
            let connection = open_connection(&path)?;
            let Some(current) = load_listener_record_by_id(&connection, listener_id)? else {
                return Ok(None);
            };
            let updated_name = name.unwrap_or(current.name);
            let updated_bind_host = bind_host.unwrap_or(current.bind_host);
            let updated_bind_port = bind_port.unwrap_or(current.bind_port);
            let updated_config = config.unwrap_or(current.config);
            let now = now_ts();

            connection.execute(
                "UPDATE listeners
                 SET name = ?2,
                     bind_host = ?3,
                     bind_port = ?4,
                     config_json = ?5,
                     updated_at = ?6
                 WHERE listener_id = ?1",
                params![
                    listener_id,
                    updated_name,
                    updated_bind_host,
                    updated_bind_port,
                    serde_json::to_string(&updated_config)?,
                    now,
                ],
            )?;
            load_listener_record_by_id(&connection, listener_id)
        })
        .await
        .context("sqlite update listener record join error")?
    }

    pub async fn set_listener_enabled(
        &self,
        listener_id: i64,
        enabled: bool,
    ) -> anyhow::Result<Option<ListenerRecord>> {
        let path = self.sqlite_path.clone();
        tokio::task::spawn_blocking(move || {
            let connection = open_connection(&path)?;
            let now = now_ts();
            let rows = connection.execute(
                "UPDATE listeners
                 SET enabled = ?2,
                     runtime_status = ?3,
                     last_error = NULL,
                     updated_at = ?4
                 WHERE listener_id = ?1",
                params![
                    listener_id,
                    if enabled { 1 } else { 0 },
                    encode_listener_runtime_status(ListenerRuntimeStatus::Stopped),
                    now,
                ],
            )?;
            if rows == 0 {
                return Ok(None);
            }
            load_listener_record_by_id(&connection, listener_id)
        })
        .await
        .context("sqlite set listener enabled join error")?
    }

    pub async fn delete_listener_record(&self, listener_id: i64) -> anyhow::Result<bool> {
        let path = self.sqlite_path.clone();
        tokio::task::spawn_blocking(move || {
            let connection = open_connection(&path)?;
            let rows = connection.execute(
                "DELETE FROM listeners
                 WHERE listener_id = ?1",
                params![listener_id],
            )?;
            Ok(rows > 0)
        })
        .await
        .context("sqlite delete listener record join error")?
    }

    pub fn update_listener_runtime_state(
        &self,
        listener_id: i64,
        runtime_status: ListenerRuntimeStatus,
        last_error: Option<String>,
    ) {
        let result = (|| -> anyhow::Result<()> {
            let connection = open_connection(&self.sqlite_path)?;
            connection.execute(
                "UPDATE listeners
                 SET runtime_status = ?2,
                     last_error = ?3,
                     updated_at = ?4
                 WHERE listener_id = ?1",
                params![
                    listener_id,
                    encode_listener_runtime_status(runtime_status),
                    last_error,
                    now_ts(),
                ],
            )?;
            Ok(())
        })();

        if let Err(error) = result {
            eprintln!("Failed to update listener runtime state: {}", error);
        }
    }
}

fn load_listener_records(connection: &Connection) -> anyhow::Result<Vec<ListenerRecord>> {
    let mut statement = connection.prepare(
        "SELECT listener_id, name, kind, bind_host, bind_port, enabled, config_json,
                runtime_status, last_error, created_at, updated_at
         FROM listeners
         ORDER BY listener_id ASC",
    )?;

    let records = statement
        .query_map([], decode_listener_row)?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(records)
}

fn load_listener_record_by_id(
    connection: &Connection,
    listener_id: i64,
) -> anyhow::Result<Option<ListenerRecord>> {
    let mut statement = connection.prepare(
        "SELECT listener_id, name, kind, bind_host, bind_port, enabled, config_json,
                runtime_status, last_error, created_at, updated_at
         FROM listeners
         WHERE listener_id = ?1",
    )?;
    let mut rows = statement.query(params![listener_id])?;
    let Some(row) = rows.next()? else {
        return Ok(None);
    };
    Ok(Some(decode_listener_row(row)?))
}

fn decode_listener_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ListenerRecord> {
    let kind_raw: String = row.get(2)?;
    let config_json: String = row.get(6)?;
    let runtime_status_raw: String = row.get(7)?;
    Ok(ListenerRecord {
        listener_id: row.get(0)?,
        name: row.get(1)?,
        kind: decode_listener_kind(&kind_raw),
        bind_host: row.get(3)?,
        bind_port: row.get(4)?,
        enabled: row.get::<_, i64>(5)? != 0,
        config: serde_json::from_str(&config_json).unwrap_or_else(|_| serde_json::json!({})),
        runtime_status: decode_listener_runtime_status(&runtime_status_raw),
        last_error: row.get(8)?,
        created_at: row.get(9)?,
        updated_at: row.get(10)?,
    })
}

fn encode_listener_kind(kind: ListenerKind) -> &'static str {
    match kind {
        ListenerKind::TcpJson => "tcp_json",
        ListenerKind::HttpsJson => "https_json",
        ListenerKind::PrivateProto => "private_proto",
    }
}

fn decode_listener_kind(raw: &str) -> ListenerKind {
    match raw {
        "tcp_json" => ListenerKind::TcpJson,
        "https_json" => ListenerKind::HttpsJson,
        "private_proto" => ListenerKind::PrivateProto,
        _ => ListenerKind::PrivateProto,
    }
}

fn encode_listener_runtime_status(status: ListenerRuntimeStatus) -> &'static str {
    match status {
        ListenerRuntimeStatus::Starting => "starting",
        ListenerRuntimeStatus::Running => "running",
        ListenerRuntimeStatus::Stopped => "stopped",
        ListenerRuntimeStatus::Error => "error",
    }
}

fn decode_listener_runtime_status(raw: &str) -> ListenerRuntimeStatus {
    match raw {
        "starting" => ListenerRuntimeStatus::Starting,
        "running" => ListenerRuntimeStatus::Running,
        "stopped" => ListenerRuntimeStatus::Stopped,
        "error" => ListenerRuntimeStatus::Error,
        _ => ListenerRuntimeStatus::Error,
    }
}

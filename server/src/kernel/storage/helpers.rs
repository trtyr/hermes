use super::*;

pub(super) fn open_connection(path: &Path) -> anyhow::Result<Connection> {
    Connection::open(path).with_context(|| format!("failed to open sqlite at {}", path.display()))
}

pub(super) fn encode_task_status(status: &TaskStatus) -> &'static str {
    match status {
        TaskStatus::Pending => "pending",
        TaskStatus::Dispatched => "dispatched",
        TaskStatus::Running => "running",
        TaskStatus::CancelRequested => "cancel_requested",
        TaskStatus::Succeeded => "succeeded",
        TaskStatus::Failed => "failed",
        TaskStatus::Cancelled => "cancelled",
        TaskStatus::Partial => "partial",
    }
}

pub(super) fn decode_task_status(status: &str) -> TaskStatus {
    match status {
        "pending" => TaskStatus::Pending,
        "dispatched" => TaskStatus::Dispatched,
        "running" => TaskStatus::Running,
        "cancel_requested" => TaskStatus::CancelRequested,
        "succeeded" => TaskStatus::Succeeded,
        "failed" => TaskStatus::Failed,
        "cancelled" => TaskStatus::Cancelled,
        "partial" => TaskStatus::Partial,
        _ => TaskStatus::Failed,
    }
}

pub(super) fn parse_task_seq(task_id: &str) -> Option<u64> {
    let suffix = task_id.strip_prefix("task-")?;
    let numeric = suffix.split(':').next()?;
    numeric.parse::<u64>().ok()
}

pub(super) fn now_ts() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or(0)
}

pub(super) fn ensure_agent_disabled_column(connection: &Connection) -> anyhow::Result<()> {
    let mut statement = connection.prepare("PRAGMA table_info(agents)")?;
    let columns = statement
        .query_map([], |row| row.get::<_, String>(1))?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    if !columns.iter().any(|column| column == "is_disabled") {
        connection.execute(
            "ALTER TABLE agents ADD COLUMN is_disabled INTEGER NOT NULL DEFAULT 0",
            [],
        )?;
    }
    Ok(())
}

pub(super) fn ensure_agent_metadata_columns(connection: &Connection) -> anyhow::Result<()> {
    let mut statement = connection.prepare("PRAGMA table_info(agents)")?;
    let columns = statement
        .query_map([], |row| row.get::<_, String>(1))?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    if !columns.iter().any(|column| column == "os") {
        connection.execute("ALTER TABLE agents ADD COLUMN os TEXT", [])?;
    }
    if !columns.iter().any(|column| column == "arch") {
        connection.execute("ALTER TABLE agents ADD COLUMN arch TEXT", [])?;
    }
    if !columns.iter().any(|column| column == "pid") {
        connection.execute("ALTER TABLE agents ADD COLUMN pid INTEGER", [])?;
    }
    if !columns.iter().any(|column| column == "internal_ip") {
        connection.execute("ALTER TABLE agents ADD COLUMN internal_ip TEXT", [])?;
    }
    if !columns.iter().any(|column| column == "sleep_interval") {
        connection.execute(
            "ALTER TABLE agents ADD COLUMN sleep_interval INTEGER NOT NULL DEFAULT 0",
            [],
        )?;
    }
    if !columns.iter().any(|column| column == "jitter") {
        connection.execute(
            "ALTER TABLE agents ADD COLUMN jitter INTEGER NOT NULL DEFAULT 0",
            [],
        )?;
    }

    Ok(())
}

pub(super) fn ensure_agent_listener_columns(connection: &Connection) -> anyhow::Result<()> {
    let mut statement = connection.prepare("PRAGMA table_info(agents)")?;
    let columns = statement
        .query_map([], |row| row.get::<_, String>(1))?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    if !columns.iter().any(|column| column == "listener_id") {
        connection.execute("ALTER TABLE agents ADD COLUMN listener_id INTEGER", [])?;
    }
    if !columns.iter().any(|column| column == "listener_name") {
        connection.execute("ALTER TABLE agents ADD COLUMN listener_name TEXT", [])?;
    }

    Ok(())
}

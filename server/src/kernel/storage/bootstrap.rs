use super::*;

impl Storage {
    pub async fn new(sqlite_path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let sqlite_path = sqlite_path.into();
        let storage = Self {
            sqlite_path: Arc::new(sqlite_path),
        };
        storage.init().await?;
        Ok(storage)
    }

    pub async fn bootstrap(&self) -> anyhow::Result<StorageBootstrap> {
        let path = self.sqlite_path.clone();
        tokio::task::spawn_blocking(move || {
            let connection = open_connection(&path)?;
            let now = now_ts();
            connection.execute(
                "UPDATE agents
                 SET is_online = 0, updated_at = CASE WHEN updated_at > ?1 THEN updated_at ELSE ?1 END
                 WHERE is_online != 0",
                params![now],
            )?;
            let mut statement = connection.prepare(
                "SELECT task_id, parent_task_id, target_agent_id, command, payload, status, \
                        created_at, updated_at, success, output, children_json \
                 FROM tasks",
            )?;

            let tasks = statement
                .query_map([], |row| {
                    let status_raw: String = row.get(5)?;
                    let children_json: String = row.get(10)?;
                    Ok(TaskSnapshot {
                        task_id: row.get(0)?,
                        parent_task_id: row.get(1)?,
                        target_agent_id: row.get(2)?,
                        command: row.get(3)?,
                        payload: row.get(4)?,
                        status: decode_task_status(&status_raw),
                        created_at: row.get(6)?,
                        updated_at: row.get(7)?,
                        success: row.get(8)?,
                        output: row.get(9)?,
                        children: serde_json::from_str(&children_json).unwrap_or_default(),
                    })
                })?
                .collect::<rusqlite::Result<Vec<_>>>()?;

            let mut state = KernelState::new();
            state.load_tasks(tasks);
            let repaired_tasks = state.recover_interrupted_tasks(
                now,
                "server restarted before task reached terminal state",
            );
            let tasks = state.task_snapshots();

            for task in repaired_tasks {
                persist_task_sync(&connection, &task)?;
            }

            let next_task_seq = tasks
                .iter()
                .filter_map(|task| parse_task_seq(&task.task_id))
                .max()
                .map(|value| value + 1)
                .unwrap_or(1);

            Ok(StorageBootstrap {
                tasks,
                next_task_seq,
            })
        })
        .await
        .context("sqlite bootstrap join error")?
    }

    async fn init(&self) -> anyhow::Result<()> {
        let path = self.sqlite_path.clone();
        tokio::task::spawn_blocking(move || {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }

            let connection = open_connection(&path)?;
            connection.execute_batch(
                "CREATE TABLE IF NOT EXISTS tasks (
                    task_id TEXT PRIMARY KEY,
                    parent_task_id TEXT,
                    target_agent_id TEXT,
                    command TEXT NOT NULL,
                    payload TEXT,
                    status TEXT NOT NULL,
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL,
                    success INTEGER,
                    output TEXT,
                    children_json TEXT NOT NULL
                );
                 CREATE TABLE IF NOT EXISTS agents (
                    agent_id TEXT PRIMARY KEY,
                    session_id INTEGER,
                    listener_id INTEGER,
                    listener_name TEXT,
                    hostname TEXT,
                    username TEXT,
                    os TEXT,
                    arch TEXT,
                    pid INTEGER,
                    internal_ip TEXT,
                    tags_json TEXT NOT NULL,
                    sleep_interval INTEGER NOT NULL DEFAULT 0,
                    jitter INTEGER NOT NULL DEFAULT 0,
                    peer_addr TEXT NOT NULL,
                    connected_at INTEGER NOT NULL,
                    last_seen INTEGER NOT NULL,
                     is_online INTEGER NOT NULL,
                     is_disabled INTEGER NOT NULL DEFAULT 0,
                     privilege TEXT NOT NULL DEFAULT '',
                     updated_at INTEGER NOT NULL
                 );
                 CREATE TABLE IF NOT EXISTS audits (
                    audit_id INTEGER PRIMARY KEY AUTOINCREMENT,
                    operator TEXT NOT NULL,
                    action TEXT NOT NULL,
                    target_kind TEXT NOT NULL,
                    target_id TEXT,
                    detail TEXT,
                    created_at INTEGER NOT NULL
                 );
                 CREATE TABLE IF NOT EXISTS listeners (
                    listener_id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL UNIQUE,
                    kind TEXT NOT NULL,
                    bind_host TEXT NOT NULL,
                    bind_port INTEGER NOT NULL,
                    enabled INTEGER NOT NULL,
                    config_json TEXT NOT NULL,
                    runtime_status TEXT NOT NULL,
                    last_error TEXT,
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL
                 );
                 CREATE TABLE IF NOT EXISTS agent_builds (
                    build_id INTEGER PRIMARY KEY AUTOINCREMENT,
                    target_triple TEXT NOT NULL,
                    profile TEXT NOT NULL,
                    listener_id INTEGER,
                    server_addr TEXT NOT NULL,
                    embedded_agent_token INTEGER NOT NULL,
                    artifact_path TEXT,
                    artifact_name TEXT,
                    status TEXT NOT NULL,
                    detail TEXT,
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL
                 );",
            )?;
            ensure_agent_disabled_column(&connection)?;
            ensure_agent_metadata_columns(&connection)?;
            ensure_agent_listener_columns(&connection)?;
            Ok(())
        })
        .await
        .context("sqlite init join error")?
    }
}

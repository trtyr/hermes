use super::*;

impl Storage {
    pub fn persist_task(&self, task: TaskSnapshot) {
        let result = (|| -> anyhow::Result<()> {
            let connection = open_connection(&self.sqlite_path)?;
            persist_task_sync(&connection, &task)
        })();

        if let Err(error) = result {
            eprintln!("Failed to persist task: {}", error);
        }
    }
}

pub(super) fn persist_task_sync(
    connection: &Connection,
    task: &TaskSnapshot,
) -> anyhow::Result<()> {
    connection.execute(
        "INSERT INTO tasks (
            task_id, parent_task_id, target_agent_id, command, payload, status,
            created_at, updated_at, success, output, children_json
         ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
         ON CONFLICT(task_id) DO UPDATE SET
            parent_task_id = excluded.parent_task_id,
            target_agent_id = excluded.target_agent_id,
            command = excluded.command,
            payload = excluded.payload,
            status = excluded.status,
            created_at = excluded.created_at,
            updated_at = excluded.updated_at,
            success = excluded.success,
            output = excluded.output,
            children_json = excluded.children_json",
        params![
            task.task_id,
            task.parent_task_id,
            task.target_agent_id,
            task.command,
            task.payload,
            encode_task_status(&task.status),
            task.created_at,
            task.updated_at,
            task.success,
            task.output,
            serde_json::to_string(&task.children)?,
        ],
    )?;
    Ok(())
}

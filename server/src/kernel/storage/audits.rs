use super::*;

impl Storage {
    pub fn append_audit_record(
        &self,
        operator: String,
        action: String,
        target_kind: String,
        target_id: Option<String>,
        detail: Option<String>,
        created_at: u64,
    ) {
        let result = (|| -> anyhow::Result<()> {
            let connection = open_connection(&self.sqlite_path)?;
            connection.execute(
                "INSERT INTO audits (
                    operator, action, target_kind, target_id, detail, created_at
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![operator, action, target_kind, target_id, detail, created_at],
            )?;
            Ok(())
        })();

        if let Err(error) = result {
            eprintln!("Failed to persist audit record: {}", error);
        }
    }

    pub async fn filtered_audit_records(
        &self,
        filter: AuditRecordFilter,
    ) -> anyhow::Result<Vec<AuditRecord>> {
        let path = self.sqlite_path.clone();
        tokio::task::spawn_blocking(move || {
            let connection = open_connection(&path)?;
            let mut statement = connection.prepare(
                "SELECT audit_id, operator, action, target_kind, target_id, detail, created_at
                 FROM audits
                 ORDER BY audit_id DESC",
            )?;

            let mut records = statement
                .query_map([], |row| {
                    Ok(AuditRecord {
                        audit_id: row.get(0)?,
                        operator: row.get(1)?,
                        action: row.get(2)?,
                        target_kind: row.get(3)?,
                        target_id: row.get(4)?,
                        detail: row.get(5)?,
                        created_at: row.get(6)?,
                    })
                })?
                .collect::<rusqlite::Result<Vec<_>>>()?;

            if let Some(operator) = filter.operator {
                let operator = operator.to_lowercase();
                records.retain(|record| record.operator.to_lowercase().contains(&operator));
            }

            if let Some(action) = filter.action {
                let action = action.to_lowercase();
                records.retain(|record| record.action.to_lowercase() == action);
            }

            if let Some(target_kind) = filter.target_kind {
                let target_kind = target_kind.to_lowercase();
                records.retain(|record| record.target_kind.to_lowercase() == target_kind);
            }

            if let Some(target_id) = filter.target_id {
                let target_id = target_id.to_lowercase();
                records.retain(|record| {
                    record
                        .target_id
                        .as_deref()
                        .unwrap_or_default()
                        .to_lowercase()
                        .contains(&target_id)
                });
            }

            Ok(records)
        })
        .await
        .context("sqlite audit records join error")?
    }
}

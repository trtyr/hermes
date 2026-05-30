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
            let mut where_parts: Vec<String> = Vec::new();
            let mut param_values: Vec<String> = Vec::new();

            if let Some(operator) = filter.operator {
                let idx = param_values.len() + 1;
                where_parts.push(format!("operator LIKE ?{idx} COLLATE NOCASE"));
                param_values.push(format!("%{operator}%"));
            }

            if let Some(action) = filter.action {
                let idx = param_values.len() + 1;
                where_parts.push(format!("action = ?{idx} COLLATE NOCASE"));
                param_values.push(action);
            }

            if let Some(target_kind) = filter.target_kind {
                let idx = param_values.len() + 1;
                where_parts.push(format!("target_kind = ?{idx} COLLATE NOCASE"));
                param_values.push(target_kind);
            }

            if let Some(target_id) = filter.target_id {
                let idx = param_values.len() + 1;
                where_parts.push(format!("COALESCE(target_id, '') LIKE ?{idx} COLLATE NOCASE"));
                param_values.push(format!("%{target_id}%"));
            }

            let sql = if where_parts.is_empty() {
                "SELECT audit_id, operator, action, target_kind, target_id, detail, created_at
                 FROM audits
                 ORDER BY audit_id DESC"
                    .to_string()
            } else {
                format!(
                    "SELECT audit_id, operator, action, target_kind, target_id, detail, created_at
                     FROM audits
                     WHERE {}
                     ORDER BY audit_id DESC",
                    where_parts.join(" AND ")
                )
            };

            let mut statement = connection.prepare(&sql)?;
            let params: Vec<&dyn rusqlite::types::ToSql> = param_values
                .iter()
                .map(|value| value as &dyn rusqlite::types::ToSql)
                .collect();

            let records = statement
                .query_map(params.as_slice(), |row| {
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

            Ok(records)
        })
        .await
        .context("sqlite audit records join error")?
    }

    pub fn clear_audit_records(&self) -> anyhow::Result<usize> {
        let connection = open_connection(&self.sqlite_path)?;
        let deleted = connection.execute("DELETE FROM audits", [])?;
        Ok(deleted)
    }
}

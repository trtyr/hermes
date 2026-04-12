use super::*;

impl KernelHandle {
    pub fn append_audit_record(
        &self,
        operator: String,
        action: String,
        target_kind: String,
        target_id: Option<String>,
        detail: Option<String>,
        created_at: u64,
    ) {
        self.storage.append_audit_record(
            operator,
            action,
            target_kind,
            target_id,
            detail,
            created_at,
        );
    }

    pub async fn filtered_audit_records(
        &self,
        operator: Option<String>,
        action: Option<String>,
        target_kind: Option<String>,
        target_id: Option<String>,
    ) -> anyhow::Result<Vec<AuditRecord>> {
        self.storage
            .filtered_audit_records(AuditRecordFilter {
                operator,
                action,
                target_kind,
                target_id,
            })
            .await
    }
}

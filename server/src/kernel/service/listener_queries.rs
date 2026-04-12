use super::KernelHandle;
use crate::kernel::storage::ListenerRecordFilter;
use crate::protocol::{ListenerKind, ListenerRecord};

#[derive(Clone)]
pub struct ListenerQueryFacade {
    pub(super) kernel: KernelHandle,
}

impl ListenerQueryFacade {
    pub async fn filtered_records(
        &self,
        enabled: Option<bool>,
        kind: Option<ListenerKind>,
        keyword: Option<String>,
    ) -> anyhow::Result<Vec<ListenerRecord>> {
        self.kernel
            .storage
            .filtered_listener_records(ListenerRecordFilter {
                enabled,
                kind,
                keyword,
            })
            .await
    }

    pub async fn record(&self, listener_id: i64) -> anyhow::Result<Option<ListenerRecord>> {
        self.kernel.storage.listener_record(listener_id).await
    }
}

use std::{fs, path::PathBuf};

use tokio::process::Command;

use crate::{
    kernel::storage::AgentBuildRecordFilter,
    protocol::{AgentBuildRecord, AgentBuildStatus, ListenerKind, WebEvent},
};

use super::KernelHandle;

mod build;
mod queries;
mod toolchain;

const DEFAULT_AGENT_PROJECT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../agent");
const DEFAULT_AGENT_ARTIFACT_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/agent-builds");

#[derive(Clone)]
pub struct AgentBuildFacade {
    pub(super) kernel: KernelHandle,
}

impl AgentBuildFacade {
    pub async fn delete_build(&self, build_id: i64) -> anyhow::Result<bool> {
        let record = self.kernel.storage.agent_build_record(build_id).await?;
        let Some(record) = record else {
            return Ok(false);
        };
        if record.status == AgentBuildStatus::Pending {
            return Err(anyhow::anyhow!(
                "agent build {} is still pending; wait for it to complete before deleting",
                build_id
            ));
        }

        let deleted = self
            .kernel
            .storage
            .delete_agent_build_record(build_id)
            .await?;
        if deleted {
            let artifact_dir =
                PathBuf::from(DEFAULT_AGENT_ARTIFACT_DIR).join(format!("build-{build_id}"));
            if artifact_dir.exists() {
                let _ = fs::remove_dir_all(&artifact_dir);
            }
            self.kernel
                .publish_web_event(WebEvent::AgentBuildDeleted { build_id });
        }
        Ok(deleted)
    }
}

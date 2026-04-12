use std::{fs, path::PathBuf};

use tokio::process::Command;

use crate::{
    kernel::storage::AgentBuildRecordFilter,
    protocol::{AgentBuildRecord, AgentBuildStatus, ListenerKind},
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

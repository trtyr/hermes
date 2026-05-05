use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context;
use rusqlite::{Connection, params};

use crate::protocol::{
    AgentBuildRecord, AgentBuildStatus, AgentRecord, AgentSnapshot, AuditRecord, ListenerKind,
    ListenerRecord, ListenerRuntimeStatus, TaskSnapshot, TaskStatus,
};

use super::state::KernelState;

mod agent_builds;
mod agents;
mod audits;
mod bootstrap;
mod helpers;
mod listeners;
mod proxy_sessions;
mod tasks;

use helpers::*;
use proxy_sessions::DbProxySession;
use tasks::persist_task_sync;

#[derive(Clone)]
pub struct Storage {
    pub(super) sqlite_path: Arc<PathBuf>,
}

pub struct StorageBootstrap {
    pub tasks: Vec<TaskSnapshot>,
    pub next_task_seq: u64,
    pub proxy_sessions: Vec<DbProxySession>,
}

pub struct AgentRecordFilter {
    pub online: Option<bool>,
    pub disabled: Option<bool>,
    pub keyword: Option<String>,
    pub tag: Option<String>,
}

pub struct AuditRecordFilter {
    pub operator: Option<String>,
    pub action: Option<String>,
    pub target_kind: Option<String>,
    pub target_id: Option<String>,
}

pub struct ListenerRecordFilter {
    pub enabled: Option<bool>,
    pub kind: Option<ListenerKind>,
    pub keyword: Option<String>,
}

pub struct AgentBuildRecordFilter {
    pub status: Option<AgentBuildStatus>,
    pub target_triple: Option<String>,
}

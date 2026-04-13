// Agent-build API handlers: generate platform-specific agent binaries from the sibling agent project.
use super::*;

use axum::extract::{Path, Query, State};

mod mutations;
mod queries;

pub(crate) use mutations::create_agent_build;
pub(crate) use queries::{download_agent_build, get_agent_build, list_agent_builds};

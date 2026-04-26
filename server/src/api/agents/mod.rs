// Agent API handlers: lifecycle, inventory, dispatch-to-agent administration.
use super::*;

use crate::protocol::WebEvent;
use axum::extract::{Path, Query, State};

mod beacon;
mod file_ops;
mod mutations;
mod queries;
mod screenshot;
mod tasking;

pub(crate) use beacon::update_agent_beacon_config;
pub(crate) use file_ops::{browse_file, download_file, upload_file};
pub(crate) use mutations::{delete_agent, disable_agent, enable_agent, update_agent};
pub(crate) use queries::{get_agent, list_agents, list_persisted_agents};
pub(crate) use screenshot::take_screenshot;
pub(crate) use tasking::{disconnect_agent, dispatch_task};

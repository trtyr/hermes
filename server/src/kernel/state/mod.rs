use std::{
    collections::{HashMap, HashSet, VecDeque},
    net::SocketAddr,
};

use tokio::sync::{mpsc, oneshot};

use crate::protocol::{
    AgentSnapshot, CommandExecutionSnapshot, CommandExecutionStatus, CommandOutputStream,
    CommandSessionResult, CommandSessionSnapshot, CommandSessionStatus, ProxySessionSnapshot,
    ProxySessionStatus, ServerCommand, TaskSnapshot, TaskStatus,
};

mod agent_state;
mod command_state;
mod proxy_state;
mod task_state;
mod types;

#[cfg(test)]
mod proxy_state_tests;

#[cfg(test)]
mod tests;

pub(crate) use types::*;

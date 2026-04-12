use tokio::sync::mpsc;

use super::KernelHandle;
use crate::kernel::message::{KernelMessage, TaskKernelMessage};
use crate::protocol::{TaskSnapshot, TaskStatus};

mod commands;
mod queries;

#[derive(Clone)]
pub struct TaskFacade {
    pub(super) kernel: KernelHandle,
}

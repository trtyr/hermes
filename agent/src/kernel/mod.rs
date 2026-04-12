//! Microkernel Core

pub mod memory;
pub mod plugin;
pub mod scheduler;

pub use memory::SecureServerAddr;
pub use plugin::{Plugin, PluginRegistry};
pub use scheduler::Kernel;

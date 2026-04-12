//! System Abstraction

pub mod native;

pub use native::{get_arch, get_hostname, get_os, get_pid, get_username};

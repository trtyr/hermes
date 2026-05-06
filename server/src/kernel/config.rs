use std::net::Ipv4Addr;
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};
use toml::from_str;
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub api: Option<ApiConfig>,
    pub storage: Option<StorageConfig>,
    pub auth: Option<AuthConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: Ipv4Addr,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct ApiConfig {
    pub host: Ipv4Addr,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StorageConfig {
    pub sqlite_path: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AuthConfig {
    pub api_token: Option<String>,
    pub agent_token: Option<String>,
    pub agent_auth_mode: Option<AgentAuthMode>,
    pub web_username: Option<String>,
    pub web_password: Option<String>,
    pub session_ttl_secs: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AgentAuthMode {
    #[default]
    PlainToken,
    ChallengeResponse,
}

impl Config {
    pub fn get_config() -> anyhow::Result<Self, anyhow::Error> {
        let config_file = std::fs::read_to_string("config.toml")?;
        let config: Config = from_str(&config_file)?;

        Ok(config)
    }

    pub fn tcp_addr(&self) -> (Ipv4Addr, u16) {
        (self.server.host, self.server.port)
    }

    pub fn api_addr(&self) -> (Ipv4Addr, u16) {
        self.api
            .map(|api| (api.host, api.port))
            .unwrap_or((Ipv4Addr::UNSPECIFIED, 3000))
    }

    pub fn sqlite_path(&self) -> &str {
        self.storage
            .as_ref()
            .map(|storage| storage.sqlite_path.as_str())
            .unwrap_or("data/server.db")
    }

    pub fn api_token(&self) -> Option<&str> {
        self.auth
            .as_ref()
            .and_then(|auth| auth.api_token.as_deref())
            .filter(|value| !value.is_empty())
    }

    pub fn agent_token(&self) -> Option<&str> {
        self.auth
            .as_ref()
            .and_then(|auth| auth.agent_token.as_deref())
            .filter(|value| !value.is_empty())
    }

    pub fn agent_auth_mode(&self) -> AgentAuthMode {
        self.auth
            .as_ref()
            .and_then(|auth| auth.agent_auth_mode)
            .unwrap_or_default()
    }

    pub fn web_username(&self) -> Option<&str> {
        self.auth
            .as_ref()
            .and_then(|auth| auth.web_username.as_deref())
            .map(str::trim)
            .filter(|value| !value.is_empty())
    }

    pub fn web_password(&self) -> Option<&str> {
        self.auth
            .as_ref()
            .and_then(|auth| auth.web_password.as_deref())
            .map(str::trim)
            .filter(|value| !value.is_empty())
    }

    pub fn session_ttl_secs(&self) -> u64 {
        self.auth
            .as_ref()
            .and_then(|auth| auth.session_ttl_secs)
            .unwrap_or(8 * 60 * 60)
            .max(60)
    }

    pub fn write_auth_config(
        agent_token: Option<String>,
        agent_auth_mode: AgentAuthMode,
    ) -> anyhow::Result<()> {
        let config_file = std::fs::read_to_string("config.toml")?;
        let mut config: Config = from_str(&config_file)?;
        let auth = config.auth.get_or_insert_with(AuthConfig::default);
        auth.agent_token = agent_token;
        auth.agent_auth_mode = Some(agent_auth_mode);
        let updated = toml::to_string_pretty(&config)?;
        std::fs::write("config.toml", updated)?;
        Ok(())
    }
}

/// Shared runtime config for agent authentication, readable by the listener
/// manager and writable by the API layer.
#[derive(Debug, Clone)]
pub struct AgentAuthConfig {
    pub agent_token: Option<String>,
    pub agent_auth_mode: AgentAuthMode,
}

impl AgentAuthConfig {
    pub fn shared(agent_token: Option<String>, agent_auth_mode: AgentAuthMode) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            agent_token,
            agent_auth_mode,
        }))
    }
}

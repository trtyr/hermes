use std::sync::{Arc, RwLock};

use anyhow::Context;

use super::{
    state::AuthState,
    types::{AuthIdentity, WebSession},
};

#[derive(Clone)]
pub struct AuthService {
    state: Arc<RwLock<AuthState>>,
}

impl AuthService {
    pub fn new(
        legacy_api_token: Option<String>,
        web_username: Option<String>,
        web_password: Option<String>,
        session_ttl_secs: u64,
    ) -> Self {
        Self {
            state: Arc::new(RwLock::new(AuthState::new(
                legacy_api_token,
                web_username.and_then(normalize_credential),
                web_password.and_then(normalize_credential),
                session_ttl_secs,
            ))),
        }
    }

    pub fn auth_required(&self) -> bool {
        let state = self.state.read().expect("auth state poisoned");
        state.web_username.is_some() && state.web_password.is_some()
            || state.legacy_api_token.is_some()
    }

    pub fn session_ttl_secs(&self) -> u64 {
        self.state
            .read()
            .expect("auth state poisoned")
            .session_ttl_secs
    }

    pub fn web_login_configured(&self) -> bool {
        let state = self.state.read().expect("auth state poisoned");
        state.web_username.is_some() && state.web_password.is_some()
    }

    pub fn validate_web_credentials(&self, username: &str, password: &str) -> Option<String> {
        let username = username.trim();
        let password = password.trim();
        let state = self.state.read().expect("auth state poisoned");
        let expected_username = state.web_username.as_deref()?;
        let expected_password = state.web_password.as_deref()?;
        if username == expected_username && password == expected_password {
            Some(expected_username.to_string())
        } else {
            None
        }
    }

    pub fn create_session(&self, username: &str) -> anyhow::Result<WebSession> {
        let expires_at = now_ts().saturating_add(self.session_ttl_secs().saturating_mul(1000));
        let session = WebSession {
            session_token: generate_session_token()?,
            username: username.to_string(),
            expires_at,
        };
        self.state
            .write()
            .expect("auth state poisoned")
            .sessions
            .insert(session.session_token.clone(), session.clone());
        Ok(session)
    }

    pub fn remove_session(&self, session_token: &str) -> bool {
        self.state
            .write()
            .expect("auth state poisoned")
            .sessions
            .remove(session_token)
            .is_some()
    }

    pub fn lookup_session_token(&self, session_token: &str) -> Option<WebSession> {
        let state = self.state.read().expect("auth state poisoned");
        let session = state.sessions.get(session_token)?.clone();
        if session.expires_at > now_ts() {
            Some(session)
        } else {
            None
        }
    }

    pub fn resolve_token(&self, provided_token: &str) -> Option<AuthIdentity> {
        if let Some(session) = self.lookup_session_token(provided_token) {
            return Some(AuthIdentity {
                username: Some(session.username),
                session_token: Some(session.session_token),
                via_legacy_api_token: false,
            });
        }

        let state = self.state.read().expect("auth state poisoned");
        if state.legacy_api_token.as_deref() == Some(provided_token) {
            Some(AuthIdentity {
                username: None,
                session_token: None,
                via_legacy_api_token: true,
            })
        } else {
            None
        }
    }
}

fn normalize_credential(value: String) -> Option<String> {
    let value = value.trim().to_string();
    if value.is_empty() { None } else { Some(value) }
}

fn generate_session_token() -> anyhow::Result<String> {
    let mut bytes = [0_u8; 24];
    getrandom::fill(&mut bytes).context("failed to generate session token")?;
    Ok(hex_encode(&bytes))
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(hex_digit(byte >> 4));
        output.push(hex_digit(byte & 0x0f));
    }
    output
}

fn hex_digit(value: u8) -> char {
    match value {
        0..=9 => (b'0' + value) as char,
        10..=15 => (b'a' + (value - 10)) as char,
        _ => '0',
    }
}

fn now_ts() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or(0)
}

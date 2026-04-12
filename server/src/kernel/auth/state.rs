use std::collections::HashMap;

use super::types::WebSession;

pub struct AuthState {
    pub legacy_api_token: Option<String>,
    pub web_username: Option<String>,
    pub web_password: Option<String>,
    pub session_ttl_secs: u64,
    pub sessions: HashMap<String, WebSession>,
}

impl AuthState {
    pub fn new(
        legacy_api_token: Option<String>,
        web_username: Option<String>,
        web_password: Option<String>,
        session_ttl_secs: u64,
    ) -> Self {
        Self {
            legacy_api_token,
            web_username,
            web_password,
            session_ttl_secs,
            sessions: HashMap::new(),
        }
    }
}

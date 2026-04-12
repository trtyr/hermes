pub const SESSION_COOKIE_NAME: &str = "hermes_session";

#[derive(Clone)]
pub struct WebSession {
    pub session_token: String,
    pub username: String,
    pub expires_at: u64,
}

#[derive(Clone)]
pub struct AuthIdentity {
    pub username: Option<String>,
    pub session_token: Option<String>,
    pub via_legacy_api_token: bool,
}

use crate::kernel::{AuthIdentity, WebSession};

use super::KernelHandle;

#[derive(Clone)]
pub struct AuthFacade {
    pub(super) kernel: KernelHandle,
}

impl AuthFacade {
    pub fn auth_required(&self) -> bool {
        self.kernel.auth.auth_required()
    }

    pub fn session_ttl_secs(&self) -> u64 {
        self.kernel.auth.session_ttl_secs()
    }

    pub fn web_login_configured(&self) -> bool {
        self.kernel.auth.web_login_configured()
    }

    pub fn validate_web_credentials(&self, username: &str, password: &str) -> Option<String> {
        self.kernel
            .auth
            .validate_web_credentials(username, password)
    }

    pub fn create_session(&self, username: &str) -> anyhow::Result<WebSession> {
        self.kernel.auth.create_session(username)
    }

    pub fn remove_session(&self, session_token: &str) -> bool {
        self.kernel.auth.remove_session(session_token)
    }

    pub fn lookup_session_token(&self, session_token: &str) -> Option<WebSession> {
        self.kernel.auth.lookup_session_token(session_token)
    }

    pub fn resolve_token(&self, provided_token: &str) -> Option<AuthIdentity> {
        self.kernel.auth.resolve_token(provided_token)
    }
}

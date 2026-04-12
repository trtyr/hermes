// Kernel root defines the internal control-plane core:
// messages, runtime dispatcher, service facades, state, and storage.
mod auth;
mod bus;
mod config;
mod message;
mod runtime;
mod service;
mod state;
mod storage;

pub use auth::{AuthIdentity, SESSION_COOKIE_NAME, WebSession};
pub use config::{AgentAuthMode, Config};
pub use message::AgentKernelMessage;
pub use runtime::new_kernel;
pub use service::{KernelHandle, is_command_session_timeout};

use super::auth::{compute_auth_response, is_agent_token_valid};
use crate::kernel::AgentAuthMode;

#[test]
fn plain_token_mode_accepts_matching_token() {
    assert!(is_agent_token_valid(
        AgentAuthMode::PlainToken,
        Some("secret"),
        Some("secret"),
        None,
        None,
        None,
        "agent-1",
    ));
}

#[test]
fn challenge_response_mode_accepts_matching_hmac() {
    let response = compute_auth_response("secret", "nonce-1", "agent-1").unwrap();
    assert!(is_agent_token_valid(
        AgentAuthMode::ChallengeResponse,
        Some("secret"),
        None,
        Some("nonce-1"),
        Some("nonce-1"),
        Some(&response),
        "agent-1",
    ));
}

#[test]
fn challenge_response_mode_rejects_wrong_nonce() {
    let response = compute_auth_response("secret", "nonce-1", "agent-1").unwrap();
    assert!(!is_agent_token_valid(
        AgentAuthMode::ChallengeResponse,
        Some("secret"),
        None,
        Some("nonce-1"),
        Some("nonce-2"),
        Some(&response),
        "agent-1",
    ));
}

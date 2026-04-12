use anyhow::Context;
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::kernel::AgentAuthMode;

pub(super) fn is_agent_token_valid(
    auth_mode: AgentAuthMode,
    expected: Option<&str>,
    provided_token: Option<&str>,
    expected_nonce: Option<&str>,
    provided_nonce: Option<&str>,
    auth_response: Option<&str>,
    agent_id: &str,
) -> bool {
    match expected {
        None => true,
        Some(expected) => match auth_mode {
            AgentAuthMode::PlainToken => provided_token == Some(expected),
            AgentAuthMode::ChallengeResponse => {
                let Some(expected_nonce) = expected_nonce else {
                    return false;
                };
                let Some(provided_nonce) = provided_nonce else {
                    return false;
                };
                let Some(auth_response) = auth_response else {
                    return false;
                };
                if provided_nonce != expected_nonce {
                    return false;
                }
                verify_auth_response(expected, expected_nonce, agent_id, auth_response)
            }
        },
    }
}

fn verify_auth_response(
    token: &str,
    session_nonce: &str,
    agent_id: &str,
    provided_response: &str,
) -> bool {
    compute_auth_response(token, session_nonce, agent_id)
        .map(|expected| expected == provided_response)
        .unwrap_or(false)
}

pub(super) fn compute_auth_response(
    token: &str,
    session_nonce: &str,
    agent_id: &str,
) -> anyhow::Result<String> {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac =
        HmacSha256::new_from_slice(token.as_bytes()).context("invalid auth token length")?;
    mac.update(session_nonce.as_bytes());
    mac.update(b":");
    mac.update(agent_id.as_bytes());
    let digest = mac.finalize().into_bytes();
    Ok(hex_encode(&digest))
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

pub(super) fn generate_session_nonce() -> anyhow::Result<String> {
    let mut bytes = [0_u8; 16];
    getrandom::fill(&mut bytes).context("failed to generate session nonce")?;
    Ok(hex_encode(&bytes))
}

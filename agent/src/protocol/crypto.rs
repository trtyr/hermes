//! Crypto - HMAC-SHA256

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn compute_auth_response(
    token: &str,
    session_nonce: &str,
    agent_id: &str,
) -> Result<String, &'static str> {
    let mut mac = HmacSha256::new_from_slice(token.as_bytes()).map_err(|_| "invalid token")?;

    mac.update(session_nonce.as_bytes());
    mac.update(b":");
    mac.update(agent_id.as_bytes());

    let digest = mac.finalize().into_bytes();
    Ok(hex_encode(&digest))
}

pub fn hex_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(hex_digit(byte >> 4));
        out.push(hex_digit(byte & 0x0f));
    }
    out
}

fn hex_digit(value: u8) -> char {
    match value {
        0..=9 => (b'0' + value) as char,
        10..=15 => (b'a' + (value - 10)) as char,
        _ => '0',
    }
}

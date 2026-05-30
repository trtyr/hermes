use std::sync::Arc;

use rustls::ServerConfig;
use tokio::net::TcpListener;

use crate::{
    console,
    kernel::{AgentAuthMode, KernelHandle},
    protocol::{ListenerRecord, ListenerRuntimeStatus},
};

use super::super::session::handle_json_line_agent_connection;
use super::super::session::protocol::TRANSPORT_PROFILE_HTTPS_JSON_V1;

pub(super) async fn run_https_json_listener(
    kernel: KernelHandle,
    listener: ListenerRecord,
    agent_token: Option<String>,
    agent_auth_mode: AgentAuthMode,
) {
    let bind_addr = format!("{}:{}", listener.bind_host, listener.bind_port);

    let tls_config = match build_tls_server_config() {
        Ok(config) => config,
        Err(error) => {
            kernel.listener_commands().update_runtime_state(
                listener.listener_id,
                ListenerRuntimeStatus::Error,
                Some(format!("TLS config failed: {}", error)),
            );
            console::listener_error(
                &format!("tls_config {} ({})", listener.listener_id, listener.name),
                error,
            );
            return;
        }
    };
    let tls_acceptor = tokio_rustls::TlsAcceptor::from(tls_config);

    match TcpListener::bind(&bind_addr).await {
        Ok(tcp_listener) => {
            kernel.listener_commands().update_runtime_state(
                listener.listener_id,
                ListenerRuntimeStatus::Running,
                None,
            );
            console::startup_listener(
                listener.listener_id,
                &listener.name,
                "https_json_v1",
                tcp_listener
                    .local_addr()
                    .map(|addr| addr.to_string())
                    .unwrap_or(bind_addr.clone()),
            );
            loop {
                match tcp_listener.accept().await {
                    Ok((tcp_stream, peer_addr)) => {
                        let kernel = kernel.clone();
                        let listener_id = listener.listener_id;
                        let listener_name = listener.name.clone();
                        let agent_token = agent_token.clone();
                        let tls_acceptor = tls_acceptor.clone();

                        tokio::spawn(async move {
                            match tls_acceptor.accept(tcp_stream).await {
                                Ok(tls_stream) => {
                                    if let Err(error) = handle_json_line_agent_connection(
                                        kernel,
                                        tls_stream,
                                        Some(listener_id),
                                        Some(listener_name),
                                        peer_addr,
                                        agent_token,
                                        agent_auth_mode,
                                        TRANSPORT_PROFILE_HTTPS_JSON_V1,
                                    )
                                    .await
                                    {
                                        console::listener_error(
                                            &format!("agent connection {}", peer_addr),
                                            error,
                                        );
                                    }
                                }
                                Err(error) => {
                                    eprintln!("TLS handshake failed for {}: {}", peer_addr, error);
                                }
                            }
                        });
                    }
                    Err(error) => {
                        kernel.listener_commands().update_runtime_state(
                            listener.listener_id,
                            ListenerRuntimeStatus::Error,
                            Some(error.to_string()),
                        );
                        console::listener_error(
                            &format!("accept {} ({})", listener.listener_id, listener.name),
                            error,
                        );
                        break;
                    }
                }
            }
        }
        Err(error) => {
            let detail = format!("无法绑定到 {}: {}", bind_addr, error);
            kernel.listener_commands().update_runtime_state(
                listener.listener_id,
                ListenerRuntimeStatus::Error,
                Some(detail),
            );
            console::listener_error(
                &format!(
                    "bind {} ({}) on {}",
                    listener.listener_id, listener.name, bind_addr
                ),
                error,
            );
        }
    }
}

fn build_tls_server_config() -> anyhow::Result<Arc<ServerConfig>> {
    use rustls::crypto::aws_lc_rs::default_provider;

    let provider = Arc::new(default_provider());

    // Generate self-signed certificate
    let cert_params = rcgen::CertificateParams::new(vec!["hermes-server".to_string()])?;
    let key_pair = rcgen::KeyPair::generate()?;
    let cert_der = cert_params.self_signed(&key_pair)?;
    let key_der = key_pair.serialize_der();

    let cert_chain = vec![rustls::pki_types::CertificateDer::from(cert_der)];
    let private_key = rustls::pki_types::PrivateKeyDer::from(
        rustls::pki_types::PrivatePkcs8KeyDer::from(key_der),
    );

    let config = ServerConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()
        .map_err(|e| anyhow::anyhow!("TLS version config failed: {}", e))?
        .with_no_client_auth()
        .with_single_cert(cert_chain, private_key)
        .map_err(|e| anyhow::anyhow!("TLS cert config failed: {}", e))?;

    Ok(Arc::new(config))
}

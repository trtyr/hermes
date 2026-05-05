use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;
use tokio::time::timeout;

use super::KernelHandle;
use crate::kernel::message::ProxyKernelMessage;
use crate::protocol::ProxySessionSnapshot;

const PROXY_OPEN_TIMEOUT: Duration = Duration::from_secs(5);
const PROXY_STREAM_OPEN_TIMEOUT: Duration = Duration::from_secs(10);
const PROXY_CLOSE_TIMEOUT: Duration = Duration::from_secs(5);

static PROXY_SEQ: AtomicU64 = AtomicU64::new(1);
static STREAM_SEQ: AtomicU64 = AtomicU64::new(1);
static LOCAL_PROXY_TASKS: OnceLock<Mutex<HashMap<String, JoinHandle<()>>>> = OnceLock::new();

fn local_proxy_tasks() -> &'static Mutex<HashMap<String, JoinHandle<()>>> {
    LOCAL_PROXY_TASKS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[derive(Clone)]
pub struct ProxyFacade {
    pub(super) kernel: KernelHandle,
}

impl ProxyFacade {
    pub async fn start(&self, agent_id: String) -> anyhow::Result<ProxySessionSnapshot> {
        let proxy_id = format!("proxy-{}", PROXY_SEQ.fetch_add(1, Ordering::Relaxed));
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let bind_addr = listener.local_addr()?.to_string();

        let (tx, rx) = oneshot::channel();
        self.kernel
            .send_proxy_message(ProxyKernelMessage::StartSession {
                agent_id: agent_id.clone(),
                proxy_id: proxy_id.clone(),
                bind_addr: bind_addr.clone(),
                respond_to: tx,
            })
            .await
            .map_err(anyhow::Error::new)?;

        let kernel = self.kernel.clone();
        let proxy_id_for_task = proxy_id.clone();
        let accept_task = tokio::spawn(async move {
            while let Ok((stream, _)) = listener.accept().await {
                let kernel = kernel.clone();
                let proxy_id = proxy_id_for_task.clone();
                tokio::spawn(async move {
                    let _ = handle_socks5_client(kernel, proxy_id, stream).await;
                });
            }
        });
        local_proxy_tasks()
            .lock()
            .unwrap()
            .insert(proxy_id.clone(), accept_task);

        match timeout(PROXY_OPEN_TIMEOUT, rx).await {
            Ok(result) => result.map_err(|_| anyhow::anyhow!("proxy start channel closed"))?,
            Err(_) => Err(anyhow::anyhow!("proxy start timed out")),
        }
    }

    pub async fn stop(&self, proxy_id: String) -> anyhow::Result<ProxySessionSnapshot> {
        let (tx, rx) = oneshot::channel();
        self.kernel
            .send_proxy_message(ProxyKernelMessage::StopSession {
                proxy_id: proxy_id.clone(),
                respond_to: tx,
            })
            .await
            .map_err(anyhow::Error::new)?;

        if let Some(handle) = local_proxy_tasks().lock().unwrap().remove(&proxy_id) {
            handle.abort();
        }

        match timeout(PROXY_CLOSE_TIMEOUT, rx).await {
            Ok(result) => result.map_err(|_| anyhow::anyhow!("proxy close channel closed"))?,
            Err(_) => Err(anyhow::anyhow!("proxy close timed out")),
        }
    }

    pub async fn list_for_agent(&self, agent_id: &str) -> Vec<ProxySessionSnapshot> {
        let state = self.kernel.state.read().await;
        state.proxy_session_snapshots_for_agent(agent_id)
    }
}

async fn handle_socks5_client(
    kernel: KernelHandle,
    proxy_id: String,
    mut stream: TcpStream,
) -> anyhow::Result<()> {
    let mut head = [0u8; 2];
    stream.read_exact(&mut head).await?;
    if head[0] != 0x05 {
        anyhow::bail!("unsupported socks version");
    }
    let nmethods = head[1] as usize;
    let mut methods = vec![0u8; nmethods];
    stream.read_exact(&mut methods).await?;
    stream.write_all(&[0x05, 0x00]).await?;

    let mut req_head = [0u8; 4];
    stream.read_exact(&mut req_head).await?;
    if req_head[0] != 0x05 || req_head[1] != 0x01 {
        stream.write_all(&[0x05, 0x07, 0x00, 0x01, 0, 0, 0, 0, 0, 0]).await?;
        anyhow::bail!("only CONNECT supported");
    }

    let host = match req_head[3] {
        0x01 => {
            let mut addr = [0u8; 4];
            stream.read_exact(&mut addr).await?;
            std::net::Ipv4Addr::from(addr).to_string()
        }
        0x03 => {
            let mut len = [0u8; 1];
            stream.read_exact(&mut len).await?;
            let mut domain = vec![0u8; len[0] as usize];
            stream.read_exact(&mut domain).await?;
            String::from_utf8(domain)?
        }
        0x04 => {
            let mut addr = [0u8; 16];
            stream.read_exact(&mut addr).await?;
            std::net::Ipv6Addr::from(addr).to_string()
        }
        _ => {
            stream.write_all(&[0x05, 0x08, 0x00, 0x01, 0, 0, 0, 0, 0, 0]).await?;
            anyhow::bail!("unsupported address type");
        }
    };

    let mut port_buf = [0u8; 2];
    stream.read_exact(&mut port_buf).await?;
    let port = u16::from_be_bytes(port_buf);

    let stream_id = format!("pstream-{}", STREAM_SEQ.fetch_add(1, Ordering::Relaxed));
    let (client_tx, mut client_rx) = mpsc::unbounded_channel::<Option<Vec<u8>>>();
    let (tx, rx) = oneshot::channel();
    kernel
        .send_proxy_message(ProxyKernelMessage::OpenStream {
            proxy_id: proxy_id.clone(),
            stream_id: stream_id.clone(),
            host,
            port,
            client_sender: client_tx,
            respond_to: tx,
        })
        .await
        .map_err(anyhow::Error::new)?;

    match timeout(PROXY_STREAM_OPEN_TIMEOUT, rx).await {
        Ok(Ok(Ok(()))) => {}
        Ok(Ok(Err(_))) | Ok(Err(_)) | Err(_) => {
            stream.write_all(&[0x05, 0x05, 0x00, 0x01, 0, 0, 0, 0, 0, 0]).await?;
            anyhow::bail!("proxy stream open failed");
        }
    }

    stream.write_all(&[0x05, 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0]).await?;

    let (mut reader, mut writer) = stream.into_split();
    let write_task = tokio::spawn(async move {
        while let Some(msg) = client_rx.recv().await {
            match msg {
                Some(data) => {
                    if writer.write_all(&data).await.is_err() {
                        break;
                    }
                }
                None => break,
            }
        }
        let _ = writer.shutdown().await;
    });

    let mut buf = vec![0u8; 8192];
    loop {
        let n = reader.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        kernel
            .send_proxy_message(ProxyKernelMessage::ClientData {
                proxy_id: proxy_id.clone(),
                stream_id: stream_id.clone(),
                data: buf[..n].to_vec(),
            })
            .await
            .map_err(anyhow::Error::new)?;
    }

    let _ = kernel
        .send_proxy_message(ProxyKernelMessage::ClientClosed {
            proxy_id,
            stream_id,
        })
        .await;
    let _ = write_task.await;
    Ok(())
}

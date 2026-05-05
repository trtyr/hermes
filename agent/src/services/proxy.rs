use crate::kernel::Plugin;
use crate::protocol::AgentMessage;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    mpsc::Sender,
    Arc, Mutex,
};

pub struct ProxyService {
    streams: Arc<Mutex<HashMap<String, Arc<Mutex<TcpStream>>>>>,
    proxy_streams: Arc<Mutex<HashMap<String, Vec<String>>>>,
    sender: Sender<AgentMessage>,
    active_streams: Arc<AtomicUsize>,
    flush_hint: Arc<AtomicBool>,
}

impl ProxyService {
    pub fn new(sender: Sender<AgentMessage>) -> Self {
        Self {
            streams: Arc::new(Mutex::new(HashMap::new())),
            proxy_streams: Arc::new(Mutex::new(HashMap::new())),
            sender,
            active_streams: Arc::new(AtomicUsize::new(0)),
            flush_hint: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn open(&mut self, proxy_id: &str, bind_addr: &str) {
        let _ = self.sender.send(AgentMessage::ProxyOpened {
            proxy_id: proxy_id.to_string(),
            bind_addr: bind_addr.to_string(),
        });
        self.flush_hint.store(true, Ordering::SeqCst);
    }

    pub fn connect(&mut self, proxy_id: &str, stream_id: &str, host: &str, port: u16) {
        match TcpStream::connect((host, port)) {
            Ok(stream) => {
                let _ = stream.set_nodelay(true);
                let read_stream = match stream.try_clone() {
                    Ok(s) => s,
                    Err(error) => {
                        let _ = self.sender.send(AgentMessage::ProxyConnectResult {
                            proxy_id: proxy_id.to_string(),
                            stream_id: stream_id.to_string(),
                            success: false,
                            detail: Some(error.to_string()),
                        });
                        return;
                    }
                };

                let stream = Arc::new(Mutex::new(stream));
                self.streams
                    .lock()
                    .unwrap()
                    .insert(stream_id.to_string(), Arc::clone(&stream));
                self.proxy_streams
                    .lock()
                    .unwrap()
                    .entry(proxy_id.to_string())
                    .or_default()
                    .push(stream_id.to_string());
                self.active_streams.fetch_add(1, Ordering::SeqCst);

                let _ = self.sender.send(AgentMessage::ProxyConnectResult {
                    proxy_id: proxy_id.to_string(),
                    stream_id: stream_id.to_string(),
                    success: true,
                    detail: None,
                });
                self.flush_hint.store(true, Ordering::SeqCst);

                let sender = self.sender.clone();
                let streams = Arc::clone(&self.streams);
                let proxy_streams = Arc::clone(&self.proxy_streams);
                let active_streams = Arc::clone(&self.active_streams);
                let flush_hint = Arc::clone(&self.flush_hint);
                let proxy_id = proxy_id.to_string();
                let stream_id = stream_id.to_string();

                std::thread::spawn(move || {
                    let mut read_stream = read_stream;
                    let mut buf = vec![0u8; 8192];
                    loop {
                        match read_stream.read(&mut buf) {
                            Ok(0) => {
                                cleanup_stream(&streams, &proxy_streams, &proxy_id, &stream_id);
                                active_streams.fetch_sub(1, Ordering::SeqCst);
                                let _ = sender.send(AgentMessage::ProxyClosed {
                                    proxy_id: proxy_id.clone(),
                                    stream_id: stream_id.clone(),
                                });
                                flush_hint.store(true, Ordering::SeqCst);
                                break;
                            }
                            Ok(n) => {
                                let _ = sender.send(AgentMessage::ProxyData {
                                    proxy_id: proxy_id.clone(),
                                    stream_id: stream_id.clone(),
                                    data_base64: STANDARD.encode(&buf[..n]),
                                });
                                flush_hint.store(true, Ordering::SeqCst);
                            }
                            Err(error) => {
                                cleanup_stream(&streams, &proxy_streams, &proxy_id, &stream_id);
                                active_streams.fetch_sub(1, Ordering::SeqCst);
                                let _ = sender.send(AgentMessage::ProxyError {
                                    proxy_id: proxy_id.clone(),
                                    stream_id: Some(stream_id.clone()),
                                    detail: error.to_string(),
                                });
                                flush_hint.store(true, Ordering::SeqCst);
                                break;
                            }
                        }
                    }
                });
            }
            Err(error) => {
                let _ = self.sender.send(AgentMessage::ProxyConnectResult {
                    proxy_id: proxy_id.to_string(),
                    stream_id: stream_id.to_string(),
                    success: false,
                    detail: Some(error.to_string()),
                });
                self.flush_hint.store(true, Ordering::SeqCst);
            }
        }
    }

    pub fn data(&mut self, proxy_id: &str, stream_id: &str, data_base64: &str) {
        let Some(stream) = self.streams.lock().unwrap().get(stream_id).cloned() else {
            let _ = self.sender.send(AgentMessage::ProxyError {
                proxy_id: proxy_id.to_string(),
                stream_id: Some(stream_id.to_string()),
                detail: "proxy stream not found".to_string(),
            });
            self.flush_hint.store(true, Ordering::SeqCst);
            return;
        };

        let data = match STANDARD.decode(data_base64) {
            Ok(data) => data,
            Err(error) => {
                let _ = self.sender.send(AgentMessage::ProxyError {
                    proxy_id: proxy_id.to_string(),
                    stream_id: Some(stream_id.to_string()),
                    detail: error.to_string(),
                });
                self.flush_hint.store(true, Ordering::SeqCst);
                return;
            }
        };

        {
            let mut locked = match stream.lock() {
                Ok(locked) => locked,
                Err(_) => return,
            };
            if let Err(error) = locked.write_all(&data) {
                let _ = self.sender.send(AgentMessage::ProxyError {
                    proxy_id: proxy_id.to_string(),
                    stream_id: Some(stream_id.to_string()),
                    detail: error.to_string(),
                });
                self.flush_hint.store(true, Ordering::SeqCst);
            }
        }
    }

    pub fn close_stream(&mut self, proxy_id: &str, stream_id: &str) {
        let removed = cleanup_stream(&self.streams, &self.proxy_streams, proxy_id, stream_id);
        if removed {
            self.active_streams.fetch_sub(1, Ordering::SeqCst);
        }
    }

    pub fn close_proxy(&mut self, proxy_id: &str) {
        let stream_ids = self
            .proxy_streams
            .lock()
            .unwrap()
            .remove(proxy_id)
            .unwrap_or_default();
        for stream_id in stream_ids {
            if self.streams.lock().unwrap().remove(&stream_id).is_some() {
                self.active_streams.fetch_sub(1, Ordering::SeqCst);
            }
        }
        let _ = self.sender.send(AgentMessage::ProxySessionClosed {
            proxy_id: proxy_id.to_string(),
        });
        self.flush_hint.store(true, Ordering::SeqCst);
    }

    pub fn should_poll_fast(&self) -> bool {
        self.active_streams.load(Ordering::SeqCst) > 0 || self.flush_hint.load(Ordering::SeqCst)
    }

    pub fn clear_flush_hint(&self) {
        self.flush_hint.store(false, Ordering::SeqCst);
    }
}

fn cleanup_stream(
    streams: &Arc<Mutex<HashMap<String, Arc<Mutex<TcpStream>>>>>,
    proxy_streams: &Arc<Mutex<HashMap<String, Vec<String>>>>,
    proxy_id: &str,
    stream_id: &str,
) -> bool {
    let removed = streams.lock().unwrap().remove(stream_id).is_some();
    if let Some(items) = proxy_streams.lock().unwrap().get_mut(proxy_id) {
        items.retain(|item| item != stream_id);
    }
    removed
}

impl Plugin for ProxyService {
    fn name(&self) -> &'static str {
        "proxy"
    }
}

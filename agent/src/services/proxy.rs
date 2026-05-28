use crate::kernel::Plugin;
use crate::protocol::AgentMessage;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    mpsc::Sender,
    Arc, Mutex,
};
use std::thread::JoinHandle;

pub struct ProxyService {
    streams: Arc<Mutex<HashMap<String, Arc<Mutex<TcpStream>>>>>,
    proxy_streams: Arc<Mutex<HashMap<String, Vec<String>>>>,
    reader_handles: HashMap<String, JoinHandle<()>>,
    sender: Sender<AgentMessage>,
    active_streams: Arc<AtomicUsize>,
    flush_hint: Arc<AtomicBool>,
}

impl ProxyService {
    pub fn new(sender: Sender<AgentMessage>) -> Self {
        Self {
            streams: Arc::new(Mutex::new(HashMap::new())),
            proxy_streams: Arc::new(Mutex::new(HashMap::new())),
            reader_handles: HashMap::new(),
            sender,
            active_streams: Arc::new(AtomicUsize::new(0)),
            flush_hint: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Close all TCP streams and clear all proxy state. Called on reconnect so
    /// stale tunnels from a dead connection do not leak.
    pub fn reset(&mut self) {
        // Shutdown all sockets to unblock reader threads
        {
            let streams = self.streams.lock().unwrap_or_else(|e| e.into_inner());
            for (_, stream) in streams.iter() {
                let _ = stream
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .shutdown(Shutdown::Both);
            }
        }
        // Join or abandon all reader threads
        for (_, handle) in self.reader_handles.drain() {
            if handle.is_finished() {
                let _ = handle.join();
            }
            // else: drop handle, thread will exit on its own from the socket shutdown
        }
        // Clear all maps
        self.streams
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clear();
        self.proxy_streams
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clear();
        // Reset counters
        self.active_streams.store(0, Ordering::SeqCst);
        self.flush_hint.store(false, Ordering::SeqCst);
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
                    .unwrap_or_else(|e| e.into_inner())
                    .insert(stream_id.to_string(), Arc::clone(&stream));
                self.proxy_streams
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
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
                let stream_id_clone = stream_id.to_string();

                let handle = std::thread::spawn(move || {
                    let mut read_stream = read_stream;
                    let mut buf = vec![0u8; 8192];
                    loop {
                        match read_stream.read(&mut buf) {
                            Ok(0) => {
                                let removed = cleanup_stream(
                                    &streams,
                                    &proxy_streams,
                                    &proxy_id,
                                    &stream_id_clone,
                                );
                                if removed {
                                    active_streams.fetch_sub(1, Ordering::SeqCst);
                                }
                                let _ = sender.send(AgentMessage::ProxyClosed {
                                    proxy_id: proxy_id.clone(),
                                    stream_id: stream_id_clone.clone(),
                                });
                                flush_hint.store(true, Ordering::SeqCst);
                                break;
                            }
                            Ok(n) => {
                                let _ = sender.send(AgentMessage::ProxyData {
                                    proxy_id: proxy_id.clone(),
                                    stream_id: stream_id_clone.clone(),
                                    data_base64: STANDARD.encode(&buf[..n]),
                                });
                                flush_hint.store(true, Ordering::SeqCst);
                            }
                            Err(error) => {
                                let removed = cleanup_stream(
                                    &streams,
                                    &proxy_streams,
                                    &proxy_id,
                                    &stream_id_clone,
                                );
                                if removed {
                                    active_streams.fetch_sub(1, Ordering::SeqCst);
                                }
                                let _ = sender.send(AgentMessage::ProxyError {
                                    proxy_id: proxy_id.clone(),
                                    stream_id: Some(stream_id_clone.clone()),
                                    detail: error.to_string(),
                                });
                                flush_hint.store(true, Ordering::SeqCst);
                                break;
                            }
                        }
                    }
                });
                self.reader_handles.insert(stream_id.to_string(), handle);
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
        let Some(stream) = self
            .streams
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(stream_id)
            .cloned()
        else {
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
            let mut locked = stream.lock().unwrap_or_else(|e| e.into_inner());
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
        // Shutdown the socket to unblock the reader thread
        if let Some(arc_stream) = self
            .streams
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(stream_id)
            .cloned()
        {
            let _ = arc_stream
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .shutdown(Shutdown::Both);
        }
        // Remove from maps
        let removed = cleanup_stream(&self.streams, &self.proxy_streams, proxy_id, stream_id);
        if removed {
            self.active_streams.fetch_sub(1, Ordering::SeqCst);
        }
        // Join reader thread if finished
        if let Some(handle) = self.reader_handles.remove(stream_id) {
            if handle.is_finished() {
                let _ = handle.join();
            }
        }
    }

    pub fn close_proxy(&mut self, proxy_id: &str) {
        let stream_ids: Vec<String> = self
            .proxy_streams
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .remove(proxy_id)
            .unwrap_or_default();
        for stream_id in &stream_ids {
            // Shutdown socket
            if let Some(arc_stream) = self
                .streams
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .get(stream_id)
                .cloned()
            {
                let _ = arc_stream
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .shutdown(Shutdown::Both);
            }
            // Remove from streams map
            if self
                .streams
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .remove(stream_id)
                .is_some()
            {
                self.active_streams.fetch_sub(1, Ordering::SeqCst);
            }
            // Join reader thread if finished
            if let Some(handle) = self.reader_handles.remove(stream_id) {
                if handle.is_finished() {
                    let _ = handle.join();
                }
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
    let removed = streams
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .remove(stream_id)
        .is_some();
    if let Some(items) = proxy_streams
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get_mut(proxy_id)
    {
        items.retain(|item| item != stream_id);
    }
    removed
}

impl Plugin for ProxyService {
    fn name(&self) -> &'static str {
        "proxy"
    }
}

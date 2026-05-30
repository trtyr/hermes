use axum::http::{Method, StatusCode, Uri};
use std::fmt::Display;

// ---------------------------------------------------------------------------
// Timestamp
// ---------------------------------------------------------------------------

fn timestamp() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    let millis = now.subsec_millis();
    let hours = (secs % 86400) / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;
    format!("{hours:02}:{minutes:02}:{seconds:02}.{millis:03}")
}

// ---------------------------------------------------------------------------
// HTTP request logging
// ---------------------------------------------------------------------------

pub fn http_request(
    method: &Method,
    uri: &Uri,
    status: StatusCode,
    elapsed_ms: u128,
    client_ip: &str,
    user_agent: &str,
    operator: &str,
) {
    let level = if status.is_server_error() {
        "ERR"
    } else if status.is_client_error() {
        "WARN"
    } else {
        " OK"
    };
    let ua_short = if user_agent.len() > 40 {
        &user_agent[..40]
    } else {
        user_agent
    };
    eprintln!(
        "[{}] [http] {} {} {} → {} ({:>4} ms) client={} op={} ua={}",
        timestamp(),
        level,
        method,
        uri,
        status.as_u16(),
        elapsed_ms,
        client_ip,
        operator,
        if ua_short.is_empty() { "-" } else { ua_short },
    );
}

// ---------------------------------------------------------------------------
// Startup
// ---------------------------------------------------------------------------

pub fn startup_http_api(addr: impl Display) {
    eprintln!("[{}] [startup] HTTP API listening on http://{}", timestamp(), addr);
}

pub fn startup_listener(listener_id: i64, name: &str, transport: &str, addr: impl Display) {
    eprintln!(
        "[{}] [startup] listener #{} «{}» [{}] on {}",
        timestamp(),
        listener_id,
        name,
        transport,
        addr,
    );
}

// ---------------------------------------------------------------------------
// Listener lifecycle
// ---------------------------------------------------------------------------

pub fn listener_stopping(listener_id: i64, name: &str, reason: &str) {
    eprintln!(
        "[{}] [listener] #{} «{}» stopping — {}",
        timestamp(),
        listener_id,
        name,
        reason,
    );
}

pub fn listener_restarting(listener_id: i64, name: &str) {
    eprintln!(
        "[{}] [listener] #{} «{}» restarting (runtime exited, backing off)",
        timestamp(),
        listener_id,
        name,
    );
}

pub fn listener_error(context: &str, detail: impl Display) {
    eprintln!("[{}] [listener] ERR {}: {}", timestamp(), context, detail);
}

// ---------------------------------------------------------------------------
// Session (agent connection) lifecycle
// ---------------------------------------------------------------------------

pub fn session_connected(
    session_id: u64,
    peer_addr: &str,
    listener_name: &str,
    listener_id: Option<i64>,
) {
    eprintln!(
        "[{}] [session] #{} connected from {} on listener «{}» (#{})",
        timestamp(),
        session_id,
        peer_addr,
        listener_name,
        listener_id.map(|id| id.to_string()).unwrap_or_else(|| "?".to_string()),
    );
}

pub fn session_register_ok(
    session_id: u64,
    agent_id: &str,
    hostname: &str,
    os: &str,
    arch: &str,
    peer_addr: &str,
    listener_name: &str,
    auth_mode: &str,
) {
    eprintln!(
        "[{}] [session] #{} registered agent {} ({}) {} {} from {} via «{}» [auth={}]",
        timestamp(),
        session_id,
        agent_id,
        hostname,
        os,
        arch,
        peer_addr,
        listener_name,
        auth_mode,
    );
}

pub fn session_register_rejected(session_id: u64, peer_addr: &str, listener_name: &str, reason: &str) {
    eprintln!(
        "[{}] [session] #{} REJECTED from {} on «{}» — {}",
        timestamp(),
        session_id,
        peer_addr,
        listener_name,
        reason,
    );
}

pub fn session_disconnected(
    session_id: u64,
    agent_id: Option<&str>,
    reason: &str,
    listener_name: &str,
) {
    eprintln!(
        "[{}] [session] #{} disconnected agent={} from «{}» — {}",
        timestamp(),
        session_id,
        agent_id.unwrap_or("(unregistered)"),
        listener_name,
        reason,
    );
}

pub fn session_superseded(
    old_session_id: u64,
    new_session_id: u64,
    agent_id: &str,
) {
    eprintln!(
        "[{}] [session] superseded old session #{} with new session #{} for agent {}",
        timestamp(),
        old_session_id,
        new_session_id,
        agent_id,
    );
}

pub fn session_error(session_id: u64, context: &str, detail: impl Display) {
    eprintln!(
        "[{}] [session] #{} ERR {}: {}",
        timestamp(),
        session_id,
        context,
        detail,
    );
}

// ---------------------------------------------------------------------------
// Agent lifecycle
// ---------------------------------------------------------------------------

pub fn agent_online(agent_id: &str, hostname: &str, peer_addr: &str) {
    eprintln!(
        "[{}] [agent] {} ({}) online from {}",
        timestamp(),
        agent_id,
        hostname,
        peer_addr,
    );
}

pub fn agent_offline(agent_id: &str, hostname: &str, reason: &str) {
    eprintln!(
        "[{}] [agent] {} ({}) offline — {}",
        timestamp(),
        agent_id,
        hostname,
        reason,
    );
}

pub fn agent_heartbeat_timeout(agent_id: &str, hostname: &str, session_id: u64, elapsed_ms: u64) {
    eprintln!(
        "[{}] [agent] {} ({}) #{} heartbeat timeout (last seen {}.{:03}s ago), cleaning up",
        timestamp(),
        agent_id,
        hostname,
        session_id,
        elapsed_ms / 1000,
        elapsed_ms % 1000,
    );
}

pub fn agent_heartbeat(agent_id: &str, hostname: &str, session_id: u64) {
    eprintln!(
        "[{}] [heartbeat] {} ({}) #{}",
        timestamp(),
        agent_id,
        hostname,
        session_id,
    );
}

// ---------------------------------------------------------------------------
// Task lifecycle
// ---------------------------------------------------------------------------

pub fn task_created(task_id: &str, command: &str, agent_id: &str) {
    eprintln!(
        "[{}] [task] {} created: command={} target={}",
        timestamp(),
        task_id,
        command,
        agent_id,
    );
}

pub fn task_dispatched(task_id: &str, command: &str, agent_id: &str) {
    eprintln!(
        "[{}] [task] {} dispatched: command={} → agent {}",
        timestamp(),
        task_id,
        command,
        agent_id,
    );
}

pub fn task_completed(task_id: &str, command: &str, agent_id: &str, success: bool) {
    let status = if success { "succeeded" } else { "failed" };
    eprintln!(
        "[{}] [task] {} completed: command={} agent={} result={}",
        timestamp(),
        task_id,
        command,
        agent_id,
        status,
    );
}

pub fn task_cancelled(task_id: &str, agent_id: Option<&str>, reason: &str) {
    eprintln!(
        "[{}] [task] {} cancelled: agent={} reason={}",
        timestamp(),
        task_id,
        agent_id.unwrap_or("-"),
        reason,
    );
}

pub fn task_failed(task_id: &str, agent_id: &str, reason: &str) {
    eprintln!(
        "[{}] [task] {} failed: agent={} reason={}",
        timestamp(),
        task_id,
        agent_id,
        reason,
    );
}

// ---------------------------------------------------------------------------
// Auth
// ---------------------------------------------------------------------------

pub fn auth_login_success(username: &str, client_ip: &str) {
    eprintln!(
        "[{}] [auth] login success: user={} from {}",
        timestamp(),
        username,
        client_ip,
    );
}

pub fn auth_login_failed(username: &str, client_ip: &str) {
    eprintln!(
        "[{}] [auth] login FAILED: user={} from {}",
        timestamp(),
        username,
        client_ip,
    );
}

// ---------------------------------------------------------------------------
// Command session
// ---------------------------------------------------------------------------

pub fn command_session_opened(command_session_id: &str, agent_id: &str, created_by: &str) {
    eprintln!(
        "[{}] [cmd-session] {} opened: agent={} by={}",
        timestamp(),
        command_session_id,
        agent_id,
        created_by,
    );
}

pub fn command_session_closed(command_session_id: &str, agent_id: &str) {
    eprintln!(
        "[{}] [cmd-session] {} closed: agent={}",
        timestamp(),
        command_session_id,
        agent_id,
    );
}

pub fn command_session_execute(command_session_id: &str, command_id: &str, line: &str) {
    let line_preview = if line.len() > 60 { &line[..60] } else { line };
    eprintln!(
        "[{}] [cmd-session] {} execute: cmd={} line={}",
        timestamp(),
        command_session_id,
        command_id,
        line_preview,
    );
}

// ---------------------------------------------------------------------------
// Proxy session
// ---------------------------------------------------------------------------

pub fn proxy_session_started(proxy_id: &str, agent_id: &str, bind_addr: &str) {
    eprintln!(
        "[{}] [proxy] {} started: agent={} bind={}",
        timestamp(),
        proxy_id,
        agent_id,
        bind_addr,
    );
}

pub fn proxy_session_deleted(proxy_id: &str, agent_id: &str) {
    eprintln!(
        "[{}] [proxy] {} deleted: agent={}",
        timestamp(),
        proxy_id,
        agent_id,
    );
}

// ---------------------------------------------------------------------------
// Storage errors (with context)
// ---------------------------------------------------------------------------

pub fn storage_error(context: &str, detail: impl Display) {
    eprintln!("[{}] [storage] ERR {}: {}", timestamp(), context, detail);
}

// ---------------------------------------------------------------------------
// Heartbeat sweep
// ---------------------------------------------------------------------------

pub fn heartbeat_sweep(count: usize) {
    if count > 0 {
        eprintln!(
            "[{}] [heartbeat] sweep: {} session(s) timed out and cleaned up",
            timestamp(),
            count,
        );
    }
}



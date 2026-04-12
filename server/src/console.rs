use axum::http::{Method, StatusCode, Uri};
use std::fmt::Display;

pub fn startup_http_api(addr: impl Display) {
    println!("[startup][http]     listening  http://{addr}");
}

pub fn startup_listener(listener_id: i64, name: &str, transport: &str, addr: impl Display) {
    println!("[startup][listener] #{listener_id} {name} [{transport}] {addr}");
}

pub fn http_request(method: &Method, uri: &Uri, status: StatusCode, elapsed_ms: u128) {
    println!(
        "[http][{} {:>3}][{:>4} ms] {:<6} {}",
        status_class_label(status),
        status.as_u16(),
        elapsed_ms,
        method,
        uri
    );
}

pub fn listener_error(context: &str, detail: impl Display) {
    eprintln!("[error][listener] {context}: {detail}");
}

fn status_class_label(status: StatusCode) -> &'static str {
    match status.as_u16() {
        100..=199 => "INFO",
        200..=299 => "OK",
        300..=399 => "MOVE",
        400..=499 => "CLNT",
        500..=599 => "SRV",
        _ => "UNKN",
    }
}

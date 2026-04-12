use super::*;

use serde_json::{Value, json};

pub(super) fn simplify_terminal_event(payload: &str) -> Option<String> {
    let value: Value = serde_json::from_str(payload).ok()?;
    let event_type = value.get("type")?.as_str()?;

    let simplified = match event_type {
        "command_session_opened" | "command_session_updated" => {
            let session = value.get("session")?;
            json!({
                "type": "terminal",
                "event": "session",
                "session_id": session.get("command_session_id")?.as_str()?,
                "state": session_status_from_value(session.get("status")?.as_str()?),
                "cwd": session.get("cwd")?.as_str()?,
            })
        }
        "command_session_closed" => {
            json!({
                "type": "terminal",
                "event": "session",
                "session_id": value.get("command_session_id")?.as_str()?,
                "state": "closed",
            })
        }
        "command_updated" => {
            let command = value.get("command")?;
            let status = command.get("status")?.as_str()?;
            json!({
                "type": "terminal",
                "event": "command",
                "session_id": command.get("command_session_id")?.as_str()?,
                "command_id": command.get("command_id")?.as_str()?,
                "state": command_state_from_value(status),
                "cwd": command.get("cwd_after").and_then(Value::as_str),
                "exit_code": command.get("exit_code").and_then(Value::as_i64),
                "stdout": command.get("stdout").and_then(Value::as_str),
                "stderr": command.get("stderr").and_then(Value::as_str),
            })
        }
        "command_output_chunk" => {
            let chunk = value.get("chunk")?;
            json!({
                "type": "terminal",
                "event": "output",
                "session_id": chunk.get("command_session_id")?.as_str()?,
                "command_id": chunk.get("request_id")?.as_str()?,
                "stream": chunk.get("stream")?.as_str()?,
                "sequence": chunk.get("sequence")?.as_u64()?,
                "chunk": chunk.get("chunk")?.as_str()?,
            })
        }
        _ => return None,
    };

    Some(simplified.to_string())
}

pub(super) fn web_terminal_error_response(error: anyhow::Error) -> axum::response::Response {
    let status = if crate::kernel::is_command_session_timeout(&error) {
        StatusCode::GATEWAY_TIMEOUT
    } else {
        StatusCode::CONFLICT
    };
    (
        status,
        Json(ApiResponse {
            success: false,
            detail: error.to_string(),
            task_id: None,
        }),
    )
        .into_response()
}

pub(super) fn simple_session_status(status: &crate::protocol::CommandSessionStatus) -> String {
    match status {
        crate::protocol::CommandSessionStatus::Open => "open",
        crate::protocol::CommandSessionStatus::Closed => "closed",
    }
    .to_string()
}

pub(super) fn simple_command_state(status: &crate::protocol::CommandExecutionStatus) -> String {
    match status {
        crate::protocol::CommandExecutionStatus::Queued
        | crate::protocol::CommandExecutionStatus::Dispatched => "queued",
        crate::protocol::CommandExecutionStatus::Running => "running",
        crate::protocol::CommandExecutionStatus::Succeeded => "done",
        crate::protocol::CommandExecutionStatus::Failed
        | crate::protocol::CommandExecutionStatus::Cancelled
        | crate::protocol::CommandExecutionStatus::Dropped => "error",
    }
    .to_string()
}

fn session_status_from_value(status: &str) -> &'static str {
    match status {
        "open" => "open",
        "closed" => "closed",
        _ => "closed",
    }
}

fn command_state_from_value(status: &str) -> &'static str {
    match status {
        "queued" | "dispatched" => "queued",
        "running" => "running",
        "succeeded" => "done",
        "failed" | "cancelled" | "dropped" => "error",
        _ => "error",
    }
}

#[cfg(test)]
mod tests {
    use super::simplify_terminal_event;

    #[test]
    fn command_output_chunk_is_simplified_for_terminal_ws() {
        let payload = r#"{
            "type":"command_output_chunk",
            "chunk":{
                "command_session_id":"cmdsess-1",
                "request_id":"cmd-1",
                "stream":"stdout",
                "sequence":0,
                "chunk":"hello"
            }
        }"#;

        let simplified = simplify_terminal_event(payload).expect("chunk event simplified");
        assert!(simplified.contains(r#""event":"output""#));
        assert!(simplified.contains(r#""session_id":"cmdsess-1""#));
        assert!(simplified.contains(r#""command_id":"cmd-1""#));
        assert!(simplified.contains(r#""chunk":"hello""#));
    }
}

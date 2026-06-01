use super::*;

use transform::{simple_command_state, simple_session_status, web_terminal_error_response};

pub(crate) async fn open_terminal_session(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<WebTerminalOpenRequest>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }

    // Reject if agent is not online
    if !state
        .kernel
        .agent_queries()
        .is_connected(&request.agent_id)
        .await
    {
        return (
            StatusCode::CONFLICT,
            Json(WebTerminalResponse {
                success: false,
                message: "agent is not online".to_string(),
                data: None::<WebTerminalSessionData>,
            }),
        )
            .into_response();
    }

    let operator = extract_operator_for_request(&state, &headers, None);
    match state
        .kernel
        .command_sessions()
        .open(request.agent_id.clone(), operator.clone())
        .await
    {
        Ok(session) => {
            state.kernel.append_audit_record(
                operator,
                "web_terminal_open".to_string(),
                "agent".to_string(),
                Some(request.agent_id),
                Some(format!("command_session_id={}", session.command_session_id)),
                now_ts(),
            );
            (
                StatusCode::CREATED,
                Json(WebTerminalResponse {
                    success: true,
                    message: "ok".to_string(),
                    data: WebTerminalSessionData {
                        session_id: session.command_session_id,
                        cwd: session.cwd,
                        status: simple_session_status(&session.status),
                    },
                }),
            )
                .into_response()
        }
        Err(error) => web_terminal_error_response(error),
    }
}

pub(crate) async fn get_terminal_session(
    Path(session_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }

    match state.kernel.command_sessions().snapshot(&session_id).await {
        Some(session) => (
            StatusCode::OK,
                Json(WebTerminalResponse {
                    success: true,
                    message: "ok".to_string(),
                    data: WebTerminalSessionData {
                    session_id: session.command_session_id,
                    cwd: session.cwd,
                    status: simple_session_status(&session.status),
                },
            }),
        )
            .into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                detail: format!("command session {} not found", session_id),
                task_id: None,
            }),
        )
            .into_response(),
    }
}

pub(crate) async fn queue_terminal_command(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<WebTerminalCommandRequest>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }

    let operator = extract_operator_for_request(&state, &headers, None);
    match state
        .kernel
        .command_sessions()
        .queue(request.session_id.clone(), request.line.clone())
        .await
    {
        Ok(command) => {
            state.kernel.append_audit_record(
                operator,
                "web_terminal_command".to_string(),
                "command_session".to_string(),
                Some(request.session_id),
                Some(format!(
                    "command_id={} line={}",
                    command.command_id, command.line
                )),
                now_ts(),
            );
            (
                StatusCode::ACCEPTED,
                Json(WebTerminalResponse {
                    success: true,
                    message: "queued".to_string(),
                    data: WebTerminalCommandData {
                        session_id: command.command_session_id,
                        command_id: command.command_id,
                        state: simple_command_state(&command.status),
                    },
                }),
            )
                .into_response()
        }
        Err(error) => web_terminal_error_response(error),
    }
}

pub(crate) async fn close_terminal_session(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<WebTerminalCloseRequest>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }

    let operator = extract_operator_for_request(&state, &headers, None);
    match state
        .kernel
        .command_sessions()
        .close(request.session_id.clone())
        .await
    {
        Ok(session) => {
            state.kernel.append_audit_record(
                operator,
                "web_terminal_close".to_string(),
                "command_session".to_string(),
                Some(request.session_id),
                None,
                now_ts(),
            );
            (
                StatusCode::OK,
                Json(WebTerminalResponse {
                    success: true,
                    message: "closed".to_string(),
                    data: WebTerminalSessionData {
                        session_id: session.command_session_id,
                        cwd: session.cwd,
                        status: simple_session_status(&session.status),
                    },
                }),
            )
                .into_response()
        }
        Err(error) => web_terminal_error_response(error),
    }
}

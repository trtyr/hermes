use super::*;

use helpers::command_session_error_response;

pub(crate) async fn create_command_session(
    Path(agent_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let operator = extract_operator_for_request(&state, &headers, None);
    match state.kernel.agent_queries().persisted(&agent_id).await {
        Ok(Some(agent)) => {
            if agent.is_disabled {
                return (
                    StatusCode::CONFLICT,
                    Json(ApiResponse {
                        success: false,
                        detail: format!(
                            "agent {} is disabled; enable it before opening a command session",
                            agent_id
                        ),
                        task_id: None,
                    }),
                )
                    .into_response();
            }
        }
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(ApiResponse {
                    success: false,
                    detail: format!("agent {} not found", agent_id),
                    task_id: None,
                }),
            )
                .into_response();
        }
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    detail: error.to_string(),
                    task_id: None,
                }),
            )
                .into_response();
        }
    }

    match state
        .kernel
        .command_sessions()
        .open(agent_id.clone(), operator.clone())
        .await
    {
        Ok(session) => {
            state.kernel.append_audit_record(
                operator,
                "open_command_session".to_string(),
                "agent".to_string(),
                Some(agent_id),
                Some(format!("command_session_id={}", session.command_session_id)),
                now_ts(),
            );
            (
                StatusCode::CREATED,
                Json(CommandSessionCreateResponse {
                    success: true,
                    detail: "command session opened".to_string(),
                    session,
                }),
            )
                .into_response()
        }
        Err(error) => command_session_error_response(error),
    }
}

pub(crate) async fn queue_command_execution(
    Path(command_session_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CommandLineRequest>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let operator = extract_operator_for_request(&state, &headers, None);
    match state
        .kernel
        .command_sessions()
        .queue(command_session_id.clone(), request.line.clone())
        .await
    {
        Ok(command) => {
            state.kernel.append_audit_record(
                operator,
                "queue_command_session".to_string(),
                "command_session".to_string(),
                Some(command_session_id),
                Some(format!(
                    "command_id={} line={}",
                    command.command_id, command.line
                )),
                now_ts(),
            );
            (
                StatusCode::ACCEPTED,
                Json(CommandQueueResponse {
                    success: true,
                    detail: "command queued".to_string(),
                    command,
                }),
            )
                .into_response()
        }
        Err(error) => command_session_error_response(error),
    }
}

pub(crate) async fn execute_command_session(
    Path(command_session_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CommandLineRequest>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let operator = extract_operator_for_request(&state, &headers, None);
    match state
        .kernel
        .command_sessions()
        .execute(command_session_id.clone(), request.line.clone())
        .await
    {
        Ok(result) => {
            state.kernel.append_audit_record(
                operator,
                "execute_command_session".to_string(),
                "command_session".to_string(),
                Some(command_session_id),
                Some(format!(
                    "line={} cwd_before={} cwd_after={} exit_code={}",
                    result.line, result.cwd_before, result.cwd_after, result.exit_code
                )),
                now_ts(),
            );
            (
                StatusCode::OK,
                Json(CommandSessionExecuteResponse {
                    success: true,
                    detail: "command session line executed".to_string(),
                    result,
                }),
            )
                .into_response()
        }
        Err(error) => command_session_error_response(error),
    }
}

pub(crate) async fn close_command_session(
    Path(command_session_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let operator = extract_operator_for_request(&state, &headers, None);
    match state
        .kernel
        .command_sessions()
        .close(command_session_id.clone())
        .await
    {
        Ok(session) => {
            state.kernel.append_audit_record(
                operator,
                "close_command_session".to_string(),
                "command_session".to_string(),
                Some(command_session_id),
                None,
                now_ts(),
            );
            (
                StatusCode::OK,
                Json(CommandSessionCreateResponse {
                    success: true,
                    detail: "command session closed".to_string(),
                    session,
                }),
            )
                .into_response()
        }
        Err(error) => command_session_error_response(error),
    }
}

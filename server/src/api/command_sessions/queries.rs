use super::*;

pub(crate) async fn list_command_sessions(
    headers: HeaderMap,
    Query(query): Query<CommandSessionListQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let mut sessions = state.kernel.command_sessions().snapshots().await;
    if let Some(agent_id) = query.agent_id {
        sessions.retain(|session| session.agent_id == agent_id);
    }
    if let Some(status) = query.status {
        match parse_command_session_status(&status) {
            Some(status) => sessions.retain(|session| session.status == status),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse {
                        success: false,
                        detail: format!("unsupported command session status {}", status),
                        task_id: None,
                    }),
                )
                    .into_response();
            }
        }
    }
    let (limit, offset) = normalize_page(query.limit, query.offset);
    let total = sessions.len();
    let sessions = paginate_vec(sessions, limit, offset);
    (
        StatusCode::OK,
        Json(CommandSessionsResponse {
            sessions,
            total,
            limit,
            offset,
        }),
    )
        .into_response()
}

pub(crate) async fn get_command_session(
    Path(command_session_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    match state
        .kernel
        .command_sessions()
        .snapshot(&command_session_id)
        .await
    {
        Some(session) => (StatusCode::OK, Json(session)).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                detail: format!("command session {} not found", command_session_id),
                task_id: None,
            }),
        )
            .into_response(),
    }
}

pub(crate) async fn list_command_executions(
    Path(command_session_id): Path<String>,
    headers: HeaderMap,
    Query(query): Query<CommandExecutionListQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    if state
        .kernel
        .command_sessions()
        .snapshot(&command_session_id)
        .await
        .is_none()
    {
        return (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                detail: format!("command session {} not found", command_session_id),
                task_id: None,
            }),
        )
            .into_response();
    }
    let commands = state
        .kernel
        .command_sessions()
        .command_snapshots(&command_session_id)
        .await;
    let (limit, offset) = normalize_page(query.limit, query.offset);
    let total = commands.len();
    let commands = paginate_vec(commands, limit, offset);
    (
        StatusCode::OK,
        Json(CommandExecutionsResponse {
            commands,
            total,
            limit,
            offset,
        }),
    )
        .into_response()
}

pub(crate) async fn get_command_execution(
    Path((command_session_id, command_id)): Path<(String, String)>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    match state
        .kernel
        .command_sessions()
        .command_snapshot(&command_id)
        .await
    {
        Some(command) if command.command_session_id == command_session_id => {
            (StatusCode::OK, Json(command)).into_response()
        }
        _ => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                detail: format!(
                    "command {} not found in command session {}",
                    command_id, command_session_id
                ),
                task_id: None,
            }),
        )
            .into_response(),
    }
}

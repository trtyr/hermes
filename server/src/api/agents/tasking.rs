use super::*;

pub(crate) async fn dispatch_task(
    Path(agent_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<DispatchTaskRequest>,
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
                            "agent {} is disabled; enable it before dispatching tasks",
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
    let detail = format!(
        "command={} payload={}",
        request.command,
        request.payload.clone().unwrap_or_default()
    );
    let result = state
        .kernel
        .tasks()
        .dispatch_to_agent(agent_id.clone(), request.command, request.payload)
        .await;

    match result {
        Ok(task_id) => {
            state.kernel.append_audit_record(
                operator,
                "dispatch_task".to_string(),
                "agent".to_string(),
                Some(agent_id.clone()),
                Some(detail),
                now_ts(),
            );
            (
                StatusCode::ACCEPTED,
                Json(ApiResponse {
                    success: true,
                    detail: format!("task dispatched to {}", agent_id),
                    task_id: Some(task_id),
                }),
            )
                .into_response()
        }
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                detail: error.to_string(),
                task_id: None,
            }),
        )
            .into_response(),
    }
}

pub(crate) async fn disconnect_agent(
    Path(agent_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let operator = extract_operator_for_request(&state, &headers, None);
    match state.kernel.agent_queries().persisted(&agent_id).await {
        Ok(Some(_)) => {}
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
    if !state.kernel.agent_queries().is_connected(&agent_id).await {
        return (
            StatusCode::CONFLICT,
            Json(ApiResponse {
                success: false,
                detail: format!("agent {} is already offline", agent_id),
                task_id: None,
            }),
        )
            .into_response();
    }
    let result = state
        .kernel
        .agent_commands()
        .disconnect(agent_id.clone())
        .await;

    match result {
        Ok(()) => {
            state.kernel.append_audit_record(
                operator,
                "disconnect_agent".to_string(),
                "agent".to_string(),
                Some(agent_id.clone()),
                None,
                now_ts(),
            );
            (
                StatusCode::ACCEPTED,
                Json(ApiResponse {
                    success: true,
                    detail: format!("disconnect queued for {}", agent_id),
                    task_id: None,
                }),
            )
                .into_response()
        }
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                detail: error.to_string(),
                task_id: None,
            }),
        )
            .into_response(),
    }
}

use super::*;

pub(crate) async fn broadcast_task(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<DispatchTaskRequest>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let operator = extract_operator_for_request(&state, &headers, None);
    let detail = format!(
        "command={} payload={}",
        request.command,
        request.payload.clone().unwrap_or_default()
    );
    let result = state
        .kernel
        .tasks()
        .broadcast(request.command, request.payload)
        .await;

    match result {
        Ok(task_id) => {
            state.kernel.append_audit_record(
                operator,
                "broadcast_task".to_string(),
                "task".to_string(),
                Some(task_id.clone()),
                Some(detail),
                now_ts(),
            );
            (
                StatusCode::ACCEPTED,
                Json(ApiResponse {
                    success: true,
                    detail: "broadcast task queued".to_string(),
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

pub(crate) async fn cancel_task(
    Path(task_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let operator = extract_operator_for_request(&state, &headers, None);
    match state.kernel.tasks().snapshot(&task_id).await {
        Some(task) => {
            if !is_task_cancellable(&task.status) {
                return (
                    StatusCode::CONFLICT,
                    Json(ApiResponse {
                        success: false,
                        detail: format!(
                            "task {} is not cancellable in status {:?}",
                            task_id, task.status
                        ),
                        task_id: Some(task_id),
                    }),
                )
                    .into_response();
            }

            let result = state.kernel.tasks().cancel(task_id.clone()).await;
            match result {
                Ok(()) => {
                    state.kernel.append_audit_record(
                        operator,
                        "cancel_task".to_string(),
                        "task".to_string(),
                        Some(task_id.clone()),
                        Some("cancel requested".to_string()),
                        now_ts(),
                    );
                    (
                        StatusCode::ACCEPTED,
                        Json(ApiResponse {
                            success: true,
                            detail: format!("cancel queued for {}", task_id),
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
                        task_id: Some(task_id),
                    }),
                )
                    .into_response(),
            }
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                detail: format!("task {} not found", task_id),
                task_id: None,
            }),
        )
            .into_response(),
    }
}

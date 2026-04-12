use super::*;

pub(crate) async fn update_agent_beacon_config(
    Path(agent_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AgentBeaconConfigRequest>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    if request.sleep_interval == 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                detail: "sleep_interval must be greater than 0".to_string(),
                task_id: None,
            }),
        )
            .into_response();
    }
    if request.jitter > 100 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                detail: "jitter must be between 0 and 100".to_string(),
                task_id: None,
            }),
        )
            .into_response();
    }

    let operator = extract_operator_for_request(&state, &headers, None);
    let agent = match state.kernel.agent_queries().persisted(&agent_id).await {
        Ok(Some(agent)) => agent,
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
    };

    if agent.is_disabled {
        return (
            StatusCode::CONFLICT,
            Json(ApiResponse {
                success: false,
                detail: format!(
                    "agent {} is disabled; enable it before updating beacon config",
                    agent_id
                ),
                task_id: None,
            }),
        )
            .into_response();
    }

    let result = state
        .kernel
        .agent_commands()
        .update_beacon_config(agent_id.clone(), request.sleep_interval, request.jitter)
        .await;

    match result {
        Ok(_) => match state.kernel.agent_queries().persisted(&agent_id).await {
            Ok(Some(agent)) => {
                state.kernel.append_audit_record(
                    operator,
                    "update_agent_beacon_config".to_string(),
                    "agent".to_string(),
                    Some(agent_id.clone()),
                    Some(format!(
                        "sleep_interval={} jitter={}",
                        request.sleep_interval, request.jitter
                    )),
                    now_ts(),
                );
                (
                    StatusCode::OK,
                    Json(AgentMutationResponse {
                        success: true,
                        detail: format!("agent {} beacon config updated", agent_id),
                        agent,
                    }),
                )
                    .into_response()
            }
            Ok(None) => (
                StatusCode::NOT_FOUND,
                Json(ApiResponse {
                    success: false,
                    detail: format!("agent {} not found", agent_id),
                    task_id: None,
                }),
            )
                .into_response(),
            Err(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    detail: error.to_string(),
                    task_id: None,
                }),
            )
                .into_response(),
        },
        Err(error) => {
            let message = error.to_string();
            let status = if message.contains("offline") || message.contains("timed out") {
                StatusCode::CONFLICT
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            (
                status,
                Json(ApiResponse {
                    success: false,
                    detail: message,
                    task_id: None,
                }),
            )
                .into_response()
        }
    }
}

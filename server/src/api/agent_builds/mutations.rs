use super::*;

pub(crate) async fn create_agent_build(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AgentBuildCreateRequest>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let operator = extract_operator_for_request(&state, &headers, None);

    match state
        .kernel
        .agent_builds()
        .build_agent_binary(
            request.target_triple,
            request.listener_id,
            request.server_addr,
            request.agent_token,
            request.profile,
            request.heartbeat_secs,
            request.jitter,
        )
        .await
    {
        Ok(build) => {
            state.kernel.append_audit_record(
                operator,
                "create_agent_build".to_string(),
                "agent_build".to_string(),
                Some(build.build_id.to_string()),
                Some(format!(
                    "target_triple={} profile={} server_addr={}",
                    build.target_triple, build.profile, build.server_addr
                )),
                now_ts(),
            );
            (
                StatusCode::ACCEPTED,
                Json(AgentBuildCreateResponse {
                    success: true,
                    detail: "agent build created".to_string(),
                    build,
                }),
            )
                .into_response()
        }
        Err(error) => (
            StatusCode::CONFLICT,
            Json(ApiResponse {
                success: false,
                detail: error.to_string(),
                task_id: None,
            }),
        )
            .into_response(),
    }
}

pub(crate) async fn delete_agent_build(
    Path(build_id): Path<i64>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let operator = extract_operator_for_request(&state, &headers, None);

    match state.kernel.agent_builds().delete_build(build_id).await {
        Ok(true) => {
            state.kernel.append_audit_record(
                operator,
                "delete_agent_build".to_string(),
                "agent_build".to_string(),
                Some(build_id.to_string()),
                Some("agent build deleted".to_string()),
                now_ts(),
            );
            (
                StatusCode::OK,
                Json(ApiResponse {
                    success: true,
                    detail: format!("agent build {} deleted", build_id),
                    task_id: None,
                }),
            )
                .into_response()
        }
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                detail: format!("agent build {} not found", build_id),
                task_id: None,
            }),
        )
            .into_response(),
        Err(error) => {
            let detail = error.to_string();
            let status = if detail.contains("still pending") {
                StatusCode::CONFLICT
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            (
                status,
                Json(ApiResponse {
                    success: false,
                    detail,
                    task_id: None,
                }),
            )
                .into_response()
        }
    }
}

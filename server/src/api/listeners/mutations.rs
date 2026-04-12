use super::*;

pub(crate) async fn create_listener(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<ListenerCreateRequest>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let operator = extract_operator_for_request(&state, &headers, None);
    let Some(kind) = parse_listener_kind(&request.kind) else {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                detail: format!("unsupported listener kind {}", request.kind),
                task_id: None,
            }),
        )
            .into_response();
    };

    match state
        .kernel
        .listener_commands()
        .create(
            request.name.clone(),
            kind,
            request.bind_host.clone(),
            request.bind_port,
            request.enabled,
            request.config.clone(),
        )
        .await
    {
        Ok(listener) => {
            state.kernel.append_audit_record(
                operator,
                "create_listener".to_string(),
                "listener".to_string(),
                Some(listener.listener_id.to_string()),
                Some(format!(
                    "name={} kind={} bind={}:{} enabled={}",
                    listener.name,
                    request.kind,
                    listener.bind_host,
                    listener.bind_port,
                    listener.enabled
                )),
                now_ts(),
            );
            (
                StatusCode::CREATED,
                Json(ListenerCreateResponse {
                    success: true,
                    detail: "listener created".to_string(),
                    listener,
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

pub(crate) async fn create_listener_agent_build(
    Path(listener_id): Path<i64>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<ListenerAgentBuildRequest>,
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
            Some(listener_id),
            None,
            request.agent_token,
            request.profile,
        )
        .await
    {
        Ok(build) => {
            state.kernel.append_audit_record(
                operator,
                "create_listener_agent_build".to_string(),
                "agent_build".to_string(),
                Some(build.build_id.to_string()),
                Some(format!(
                    "listener_id={} target_triple={} profile={} server_addr={}",
                    listener_id, build.target_triple, build.profile, build.server_addr
                )),
                now_ts(),
            );
            (
                StatusCode::CREATED,
                Json(AgentBuildCreateResponse {
                    success: true,
                    detail: "listener agent build created".to_string(),
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

pub(crate) async fn update_listener(
    Path(listener_id): Path<i64>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<ListenerUpdateRequest>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let operator = extract_operator_for_request(&state, &headers, None);
    match state
        .kernel
        .listener_commands()
        .update(
            listener_id,
            request.name.clone(),
            request.bind_host.clone(),
            request.bind_port,
            request.config.clone(),
        )
        .await
    {
        Ok(Some(listener)) => {
            state.kernel.append_audit_record(
                operator,
                "update_listener".to_string(),
                "listener".to_string(),
                Some(listener.listener_id.to_string()),
                Some(format!(
                    "name={} bind={}:{}",
                    listener.name, listener.bind_host, listener.bind_port
                )),
                now_ts(),
            );
            (
                StatusCode::OK,
                Json(ListenerCreateResponse {
                    success: true,
                    detail: "listener updated".to_string(),
                    listener,
                }),
            )
                .into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                detail: format!("listener {} not found", listener_id),
                task_id: None,
            }),
        )
            .into_response(),
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

pub(crate) async fn enable_listener(
    Path(listener_id): Path<i64>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    set_listener_enabled(listener_id, headers, state, true).await
}

pub(crate) async fn disable_listener(
    Path(listener_id): Path<i64>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    set_listener_enabled(listener_id, headers, state, false).await
}

async fn set_listener_enabled(
    listener_id: i64,
    headers: HeaderMap,
    state: AppState,
    enabled: bool,
) -> axum::response::Response {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let operator = extract_operator_for_request(&state, &headers, None);
    let action = if enabled {
        "enable_listener"
    } else {
        "disable_listener"
    };

    match state
        .kernel
        .listener_commands()
        .set_enabled(listener_id, enabled)
        .await
    {
        Ok(Some(listener)) => {
            state.kernel.append_audit_record(
                operator,
                action.to_string(),
                "listener".to_string(),
                Some(listener.listener_id.to_string()),
                Some(format!(
                    "name={} bind={}:{} enabled={}",
                    listener.name, listener.bind_host, listener.bind_port, listener.enabled
                )),
                now_ts(),
            );
            (
                StatusCode::OK,
                Json(ListenerCreateResponse {
                    success: true,
                    detail: format!(
                        "listener {} {}",
                        listener_id,
                        if enabled { "enabled" } else { "disabled" }
                    ),
                    listener,
                }),
            )
                .into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                detail: format!("listener {} not found", listener_id),
                task_id: None,
            }),
        )
            .into_response(),
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

pub(crate) async fn delete_listener(
    Path(listener_id): Path<i64>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let operator = extract_operator_for_request(&state, &headers, None);
    match state.kernel.listener_queries().record(listener_id).await {
        Ok(Some(listener)) => {
            if listener.enabled {
                return (
                    StatusCode::CONFLICT,
                    Json(ApiResponse {
                        success: false,
                        detail: format!("listener {} is enabled; disable it first", listener_id),
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
                    detail: format!("listener {} not found", listener_id),
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

    match state.kernel.listener_commands().delete(listener_id).await {
        Ok(true) => {
            state.kernel.append_audit_record(
                operator,
                "delete_listener".to_string(),
                "listener".to_string(),
                Some(listener_id.to_string()),
                Some("listener definition deleted".to_string()),
                now_ts(),
            );
            (
                StatusCode::OK,
                Json(ApiResponse {
                    success: true,
                    detail: format!("listener {} deleted", listener_id),
                    task_id: None,
                }),
            )
                .into_response()
        }
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                detail: format!("listener {} not found", listener_id),
                task_id: None,
            }),
        )
            .into_response(),
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

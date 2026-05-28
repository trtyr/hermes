use super::*;

pub(crate) async fn start_proxy(
    Path(agent_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let operator = extract_operator_for_request(&state, &headers, None);
    match state.kernel.agent_queries().persisted(&agent_id).await {
        Ok(Some(agent)) if agent.is_disabled => {
            return (
                StatusCode::CONFLICT,
                Json(ApiResponse {
                    success: false,
                    detail: format!(
                        "agent {} is disabled; enable it before opening proxy",
                        agent_id
                    ),
                    task_id: None,
                }),
            )
                .into_response();
        }
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

    match state.kernel.proxy().start(agent_id.clone()).await {
        Ok(proxy) => {
            state.kernel.append_audit_record(
                operator,
                "open_proxy".to_string(),
                "agent".to_string(),
                Some(agent_id),
                Some(format!(
                    "proxy_id={} bind_addr={}",
                    proxy.proxy_id, proxy.bind_addr
                )),
                now_ts(),
            );
            (
                StatusCode::CREATED,
                Json(ProxySessionResponse {
                    success: true,
                    detail: "proxy opened".to_string(),
                    proxy,
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

pub(crate) async fn list_proxy(
    Path(agent_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let proxies = state.kernel.proxy().list_for_agent(&agent_id).await;
    (StatusCode::OK, Json(ProxySessionsResponse { proxies })).into_response()
}

pub(crate) async fn delete_proxy(
    Path((agent_id, proxy_id)): Path<(String, String)>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let operator = extract_operator_for_request(&state, &headers, None);
    match state.kernel.proxy().delete(proxy_id.clone()).await {
        Ok(proxy) => {
            state.kernel.append_audit_record(
                operator,
                "delete_proxy".to_string(),
                "agent".to_string(),
                Some(agent_id),
                Some(format!("proxy_id={}", proxy_id)),
                now_ts(),
            );
            (
                StatusCode::OK,
                Json(ProxySessionResponse {
                    success: true,
                    detail: "proxy deleted".to_string(),
                    proxy,
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

use super::*;

pub(crate) async fn list_agent_builds(
    headers: HeaderMap,
    Query(query): Query<AgentBuildListQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let status = match query.status.as_deref() {
        Some(status) => match parse_agent_build_status(status) {
            Some(status) => Some(status),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse {
                        success: false,
                        detail: format!("unsupported agent build status {}", status),
                        task_id: None,
                    }),
                )
                    .into_response();
            }
        },
        None => None,
    };

    match state
        .kernel
        .agent_builds()
        .filtered_records(status, query.target_triple)
        .await
    {
        Ok(builds) => {
            let (limit, offset) = normalize_page(query.limit, query.offset);
            let total = builds.len();
            let builds = paginate_vec(builds, limit, offset);
            (
                StatusCode::OK,
                Json(AgentBuildsResponse {
                    builds,
                    total,
                    limit,
                    offset,
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

pub(crate) async fn get_agent_build(
    Path(build_id): Path<i64>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    match state.kernel.agent_builds().record(build_id).await {
        Ok(Some(build)) => (StatusCode::OK, Json(build)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                detail: format!("agent build {} not found", build_id),
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
    }
}

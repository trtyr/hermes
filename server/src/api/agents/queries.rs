use super::*;

pub(crate) async fn list_agents(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    Json(AgentsResponse {
        agents: state.kernel.agent_queries().snapshots().await,
    })
    .into_response()
}

pub(crate) async fn list_persisted_agents(
    headers: HeaderMap,
    Query(query): Query<AgentHistoryQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    match state
        .kernel
        .agent_queries()
        .filtered_persisted(query.online, query.disabled, query.keyword, query.tag)
        .await
    {
        Ok(agents) => {
            let (limit, offset) = normalize_page(query.limit, query.offset);
            let total = agents.len();
            let agents = paginate_vec(agents, limit, offset);
            (
                StatusCode::OK,
                Json(AgentRecordsResponse {
                    agents,
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

pub(crate) async fn get_agent(
    Path(agent_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    match state.kernel.agent_queries().persisted(&agent_id).await {
        Ok(Some(agent)) => (StatusCode::OK, Json(agent)).into_response(),
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
    }
}

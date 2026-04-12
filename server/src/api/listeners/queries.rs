use super::*;

pub(crate) async fn list_listeners(
    headers: HeaderMap,
    Query(query): Query<ListenerListQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let kind = match query.kind.as_deref() {
        Some(kind) => match parse_listener_kind(kind) {
            Some(kind) => Some(kind),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse {
                        success: false,
                        detail: format!("unsupported listener kind {}", kind),
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
        .listener_queries()
        .filtered_records(query.enabled, kind, query.keyword)
        .await
    {
        Ok(listeners) => {
            let (limit, offset) = normalize_page(query.limit, query.offset);
            let total = listeners.len();
            let listeners = paginate_vec(listeners, limit, offset);
            (
                StatusCode::OK,
                Json(ListenersResponse {
                    listeners,
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

pub(crate) async fn get_listener(
    Path(listener_id): Path<i64>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    match state.kernel.listener_queries().record(listener_id).await {
        Ok(Some(listener)) => (StatusCode::OK, Json(listener)).into_response(),
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

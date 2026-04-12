use super::*;

pub(crate) async fn list_tasks(
    headers: HeaderMap,
    Query(query): Query<TaskListQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let status = match query.status {
        Some(status) => match parse_task_status(&status) {
            Some(status) => Some(status),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse {
                        success: false,
                        detail: format!("unsupported task status {}", status),
                        task_id: None,
                    }),
                )
                    .into_response();
            }
        },
        None => None,
    };

    let tasks = state
        .kernel
        .tasks()
        .filtered_snapshots(status, query.agent_id, query.keyword)
        .await;

    let (limit, offset) = normalize_page(query.limit, query.offset);
    let total = tasks.len();
    let tasks = paginate_vec(tasks, limit, offset);

    (
        StatusCode::OK,
        Json(TasksResponse {
            tasks,
            total,
            limit,
            offset,
        }),
    )
        .into_response()
}

pub(crate) async fn get_task(
    Path(task_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    match state.kernel.tasks().snapshot(&task_id).await {
        Some(task) => (StatusCode::OK, Json(task)).into_response(),
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

use super::*;

use axum::body::Body;
use crate::protocol::AgentBuildStatus;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

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

pub(crate) async fn download_agent_build(
    Path(build_id): Path<i64>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }

    let build = match state.kernel.agent_builds().record(build_id).await {
        Ok(Some(b)) => b,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(ApiResponse {
                    success: false,
                    detail: format!("agent build {} not found", build_id),
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

    let artifact_path = match build.artifact_path {
        Some(p) => p,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(ApiResponse {
                    success: false,
                    detail: format!("agent build {} has no artifact", build_id),
                    task_id: None,
                }),
            )
                .into_response();
        }
    };

    if build.status != AgentBuildStatus::Succeeded {
        return (
            StatusCode::CONFLICT,
            Json(ApiResponse {
                success: false,
                detail: format!("agent build {} status is {:?}, not succeeded", build_id, build.status),
                task_id: None,
            }),
        )
            .into_response();
    }

    let mut file = match File::open(&artifact_path).await {
        Ok(f) => f,
        Err(_) => {
            return (
                StatusCode::NOT_FOUND,
                Json(ApiResponse {
                    success: false,
                    detail: "artifact file not found on disk".to_string(),
                    task_id: None,
                }),
            )
                .into_response();
        }
    };

    let mut buffer = Vec::new();
    if file.read_to_end(&mut buffer).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                detail: "failed to read artifact file".to_string(),
                task_id: None,
            }),
        )
            .into_response();
    }

    let filename = build.artifact_name.unwrap_or_else(|| format!("agent-build-{}", build_id));

    (
        StatusCode::OK,
        [
            ("content-type", "application/octet-stream".to_string()),
            ("content-disposition", format!("attachment; filename=\"{}\"", filename)),
        ],
        Body::from(buffer),
    )
        .into_response()
}

use super::*;

use crate::api::common::{
    ApiResponse, AppState, BrowseFileRequest, FileDownloadRequest, FileUploadRequest,
};
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

pub(crate) async fn upload_file(
    Path(agent_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<FileUploadRequest>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let operator = extract_operator_for_request(&state, &headers, None);
    match state.kernel.agent_queries().persisted(&agent_id).await {
        Ok(Some(agent)) => {
            if agent.is_disabled {
                return (
                    StatusCode::CONFLICT,
                    Json(ApiResponse {
                        success: false,
                        detail: format!(
                            "agent {} is disabled; enable it before dispatching tasks",
                            agent_id
                        ),
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
    let payload = json!({
        "remote_path": request.remote_path,
        "content_base64": request.content_base64,
    })
    .to_string();
    let detail = format!("command=upload remote_path={}", request.remote_path);
    let result = state
        .kernel
        .tasks()
        .dispatch_to_agent(agent_id.clone(), "upload".to_string(), Some(payload))
        .await;

    match result {
        Ok(task_id) => {
            state.kernel.append_audit_record(
                operator,
                "upload_file".to_string(),
                "agent".to_string(),
                Some(agent_id.clone()),
                Some(detail),
                now_ts(),
            );
            (
                StatusCode::ACCEPTED,
                Json(ApiResponse {
                    success: true,
                    detail: format!("upload task dispatched to {}", agent_id),
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

pub(crate) async fn browse_file(
    Path(agent_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<BrowseFileRequest>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let operator = extract_operator_for_request(&state, &headers, None);
    match state.kernel.agent_queries().persisted(&agent_id).await {
        Ok(Some(agent)) => {
            if agent.is_disabled {
                return (
                    StatusCode::CONFLICT,
                    Json(ApiResponse {
                        success: false,
                        detail: format!(
                            "agent {} is disabled; enable it before dispatching tasks",
                            agent_id
                        ),
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
    let payload = build_browse_payload(&request);
    let detail = format!("command=browse path={}", request.path);
    let result = state
        .kernel
        .tasks()
        .dispatch_to_agent(agent_id.clone(), "browse".to_string(), Some(payload))
        .await;

    match result {
        Ok(task_id) => {
            state.kernel.append_audit_record(
                operator,
                "browse_file".to_string(),
                "agent".to_string(),
                Some(agent_id.clone()),
                Some(detail),
                now_ts(),
            );
            (
                StatusCode::ACCEPTED,
                Json(ApiResponse {
                    success: true,
                    detail: format!("browse task dispatched to {}", agent_id),
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

pub(crate) async fn download_file(
    Path(agent_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<FileDownloadRequest>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let operator = extract_operator_for_request(&state, &headers, None);
    match state.kernel.agent_queries().persisted(&agent_id).await {
        Ok(Some(agent)) => {
            if agent.is_disabled {
                return (
                    StatusCode::CONFLICT,
                    Json(ApiResponse {
                        success: false,
                        detail: format!(
                            "agent {} is disabled; enable it before dispatching tasks",
                            agent_id
                        ),
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
    let detail = format!("command=download remote_path={}", request.remote_path);
    let result = state
        .kernel
        .tasks()
        .dispatch_to_agent(
            agent_id.clone(),
            "download".to_string(),
            Some(request.remote_path.clone()),
        )
        .await;

    match result {
        Ok(task_id) => {
            state.kernel.append_audit_record(
                operator,
                "download_file".to_string(),
                "agent".to_string(),
                Some(agent_id.clone()),
                Some(detail),
                now_ts(),
            );
            (
                StatusCode::ACCEPTED,
                Json(ApiResponse {
                    success: true,
                    detail: format!("download task dispatched to {}", agent_id),
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

fn build_browse_payload(request: &BrowseFileRequest) -> String {
    json!({
        "path": request.path,
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_browse_payload_serializes_frontend_shape() {
        let payload = build_browse_payload(&BrowseFileRequest {
            path: "C:\\".to_string(),
        });

        let value: serde_json::Value = serde_json::from_str(&payload).unwrap();
        assert_eq!(value, serde_json::json!({ "path": "C:\\" }));
    }
}

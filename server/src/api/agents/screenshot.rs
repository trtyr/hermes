use super::*;

use crate::api::common::{ApiResponse, AppState};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;

pub(crate) async fn take_screenshot(
    Path(agent_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
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
    let result = state
        .kernel
        .tasks()
        .dispatch_to_agent(agent_id.clone(), "screenshot".to_string(), None)
        .await;

    match result {
        Ok(task_id) => {
            state.kernel.append_audit_record(
                operator,
                "take_screenshot".to_string(),
                "agent".to_string(),
                Some(agent_id.clone()),
                Some("command=screenshot".to_string()),
                now_ts(),
            );
            (
                StatusCode::ACCEPTED,
                Json(ApiResponse {
                    success: true,
                    detail: format!("screenshot task dispatched to {}", agent_id),
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

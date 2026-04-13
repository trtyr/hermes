use super::*;

use crate::api::common::{ApiResponse, AppState};
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct UpdateAgentRequest {
    pub tags: Option<Vec<String>>,
}

pub(crate) async fn update_agent(
    Path(agent_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateAgentRequest>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let operator = extract_operator_for_request(&state, &headers, None);

    match state.kernel.agent_queries().persisted(&agent_id).await {
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

    if let Some(tags) = request.tags {
        match state
            .kernel
            .agent_commands()
            .update_tags(&agent_id, &tags)
            .await
        {
            Ok(true) => {
                state.kernel.append_audit_record(
                    operator,
                    "update_agent_tags".to_string(),
                    "agent".to_string(),
                    Some(agent_id.clone()),
                    Some(format!("tags updated to {:?}", tags)),
                    now_ts(),
                );
                (
                    StatusCode::OK,
                    Json(ApiResponse {
                        success: true,
                        detail: format!("agent {} tags updated", agent_id),
                        task_id: None,
                    }),
                )
                    .into_response()
            }
            Ok(false) => (
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
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                detail: "no fields to update".to_string(),
                task_id: None,
            }),
        )
            .into_response()
    }
}

pub(crate) async fn disable_agent(
    Path(agent_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let operator = extract_operator_for_request(&state, &headers, None);
    let agent = match state.kernel.agent_queries().persisted(&agent_id).await {
        Ok(Some(agent)) => agent,
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
    };

    if agent.is_disabled {
        return (
            StatusCode::CONFLICT,
            Json(ApiResponse {
                success: false,
                detail: format!("agent {} is already disabled", agent_id),
                task_id: None,
            }),
        )
            .into_response();
    }

    match state
        .kernel
        .agent_commands()
        .set_disabled(&agent_id, true)
        .await
    {
        Ok(true) => {
            if state.kernel.agent_queries().is_connected(&agent_id).await {
                let _ = state
                    .kernel
                    .agent_commands()
                    .disconnect(agent_id.clone())
                    .await;
            }
            state.kernel.append_audit_record(
                operator,
                "disable_agent".to_string(),
                "agent".to_string(),
                Some(agent_id.clone()),
                Some("agent disabled; new registration and task dispatch blocked".to_string()),
                now_ts(),
            );
            state.kernel.publish_web_event(WebEvent::AgentDisabled {
                agent_id: agent_id.clone(),
            });
            (
                StatusCode::ACCEPTED,
                Json(ApiResponse {
                    success: true,
                    detail: format!("agent {} disabled", agent_id),
                    task_id: None,
                }),
            )
                .into_response()
        }
        Ok(false) => (
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

pub(crate) async fn enable_agent(
    Path(agent_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let operator = extract_operator_for_request(&state, &headers, None);
    let agent = match state.kernel.agent_queries().persisted(&agent_id).await {
        Ok(Some(agent)) => agent,
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
    };

    if !agent.is_disabled {
        return (
            StatusCode::CONFLICT,
            Json(ApiResponse {
                success: false,
                detail: format!("agent {} is not disabled", agent_id),
                task_id: None,
            }),
        )
            .into_response();
    }

    match state
        .kernel
        .agent_commands()
        .set_disabled(&agent_id, false)
        .await
    {
        Ok(true) => {
            state.kernel.append_audit_record(
                operator,
                "enable_agent".to_string(),
                "agent".to_string(),
                Some(agent_id.clone()),
                Some("agent enabled; registration and task dispatch allowed".to_string()),
                now_ts(),
            );
            state.kernel.publish_web_event(WebEvent::AgentEnabled {
                agent_id: agent_id.clone(),
            });
            (
                StatusCode::OK,
                Json(ApiResponse {
                    success: true,
                    detail: format!("agent {} enabled", agent_id),
                    task_id: None,
                }),
            )
                .into_response()
        }
        Ok(false) => (
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

pub(crate) async fn delete_agent(
    Path(agent_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let operator = extract_operator_for_request(&state, &headers, None);

    match state.kernel.agent_queries().persisted(&agent_id).await {
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

    if state.kernel.agent_queries().is_connected(&agent_id).await {
        return (
            StatusCode::CONFLICT,
            Json(ApiResponse {
                success: false,
                detail: format!(
                    "agent {} is online; disconnect it first before deleting its record",
                    agent_id
                ),
                task_id: None,
            }),
        )
            .into_response();
    }

    match state
        .kernel
        .agent_commands()
        .delete_persisted(&agent_id)
        .await
    {
        Ok(true) => {
            state.kernel.append_audit_record(
                operator,
                "delete_agent".to_string(),
                "agent".to_string(),
                Some(agent_id.clone()),
                Some("removed persisted agent record; task/audit history retained".to_string()),
                now_ts(),
            );
            state.kernel.publish_web_event(WebEvent::AgentDeleted {
                agent_id: agent_id.clone(),
            });
            (
                StatusCode::OK,
                Json(ApiResponse {
                    success: true,
                    detail: format!("agent {} record deleted", agent_id),
                    task_id: None,
                }),
            )
                .into_response()
        }
        Ok(false) => (
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

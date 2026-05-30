// Audit API handlers: persisted audit history queries.
use super::*;

use axum::extract::{Query, State};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub(super) struct AuditsResponse {
    audits: Vec<crate::protocol::AuditRecord>,
    total: usize,
    limit: usize,
    offset: usize,
}

#[derive(Deserialize)]
pub(super) struct AuditListQuery {
    operator: Option<String>,
    action: Option<String>,
    target_kind: Option<String>,
    target_id: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
}

pub(super) async fn list_audits(
    headers: HeaderMap,
    Query(query): Query<AuditListQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    match state
        .kernel
        .filtered_audit_records(
            query.operator,
            query.action,
            query.target_kind,
            query.target_id,
        )
        .await
    {
        Ok(audits) => {
            let (limit, offset) = normalize_page(query.limit, query.offset);
            let total = audits.len();
            let audits = paginate_vec(audits, limit, offset);
            (
                StatusCode::OK,
                Json(AuditsResponse {
                    audits,
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

pub(super) async fn clear_audits(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let operator = extract_operator_for_request(&state, &headers, None);
    match state.kernel.clear_audit_records() {
        Ok(deleted) => {
            state.kernel.append_audit_record(
                operator,
                "clear_audits".to_string(),
                "audit".to_string(),
                None,
                Some(format!("deleted {} records", deleted)),
                now_ts(),
            );
            (
                StatusCode::OK,
                Json(ApiResponse {
                    success: true,
                    detail: format!("cleared {} audit records", deleted),
                    task_id: None,
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

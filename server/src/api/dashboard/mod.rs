use super::*;

use axum::extract::State;

mod host;
mod summary;

#[cfg(test)]
mod tests;

use host::collect_host_ops_summary;
use summary::{summarize_agents, summarize_listeners};

pub(crate) async fn dashboard_stats(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }

    let agent_records = match state
        .kernel
        .agent_queries()
        .filtered_persisted(None, None, None, None)
        .await
    {
        Ok(records) => records,
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

    let live_sessions = state.kernel.agent_queries().snapshots().await.len();

    let listeners = match state
        .kernel
        .listener_queries()
        .filtered_records(None, None, None)
        .await
    {
        Ok(records) => records,
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

    (
        StatusCode::OK,
        Json(DashboardOverviewResponse {
            generated_at: now_ts(),
            server: collect_host_ops_summary(),
            agents: summarize_agents(&agent_records, live_sessions),
            listeners: summarize_listeners(&listeners),
        }),
    )
        .into_response()
}

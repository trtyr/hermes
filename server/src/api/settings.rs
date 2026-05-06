use super::*;

use axum::extract::State;
use serde::Deserialize;

use crate::kernel::{AgentAuthMode, Config};

#[derive(Serialize)]
pub(crate) struct AuthSettingsResponse {
    agent_token: Option<String>,
    agent_auth_mode: String,
}

#[derive(Deserialize)]
pub(crate) struct UpdateAuthSettingsRequest {
    agent_token: Option<String>,
    agent_auth_mode: Option<String>,
}

pub(crate) async fn get_auth_settings(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if let Some(err) = authorize_api(&state, &headers, None) {
        return err;
    }

    let cfg = state.kernel.agent_auth_config().read().unwrap();
    let mode_str = match cfg.agent_auth_mode {
        AgentAuthMode::PlainToken => "plain_token",
        AgentAuthMode::ChallengeResponse => "challenge_response",
    };

    (
        StatusCode::OK,
        Json(AuthSettingsResponse {
            agent_token: cfg.agent_token.clone(),
            agent_auth_mode: mode_str.to_string(),
        }),
    )
        .into_response()
}

pub(crate) async fn update_auth_settings(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<UpdateAuthSettingsRequest>,
) -> impl IntoResponse {
    if let Some(err) = authorize_api(&state, &headers, None) {
        return err;
    }

    let new_mode = match request.agent_auth_mode.as_deref() {
        Some("plain_token") | None => AgentAuthMode::PlainToken,
        Some("challenge_response") => AgentAuthMode::ChallengeResponse,
        Some(other) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse {
                    success: false,
                    detail: format!("invalid agent_auth_mode: {}", other),
                    task_id: None,
                }),
            )
                .into_response()
        }
    };

    let new_token = request.agent_token;

    // Persist to config.toml
    if let Err(e) = Config::write_auth_config(new_token.clone(), new_mode) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                detail: format!("failed to persist config: {}", e),
                task_id: None,
            }),
        )
            .into_response();
    }

    // Update the in-memory shared config — the listener manager's
    // reconcile loop will pick up the new values within ~1 second.
    {
        let mut cfg = state.kernel.agent_auth_config().write().unwrap();
        cfg.agent_token = new_token.clone();
        cfg.agent_auth_mode = new_mode;
    }

    let mode_str = match new_mode {
        AgentAuthMode::PlainToken => "plain_token",
        AgentAuthMode::ChallengeResponse => "challenge_response",
    };

    (
        StatusCode::OK,
        Json(AuthSettingsResponse {
            agent_token: new_token,
            agent_auth_mode: mode_str.to_string(),
        }),
    )
        .into_response()
}

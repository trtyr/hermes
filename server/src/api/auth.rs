use super::*;

use axum::extract::State;

pub(super) async fn login(
    State(state): State<AppState>,
    Json(request): Json<AuthLoginRequest>,
) -> impl IntoResponse {
    if !state.kernel.auth().web_login_configured() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ApiResponse {
                success: false,
                detail: "backend login is not configured".to_string(),
                task_id: None,
            }),
        )
            .into_response();
    }

    let Some(username) = state
        .kernel
        .auth()
        .validate_web_credentials(&request.username, &request.password)
    else {
        return unauthorized_response();
    };

    match state.kernel.auth().create_session(&username) {
        Ok(session) => (
            StatusCode::OK,
            [(
                header::SET_COOKIE,
                session_cookie_header(
                    &session.session_token,
                    state.kernel.auth().session_ttl_secs(),
                ),
            )],
            Json(AuthLoginResponse {
                success: true,
                detail: "login ok".to_string(),
                session_token: session.session_token,
                username: session.username,
                expires_at: session.expires_at,
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

pub(super) async fn logout(headers: HeaderMap, State(state): State<AppState>) -> impl IntoResponse {
    let Some(identity) = resolve_auth_identity(&state, &headers, None) else {
        return unauthorized_response();
    };

    if let Some(session_token) = identity.session_token {
        let _ = state.kernel.auth().remove_session(&session_token);
    }

    (
        StatusCode::OK,
        [(header::SET_COOKIE, expired_session_cookie_header())],
        Json(ApiResponse {
            success: true,
            detail: "logout ok".to_string(),
            task_id: None,
        }),
    )
        .into_response()
}

pub(super) async fn me(headers: HeaderMap, State(state): State<AppState>) -> impl IntoResponse {
    let Some(identity) = resolve_auth_identity(&state, &headers, None) else {
        return unauthorized_response();
    };

    if identity.via_legacy_api_token {
        return (
            StatusCode::OK,
            Json(AuthMeResponse {
                authenticated: true,
                username: "legacy-api-token".to_string(),
                expires_at: 0,
            }),
        )
            .into_response();
    }

    let Some(session_token) = identity.session_token else {
        return unauthorized_response();
    };
    let Some(session) = state.kernel.auth().lookup_session_token(&session_token) else {
        return unauthorized_response();
    };

    (
        StatusCode::OK,
        Json(AuthMeResponse {
            authenticated: true,
            username: session.username,
            expires_at: session.expires_at,
        }),
    )
        .into_response()
}

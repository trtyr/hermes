use super::*;

pub(crate) fn is_task_cancellable(status: &TaskStatus) -> bool {
    matches!(
        status,
        TaskStatus::Pending | TaskStatus::Dispatched | TaskStatus::Running
    )
}

pub(crate) fn extract_operator(headers: &HeaderMap) -> String {
    headers
        .get("x-operator")
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("unknown")
        .to_string()
}

pub(crate) fn extract_operator_for_request(
    state: &AppState,
    headers: &HeaderMap,
    query_token: Option<&str>,
) -> String {
    let explicit = extract_operator(headers);
    if explicit != "unknown" {
        return explicit;
    }
    resolve_auth_identity(state, headers, query_token)
        .and_then(|identity| identity.username)
        .unwrap_or(explicit)
}

pub(crate) fn authorize_api(
    state: &AppState,
    headers: &HeaderMap,
    query_token: Option<&str>,
) -> Option<axum::response::Response> {
    if !state.kernel.auth().auth_required() {
        return None;
    }
    match resolve_auth_identity(state, headers, query_token) {
        Some(_) => None,
        None => Some(unauthorized_response()),
    }
}

pub(crate) fn unauthorized_response() -> axum::response::Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(ApiResponse {
            success: false,
            detail: "unauthorized".to_string(),
            task_id: None,
        }),
    )
        .into_response()
}

pub(crate) fn resolve_auth_identity(
    state: &AppState,
    headers: &HeaderMap,
    query_token: Option<&str>,
) -> Option<AuthIdentity> {
    let provided = extract_auth_token(headers, query_token)?;
    state.kernel.auth().resolve_token(provided)
}

pub(crate) fn session_cookie_header(session_token: &str, ttl_secs: u64) -> String {
    format!(
        "{name}={value}; Max-Age={ttl}; Path=/; HttpOnly; SameSite=Lax",
        name = SESSION_COOKIE_NAME,
        value = session_token,
        ttl = ttl_secs
    )
}

pub(crate) fn expired_session_cookie_header() -> String {
    format!(
        "{name}=; Max-Age=0; Path=/; HttpOnly; SameSite=Lax",
        name = SESSION_COOKIE_NAME
    )
}

fn extract_auth_token<'a>(headers: &'a HeaderMap, query_token: Option<&'a str>) -> Option<&'a str> {
    let bearer = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let session_header = headers
        .get("x-session-token")
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let legacy = headers
        .get("x-api-token")
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let cookie = extract_cookie(headers, SESSION_COOKIE_NAME);

    bearer
        .or(session_header)
        .or(cookie)
        .or(legacy)
        .or(query_token.map(str::trim).filter(|value| !value.is_empty()))
}

fn extract_cookie<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    let raw = headers.get(header::COOKIE)?.to_str().ok()?;
    raw.split(';')
        .filter_map(|part| {
            let (key, value) = part.trim().split_once('=')?;
            if key.trim() == name {
                Some(value.trim())
            } else {
                None
            }
        })
        .find(|value| !value.is_empty())
}

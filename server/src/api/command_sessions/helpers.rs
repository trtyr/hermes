use super::*;

pub(super) fn command_session_error_response(error: anyhow::Error) -> axum::response::Response {
    let status = if crate::kernel::is_command_session_timeout(&error) {
        StatusCode::GATEWAY_TIMEOUT
    } else {
        StatusCode::CONFLICT
    };
    (
        status,
        Json(ApiResponse {
            success: false,
            detail: error.to_string(),
            task_id: None,
        }),
    )
        .into_response()
}

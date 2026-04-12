use super::*;

pub(crate) async fn create_agent_build(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AgentBuildCreateRequest>,
) -> impl IntoResponse {
    if let Some(response) = authorize_api(&state, &headers, None) {
        return response;
    }
    let operator = extract_operator_for_request(&state, &headers, None);

    match state
        .kernel
        .agent_builds()
        .build_agent_binary(
            request.target_triple,
            request.listener_id,
            request.server_addr,
            request.agent_token,
            request.profile,
        )
        .await
    {
        Ok(build) => {
            state.kernel.append_audit_record(
                operator,
                "create_agent_build".to_string(),
                "agent_build".to_string(),
                Some(build.build_id.to_string()),
                Some(format!(
                    "target_triple={} profile={} server_addr={}",
                    build.target_triple, build.profile, build.server_addr
                )),
                now_ts(),
            );
            (
                StatusCode::CREATED,
                Json(AgentBuildCreateResponse {
                    success: true,
                    detail: "agent build created".to_string(),
                    build,
                }),
            )
                .into_response()
        }
        Err(error) => (
            StatusCode::CONFLICT,
            Json(ApiResponse {
                success: false,
                detail: error.to_string(),
                task_id: None,
            }),
        )
            .into_response(),
    }
}

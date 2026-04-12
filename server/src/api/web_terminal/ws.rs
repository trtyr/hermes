use super::*;

use axum::extract::ws::Message;
use serde_json::json;
use transform::simplify_terminal_event;

pub(crate) async fn terminal_ws_events(
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    Query(query): Query<WsAuthQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let query_token = query
        .session_token
        .as_deref()
        .or(query.api_token.as_deref());
    if let Some(response) = authorize_api(&state, &headers, query_token) {
        return response;
    }
    ws.on_upgrade(move |socket| handle_terminal_ws(socket, state))
}

async fn handle_terminal_ws(mut socket: WebSocket, state: AppState) {
    if socket
        .send(Message::Text(
            json!({
                "type": "terminal",
                "event": "connected",
                "state": "connected",
            })
            .to_string()
            .into(),
        ))
        .await
        .is_err()
    {
        return;
    }

    let mut event_rx = state.kernel.events().subscribe();

    loop {
        tokio::select! {
            event = event_rx.recv() => {
                match event {
                    Ok(payload) => {
                        let Some(simple_payload) = simplify_terminal_event(&payload) else {
                            continue;
                        };
                        if socket.send(Message::Text(simple_payload.into())).await.is_err() {
                            break;
                        }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                }
            }
            inbound = socket.recv() => {
                match inbound {
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(_)) => {}
                    Some(Err(_)) => break,
                }
            }
        }
    }
}

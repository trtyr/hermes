use super::*;

use crate::protocol::WebEvent;

pub(crate) async fn ws_events(
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
    ws.on_upgrade(move |socket| handle_ws(socket, state))
}

async fn handle_ws(mut socket: WebSocket, state: AppState) {
    let snapshot = WebEvent::Snapshot {
        agents: state.kernel.agent_queries().snapshots().await,
    };

    if send_ws_event(&mut socket, &snapshot).await.is_err() {
        return;
    }

    let mut event_rx = state.kernel.events().subscribe();

    loop {
        tokio::select! {
            event = event_rx.recv() => {
                match event {
                    Ok(payload) => {
                        if socket.send(Message::Text(payload.into())).await.is_err() {
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

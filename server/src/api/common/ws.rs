use super::*;

pub(crate) async fn send_ws_event(
    socket: &mut axum::extract::ws::WebSocket,
    event: &WebEvent,
) -> Result<(), ()> {
    let payload = serde_json::to_string(event).map_err(|_| ())?;
    socket
        .send(axum::extract::ws::Message::Text(payload.into()))
        .await
        .map_err(|_| ())
}

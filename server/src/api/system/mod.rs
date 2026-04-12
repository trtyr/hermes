// System API handlers: health, docs, OpenAPI, and websocket event streaming.
use super::*;

use axum::extract::{
    Query, State,
    ws::{Message, WebSocket, WebSocketUpgrade},
};

mod docs;
mod events;
mod health;

pub(crate) use docs::{api_docs, openapi_spec};
pub(crate) use events::ws_events;
pub(crate) use health::health;

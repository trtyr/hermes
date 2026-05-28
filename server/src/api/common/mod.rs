use axum::{
    Json,
    http::{HeaderMap, StatusCode, header},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::{
    kernel::{AuthIdentity, KernelHandle, SESSION_COOKIE_NAME},
    protocol::{
        ListenerKind, ListenerRecord, TaskStatus, WebEvent,
    },
};

mod app_state;
mod auth;
#[cfg(test)]
mod auth_tests;
mod paging;
mod requests;
mod responses;
mod ws;

pub(crate) use app_state::*;
pub(crate) use auth::*;
pub(crate) use paging::*;
pub(crate) use requests::*;
pub(crate) use responses::*;
pub(crate) use ws::*;

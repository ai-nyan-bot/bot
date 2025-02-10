// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::http::error::HttpError;
use crate::http::json::JsonReq;
use crate::http::state::AppState;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use base::model::AuthenticatedUser;
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct SendRequest {}

#[derive(Debug, Deserialize, Serialize)]
pub struct SendResponse {}

pub async fn send(
    Path(id): Path<String>,
    Extension(user): Extension<AuthenticatedUser>,
    State(_state): State<AppState>,
    JsonReq(req): JsonReq<SendRequest>,
) -> Result<Json<SendResponse>, HttpError> {
    debug!("POST /v1/wallets/{}/send {:?}", id, req);

    println!("{:#?}", user);

    Ok(Json(SendResponse {}))
}

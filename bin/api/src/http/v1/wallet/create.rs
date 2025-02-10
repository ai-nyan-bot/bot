// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use axum::extract::State;
use axum::Json;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::http::error::HttpError;
use crate::http::json::JsonReq;
use crate::http::state::AppState;

#[derive(Deserialize, Debug)]
pub struct CreateWalletRequest {}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateWalletResponse {}

pub async fn create(
    State(_state): State<AppState>,
    JsonReq(req): JsonReq<CreateWalletRequest>,
) -> Result<Json<CreateWalletResponse>, HttpError> {
    debug!("POST /v1/wallets {:?}", req);

    Ok(Json(CreateWalletResponse {}))
}

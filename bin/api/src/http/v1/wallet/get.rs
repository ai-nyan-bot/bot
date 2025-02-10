// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use axum::extract::{Path, State};
use axum::Json;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::http::error::HttpError;
use crate::http::state::AppState;

#[derive(Debug, Deserialize, Serialize)]
pub struct GetWalletResponse {}

pub async fn get(
    Path(id): Path<String>,
    State(_state): State<AppState>,
) -> Result<Json<GetWalletResponse>, HttpError> {
    debug!("GET /v1/wallets/{}", id);

    Ok(Json(GetWalletResponse {}))
}

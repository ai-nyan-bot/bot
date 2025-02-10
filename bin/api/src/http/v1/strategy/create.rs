// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::http::error::HttpError;
use crate::http::json::JsonReq;
use crate::http::model::strategy::{StrategyCreateRequest, StrategyCreateResponse};
use crate::http::state::AppState;
use axum::extract::State;
use axum::{Extension, Json};
use base::model::AuthenticatedUser;
use log::debug;
use std::os::linux::raw::stat;

pub async fn create(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    JsonReq(req): JsonReq<StrategyCreateRequest>,
) -> Result<Json<StrategyCreateResponse>, HttpError> {
    debug!("POST /v1/strategies {:?}", req);

    let result = state.strategy_service().create(user).await?;

    Ok(Json(StrategyCreateResponse { id: result.id }))
}

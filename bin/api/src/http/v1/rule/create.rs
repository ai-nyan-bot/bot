// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::http::error::HttpError;
use crate::http::json::JsonReq;
use crate::http::model::rule::{RuleCreateRequest, RuleCreateResponse};
use crate::http::state::AppState;
use axum::extract::State;
use axum::{Extension, Json};
use base::model::AuthenticatedUser;
use base::service::RuleCreateCmd;
use log::debug;
use std::os::linux::raw::stat;

pub async fn create(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    JsonReq(req): JsonReq<RuleCreateRequest>,
) -> Result<Json<RuleCreateResponse>, HttpError> {
    debug!("POST /v1/rules {:?}", req);

    let result = state
        .rule_service()
        .create(
            RuleCreateCmd {
                name: req.name,
                sequence: req.sequence,
            },
            user,
        )
        .await?;

    Ok(Json(RuleCreateResponse { id: result.id }))
}

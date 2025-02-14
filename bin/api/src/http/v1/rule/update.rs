// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::http::error::HttpError;
use crate::http::json::JsonReq;
use crate::http::model::rule::{RuleUpdateRequest, RuleUpdateResponse};
use crate::http::state::AppState;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use base::model::AuthenticatedUser;
use base::service::RuleUpdateCmd;
use log::debug;
use std::os::linux::raw::stat;

pub async fn update(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    JsonReq(req): JsonReq<RuleUpdateRequest>,
) -> Result<Json<RuleUpdateResponse>, HttpError> {
    debug!("PATCH /v1/rules/{id} {:?}", req);

    let result = state
        .rule_service()
        .update(
            id,
            RuleUpdateCmd {
                name: req.name,
                sequence: req.sequence,
            },
            user,
        )
        .await?;

    Ok(Json(RuleUpdateResponse { id: 1.into() }))
}

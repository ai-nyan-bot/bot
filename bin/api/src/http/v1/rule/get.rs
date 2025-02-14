// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::http::error::HttpError;
use crate::http::json::JsonReq;
use crate::http::model::rule::{HttpRulGetResponse, HttpRuleList, HttpRuleListResponse, RuleCreateRequest, RuleCreateResponse};
use crate::http::state::AppState;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use base::model::{AuthenticatedUser, RuleId};
use log::debug;
use std::os::linux::raw::stat;

pub async fn get(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
) -> Result<Json<HttpRulGetResponse>, HttpError> {
    debug!("GET /v1/rules/{id}");

    let r = state
        .service
        .rule
        .list_user(user.id)
        .await?
        .into_iter()
        .find(|r| r.id == RuleId(id.parse::<i32>().unwrap()))
        .unwrap();

    Ok(Json(HttpRulGetResponse {
        id: r.id,
        name: r.name,
        sequence: r.sequence,
    }))
}

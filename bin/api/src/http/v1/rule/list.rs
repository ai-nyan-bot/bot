// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::http::error::HttpError;
use crate::http::model::rule::{HttpRuleList, HttpRuleListResponse};
use crate::http::state::AppState;
use axum::extract::State;
use axum::{Extension, Json};
use base::model::AuthenticatedUser;
use log::debug;

pub async fn list(State(state): State<AppState>, Extension(user): Extension<AuthenticatedUser>) -> Result<Json<HttpRuleListResponse>, HttpError> {
    debug!("GET /v1/rules");

    let rules = state.service.rule.list_user(user.id).await?;

    Ok(Json(HttpRuleListResponse {
        rules: rules.into_iter().map(|r| HttpRuleList { id: r.id, name: r.name }).collect(),
    }))
}

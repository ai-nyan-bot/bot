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

#[cfg(test)]
mod tests {
    use crate::http::model::rule::HttpRuleListResponse;
    use crate::http::testing::{extract, extract_error, Test};
    use axum::http::StatusCode;

    #[tokio::test]
    async fn no_rules() {
        let test = Test::new().await;
        let response = test.get_as_test_user("/v1/rules").await;
        assert_eq!(response.status(), StatusCode::OK);

        let response = extract::<HttpRuleListResponse>(response).await.unwrap();
        assert_eq!(response.rules.len(), 0);
    }

    #[tokio::test]
    async fn requires_authentication() {
        let test = Test::new_empty_db().await;
        let response = test.get_unauthenticated("/v1/rules").await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let error = extract_error(response).await;
        assert_eq!(error.code, StatusCode::NOT_FOUND);
        assert_eq!(error.message, "User not found");
    }
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::http::error::HttpError;
use crate::http::model::rule::HttpRulGetResponse;
use crate::http::state::AppState;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use base::model::{AuthenticatedUser, RuleId};
use log::debug;

pub async fn get(
    Path(id): Path<RuleId>,
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
) -> Result<Json<HttpRulGetResponse>, HttpError> {
    debug!("GET /v1/rules/{id}");

    let r = state.service.rule.get_by_id_user(id, user.id).await?;

    Ok(Json(HttpRulGetResponse {
        id: r.id,
        name: r.name,
        sequence: r.sequence,
    }))
}

#[cfg(test)]
mod tests {
    use crate::http::model::rule::HttpRulGetResponse;
    use crate::http::testing::{extract, extract_error, Test};
    use axum::http::StatusCode;
    use testing::rule::create_rule_for_test_user;

    #[tokio::test]
    async fn ok() {
        let test = Test::new().await;

        test.tx(|mut tx| async move {
            create_rule_for_test_user(&mut tx, "MoneyMaker").await;
            tx.commit().await.unwrap()
        })
        .await;

        let response = test.get_as_test_user("/v1/rules/4").await;
        assert_eq!(response.status(), StatusCode::OK);

        let response = extract::<HttpRulGetResponse>(response).await.unwrap();
        assert_eq!(response.id, 4);
        assert_eq!(response.name, "MoneyMaker");
    }

    #[tokio::test]
    async fn not_found() {
        let test = Test::new().await;
        let response = test.get_as_test_user("/v1/rules/1234").await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let error = extract_error(response).await;
        assert_eq!(error.code, StatusCode::NOT_FOUND);
        assert_eq!(error.message, "Rule not found");
    }

    #[tokio::test]
    async fn belongs_to_another_user() {
        let test = Test::new().await;
        let response = test.get_as_another_user("/v1/rules/1").await;
        assert_eq!(response.status(), StatusCode::OK);

        let response = test.get_as_test_user("/v1/rules/1").await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let error = extract_error(response).await;
        assert_eq!(error.code, StatusCode::NOT_FOUND);
        assert_eq!(error.message, "Rule not found");
    }

    #[tokio::test]
    async fn requires_authentication() {
        let test = Test::new_empty_db().await;
        let response = test.get_unauthenticated("/v1/rules/4").await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        let error = extract_error(response).await;
        assert_eq!(error.code, StatusCode::FORBIDDEN);
        assert_eq!(error.message, "User not found");
    }
}

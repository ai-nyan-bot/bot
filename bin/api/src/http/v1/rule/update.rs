// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::http::error::HttpError;
use crate::http::json::JsonReq;
use crate::http::model::rule::{HttpRuleUpdateRequest, HttpRuleUpdateResponse};
use crate::http::state::AppState;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use base::model::AuthenticatedUser;
use base::service::RuleUpdateCmd;
use log::debug;

pub async fn update(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    JsonReq(req): JsonReq<HttpRuleUpdateRequest>,
) -> Result<Json<HttpRuleUpdateResponse>, HttpError> {
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

    Ok(Json(HttpRuleUpdateResponse {
        id: result.id,
        name: result.name,
        sequence: result.sequence,
    }))
}

#[cfg(test)]
mod tests {
    use crate::http::model::rule::HttpRuleUpdateResponse;
    use crate::http::testing::{extract, extract_error, Test};
    use axum::http::StatusCode;
    use base::model::Field::PriceAvg;
    use base::model::Operator::MoreThan;
    use base::model::{Action, Condition, TelegramActionButtonConfig, Value};
    use common::model::Timeframe::M15;
    use testing::rule::create_rule_for_test_user;
    use Condition::Compare;

    #[tokio::test]
    async fn ok() {
        let test = Test::new().await;

        test.tx(|mut tx| async move {
            create_rule_for_test_user(&mut tx, "MoneyMaker").await;
            tx.commit().await.unwrap()
        })
        .await;

        let response = test.patch_json_as_test_user(
			"/v1/rules/4",
			r#"{"name":"UpdatedMoneyMaker","sequence":{"condition":{"id":"root","type":"OR","conditions":[]},
			"action":{"type":"NOTIFY_TELEGRAM",
				"buttons":[
					{"action":"NONE"},
					{"action":"BUY","value":{"type":"SOL","value":"1.2"}},
					{"action":"SELL","value":{"type":"PERCENT","value":3.4}},
					{"action":"NONE"},
					{"action":"NONE"},
					{"action":"NONE"}]}}}"#,
		).await;
        assert_eq!(response.status(), StatusCode::OK);

        let response = extract::<HttpRuleUpdateResponse>(response).await.unwrap();
        assert_eq!(response.id, 4);
        assert_eq!(response.name, "UpdatedMoneyMaker");
        assert_eq!(
            response.sequence.condition,
            Condition::Or { conditions: vec![] }
        );

        let Action::NotifyTelegram { buttons } = response.sequence.action else {
            panic!()
        };
        assert_eq!(buttons.len(), 6);
        assert_eq!(buttons.get(0).unwrap(), &TelegramActionButtonConfig::None);
        assert_eq!(
            buttons.get(1).unwrap(),
            &TelegramActionButtonConfig::Buy {
                value: Value::sol_from_str("1.2")
            }
        );
        assert_eq!(
            buttons.get(2).unwrap(),
            &TelegramActionButtonConfig::Sell {
                value: Value::percent(3.4)
            }
        );
    }

    #[tokio::test]
    async fn empty_json_object() {
        let test = Test::new().await;

        test.tx(|mut tx| async move {
            create_rule_for_test_user(&mut tx, "MoneyMaker").await;
            tx.commit().await.unwrap()
        })
        .await;

        let response = test.patch_json_as_test_user("/v1/rules/4", "{}").await;
        assert_eq!(response.status(), StatusCode::OK);

        let response = extract::<HttpRuleUpdateResponse>(response).await.unwrap();
        assert_eq!(response.id, 4);
        assert_eq!(response.name, "MoneyMaker");
        assert_eq!(
            response.sequence.condition,
            Compare {
                field: PriceAvg,
                operator: MoreThan,
                value: Value::percent(2.0),
                timeframe: Some(M15)
            }
        );
    }

    #[tokio::test]
    async fn partial_update() {
        let test = Test::new().await;

        test.tx(|mut tx| async move {
            create_rule_for_test_user(&mut tx, "MoneyMaker").await;
            tx.commit().await.unwrap()
        })
        .await;

        let response = test
            .patch_json_as_test_user("/v1/rules/4", r#"{"name":"NameWasUpdated"}"#)
            .await;
        assert_eq!(response.status(), StatusCode::OK);

        let response = extract::<HttpRuleUpdateResponse>(response).await.unwrap();
        assert_eq!(response.id, 4);
        assert_eq!(response.name, "NameWasUpdated");
        assert_eq!(
            response.sequence.condition,
            Compare {
                field: PriceAvg,
                operator: MoreThan,
                value: Value::percent(2.0),
                timeframe: Some(M15)
            }
        );
    }

    #[tokio::test]
    async fn not_found() {
        let test = Test::new().await;
        let response = test.patch_json_as_test_user(
			"/v1/rules/4",
			r#"{"name":"UpdatedMoneyMaker","sequence":{"condition":{"id":"root","type":"OR","conditions":[]},"action":{"type":"NOTIFY_TELEGRAM","buttons":[]}}}"#,
		).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let error = extract_error(response).await;
        assert_eq!(error.code, StatusCode::NOT_FOUND);
        assert_eq!(error.message, "Rule not found");
    }

    #[tokio::test]
    async fn belongs_to_another_user() {
        let test = Test::new().await;
        let response = test.patch_json_as_test_user(
			"/v1/rules/1",
			r#"{"name":"UpdatedMoneyMaker","sequence":{"condition":{"id":"root","type":"OR","conditions":[]},"action":{"type":"NOTIFY_TELEGRAM","buttons":[]}}}"#,
		).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let error = extract_error(response).await;
        assert_eq!(error.code, StatusCode::NOT_FOUND);
        assert_eq!(error.message, "Rule not found");
    }

    #[tokio::test]
    async fn requires_authentication() {
        let test = Test::new().await;
        let response = test.patch_unauthenticated_json(
			"/v1/rules/1",
			r#"{"name":"UpdatedMoneyMaker","sequence":{"condition":{"id":"root","type":"OR","conditions":[]},"action":{"type":"NOTIFY_TELEGRAM","buttons":[]}}}"#,
		).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        let error = extract_error(response).await;
        assert_eq!(error.code, StatusCode::FORBIDDEN);
        assert_eq!(error.message, "User not found");
    }

    #[tokio::test]
    async fn without_body_and_content_type() {
        let test = Test::new().await;

        test.tx(|mut tx| async move {
            create_rule_for_test_user(&mut tx, "MoneyMaker").await;
            tx.commit().await.unwrap()
        })
        .await;

        let response = test.patch_no_content_as_test_user("/v1/rules/4").await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let error = extract_error(response).await;
        assert_eq!(error.code, StatusCode::BAD_REQUEST);
        assert_eq!(
            error.message,
            "Request needs to be of content type application/json"
        );
    }

    #[tokio::test]
    async fn malformed_json() {
        let test = Test::new().await;

        test.tx(|mut tx| async move {
            create_rule_for_test_user(&mut tx, "MoneyMaker").await;
            tx.commit().await.unwrap()
        })
        .await;

        let response = test.patch_json_as_test_user("/v1/rules/4", "{,}").await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let error = extract_error(response).await;
        assert_eq!(error.code, StatusCode::BAD_REQUEST);
        assert_eq!(
            error.message,
            "Failed to parse the request body as JSON: key must be a string at line 1 column 2"
        );
    }
}

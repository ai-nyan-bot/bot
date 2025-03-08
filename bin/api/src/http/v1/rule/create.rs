// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::http::error::HttpError;
use crate::http::json::JsonReq;
use crate::http::model::rule::{HttpRuleCreateRequest, HttpRuleCreateResponse};
use crate::http::state::AppState;
use axum::extract::State;
use axum::{Extension, Json};
use base::model::AuthenticatedUser;
use base::service::RuleCreateCmd;
use log::debug;

pub async fn create(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    JsonReq(req): JsonReq<HttpRuleCreateRequest>,
) -> Result<Json<HttpRuleCreateResponse>, HttpError> {
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

    Ok(Json(HttpRuleCreateResponse {
        id: result.id,
        name: result.name,
        status: result.status,
        sequence: result.sequence,
    }))
}

#[cfg(test)]
mod tests {
    use crate::http::model::rule::HttpRuleCreateResponse;
    use crate::http::testing::{extract, extract_error, Test};
    use axum::http::StatusCode;
    use base::model::{Action, Condition, TelegramActionButtonConfig, Value};

    #[test_log::test(tokio::test)]
    async fn ok() {
        let test = Test::new().await;
        let response = test
			.post_json_as_test_user(
				"/v1/rules",
				r#"{"name":"test","sequence":{"condition":{"id":"root","type":"AND","conditions":[]},
			"action":{"type":"NOTIFY_TELEGRAM",
				"buttons":[
					{"action":"NOTHING"},
					{"action":"BUY","value":{"type":"SOL","value":"1.2"}},
					{"action":"SELL","value":{"type":"PERCENT","value":3.4}},
					{"action":"NOTHING"},
					{"action":"NOTHING"},
					{"action":"NOTHING"}]}}}"#, )
			.await;
        assert_eq!(response.status(), StatusCode::OK);

        let response = extract::<HttpRuleCreateResponse>(response).await.unwrap();
        assert_eq!(response.id, 4);
        assert_eq!(response.name, "test");
        assert_eq!(
            response.sequence.condition,
            Condition::And { conditions: vec![] }
        );

        let Action::NotifyTelegram { buttons } = response.sequence.action else {
            panic!()
        };
        assert_eq!(buttons.len(), 6);
        assert_eq!(buttons.get(0).unwrap(), &TelegramActionButtonConfig::Nothing);
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

    #[test_log::test(tokio::test)]
    async fn requires_authentication() {
        let test = Test::new_empty_db().await;
        let response = test
			.post_unauthenticated_json(
				"/v1/rules",
				r#"{"name":"test","sequence":{"condition":{"id":"root","type":"AND","conditions":[]},"action":{"type":"NOTIFY_TELEGRAM","buttons":[{"action":"NOTHING"},{"action":"NOTHING"},{"action":"NOTHING"},{"action":"NOTHING"},{"action":"NOTHING"},{"action":"NOTHING"}]}}}"#,
			)
			.await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        let error = extract_error(response).await;
        assert_eq!(error.code, StatusCode::FORBIDDEN);
        assert_eq!(error.message, "User not found");
    }

    #[test_log::test(tokio::test)]
    async fn without_body_and_content_type() {
        let test = Test::new().await;
        let response = test.post_no_content_as_test_user("/v1/rules").await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let error = extract_error(response).await;
        assert_eq!(error.code, StatusCode::BAD_REQUEST);
        assert_eq!(
            error.message,
            "Request needs to be of content type application/json"
        );
    }

    #[test_log::test(tokio::test)]
    async fn malformed_json() {
        let test = Test::new().await;
        let response = test.post_json_as_test_user("/v1/rules", "{,}").await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let error = extract_error(response).await;
        assert_eq!(error.code, StatusCode::BAD_REQUEST);
        assert_eq!(
            error.message,
            "Failed to parse the request body as JSON: key must be a string at line 1 column 2"
        );
    }

    #[test_log::test(tokio::test)]
    async fn empty_json_object() {
        let test = Test::new().await;
        let response = test.post_json_as_test_user("/v1/rules", "{}").await;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let error = extract_error(response).await;
        assert_eq!(error.code, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(
			error.message,
			"Failed to deserialize the JSON body into the target type: missing field `name` at line 1 column 2"
		);
    }
}

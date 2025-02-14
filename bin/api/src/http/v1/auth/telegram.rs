// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use axum::extract::State;
use axum::Json;
use base::service::AuthenticateUserTelegramCmd;
use http::json::JsonReq;
use integration::telegram::{verify_telegram_user, TelegramLogin};
use log::{debug, error};

use crate::http;
use crate::http::error::HttpError;
use crate::http::model::auth::{Telegram, TelegramAuthRequest, TelegramAuthResponse, User, Wallet};
use crate::http::state::AppState;

pub async fn telegram(State(state): State<AppState>, JsonReq(req): JsonReq<TelegramAuthRequest>) -> Result<Json<TelegramAuthResponse>, HttpError> {
    debug!("POST /v1/auth/telegram {:?}", req);
    let bot_token = state.config.telegram.token.resolve();

    let login = telegram_login(bot_token, req)?;

    let (user, auth, wallet) = state
        .user_service()
        .authenticate_and_create_telegram_user_if_not_exists(AuthenticateUserTelegramCmd {
            telegram_id: login.user.id.into(),
        })
        .await?;

    debug!("user {} authenticated via telegram", user.id);

    Ok(Json(TelegramAuthResponse {
        // token: auth.token,
        token: "valid-token".into(),
        user: User { id: user.id },
        telegram: Telegram { id: user.telegram_id.unwrap() },
        wallet: Wallet {
            id: wallet.id,
            solana: wallet.solana_public_key,
        },
    }))
}

fn telegram_login(bot_token: String, req: TelegramAuthRequest) -> Result<TelegramLogin, HttpError> {
    TelegramLogin::from_query_string(req.query)
        .and_then(|login| {
            verify_telegram_user(bot_token.as_str(), login.clone())?;
            Ok(login)
        })
        .map_err(|err| {
            error!("failed to authenticate telegram user: {err}");
            HttpError::unprocessable("Invalid query string")
        })
}

#[cfg(test)]
mod tests {
    use crate::http::model::auth::TelegramAuthResponse;
    use crate::http::testing::{extract, extract_error, Test};
    use axum::http::StatusCode;

    #[tokio::test]
    async fn without_body_and_content_type() {
        let test = Test::new().await;
        let response = test.post_no_content("/v1/auth/telegram").await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let error = extract_error(response).await;
        assert_eq!(error.code, StatusCode::BAD_REQUEST);
        assert_eq!(error.message, "Request needs to be of content type application/json");
    }

    #[tokio::test]
    async fn malformed_json() {
        let test = Test::new().await;
        let response = test.post_json("/v1/auth/telegram", "{,}").await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let error = extract_error(response).await;
        assert_eq!(error.code, StatusCode::BAD_REQUEST);
        assert_eq!(
            error.message,
            "Failed to parse the request body as JSON: key must be a string at line 1 column 2"
        );
    }

    #[tokio::test]
    async fn empty_json_object() {
        let test = Test::new().await;
        let response = test.post_json("/v1/auth/telegram", "{}").await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let error = extract_error(response).await;
        assert_eq!(error.code, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(
            error.message,
            "Failed to deserialize the JSON body into the target type: missing field `query` at line 1 column 2"
        );
    }

    #[tokio::test]
    async fn invalid_query() {
        let test = Test::new().await;
        let response = test
            .post_json(
                "/v1/auth/telegram",
                r#"{
            "query": "invalid"
        }"#,
            )
            .await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let error = extract_error(response).await;
        assert_eq!(error.code, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(error.message, "Invalid query string");
    }

    #[tokio::test]
    async fn invalid_signature() {
        let test = Test::new().await;
        let response = test.post_json("/v1/auth/telegram", r#"{
            "query": "query_id=AAGqmHAaAwAAAKqYcBo0s6pa&user=%7B%22id%22%3A6886037674%2C%22first_name%22%3A%22Dee%22%2C%22last_name%22%3A%22Dee%22%2C%22username%22%3A%22deedee1337%22%2C%22language_code%22%3A%22en%22%2C%22allows_write_to_pm%22%3Atrue%2C%22photo_url%22%3A%22https%3A%5C%2F%5C%2Ft.me%5C%2Fi%5C%2Fuserpic%5C%2F320%5C%2FETDX5qwxULtIfWuOA5pSNI9hzRzil7XA4Tnx5NqNypDqBM6OTFA_li21aEd-wI4r.svg%22%7D&auth_date=1738054894&hash=aa7bdf3fff1121862c7a118ba15ddeca6d7299a9f9e882ad824010edceb67e6a"
        }"#).await;

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let error = extract_error(response).await;
        assert_eq!(error.code, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(error.message, "Invalid query string");
    }

    #[tokio::test]
    #[ignore]
    async fn ok() {
        let test = Test::new().await;
        let response = test.post_json("/v1/auth/telegram", r#"{
            "query": "query_id=AAGqmHAaAwAAAKqYcBo0s6pa&user=%7B%22id%22%3A6886037674%2C%22first_name%22%3A%22Dee%22%2C%22last_name%22%3A%22Dee%22%2C%22username%22%3A%22deedee1337%22%2C%22language_code%22%3A%22en%22%2C%22allows_write_to_pm%22%3Atrue%2C%22photo_url%22%3A%22https%3A%5C%2F%5C%2Ft.me%5C%2Fi%5C%2Fuserpic%5C%2F320%5C%2FETDX5qwxULtIfWuOA5pSNI9hzRzil7XA4Tnx5NqNypDqBM6OTFA_li21aEd-wI4r.svg%22%7D&auth_date=1738054894&signature=HkX8UoSMG7njc50r3GDQ95XNHNqc6E0E95GGVsYbqMDObyIUL6omfTa_gkBFhXzbg8Z6KSX07Fzzd9R4WwAPAg&hash=aa7bdf3fff1121862c7a118ba15ddeca6d7299a9f9e882ad824010edceb67e6a"
        }"#).await;

        assert_eq!(response.status(), StatusCode::OK);

        let response: TelegramAuthResponse = extract(response).await.unwrap();
        assert_eq!(response.token.0.len(), 128);
        assert_eq!(response.user.id, 1);
        assert_eq!(response.telegram.id, "6886037674");
        assert_eq!(response.wallet.id, 1);
        assert!(response.wallet.solana.0.len() > 32);
    }

    #[tokio::test]
    #[ignore]
    async fn multiple_logins() {
        let test = Test::new().await;
        let previous_response = test.post_json("/v1/auth/telegram", r#"{
            "query": "query_id=AAGqmHAaAwAAAKqYcBo0s6pa&user=%7B%22id%22%3A6886037674%2C%22first_name%22%3A%22Dee%22%2C%22last_name%22%3A%22Dee%22%2C%22username%22%3A%22deedee1337%22%2C%22language_code%22%3A%22en%22%2C%22allows_write_to_pm%22%3Atrue%2C%22photo_url%22%3A%22https%3A%5C%2F%5C%2Ft.me%5C%2Fi%5C%2Fuserpic%5C%2F320%5C%2FETDX5qwxULtIfWuOA5pSNI9hzRzil7XA4Tnx5NqNypDqBM6OTFA_li21aEd-wI4r.svg%22%7D&auth_date=1738054894&signature=HkX8UoSMG7njc50r3GDQ95XNHNqc6E0E95GGVsYbqMDObyIUL6omfTa_gkBFhXzbg8Z6KSX07Fzzd9R4WwAPAg&hash=aa7bdf3fff1121862c7a118ba15ddeca6d7299a9f9e882ad824010edceb67e6a"
        }"#).await;

        assert_eq!(previous_response.status(), StatusCode::OK);

        let response: TelegramAuthResponse = extract(previous_response).await.unwrap();
        let previous_token = response.token;
        let previous_wallet = response.wallet;

        let current_response = test.post_json("/v1/auth/telegram", r#"{
            "query": "query_id=AAGqmHAaAwAAAKqYcBo0s6pa&user=%7B%22id%22%3A6886037674%2C%22first_name%22%3A%22Dee%22%2C%22last_name%22%3A%22Dee%22%2C%22username%22%3A%22deedee1337%22%2C%22language_code%22%3A%22en%22%2C%22allows_write_to_pm%22%3Atrue%2C%22photo_url%22%3A%22https%3A%5C%2F%5C%2Ft.me%5C%2Fi%5C%2Fuserpic%5C%2F320%5C%2FETDX5qwxULtIfWuOA5pSNI9hzRzil7XA4Tnx5NqNypDqBM6OTFA_li21aEd-wI4r.svg%22%7D&auth_date=1738054894&signature=HkX8UoSMG7njc50r3GDQ95XNHNqc6E0E95GGVsYbqMDObyIUL6omfTa_gkBFhXzbg8Z6KSX07Fzzd9R4WwAPAg&hash=aa7bdf3fff1121862c7a118ba15ddeca6d7299a9f9e882ad824010edceb67e6a"
        }"#).await;

        assert_eq!(current_response.status(), StatusCode::OK);

        let current_response: TelegramAuthResponse = extract(current_response).await.unwrap();
        let current_token = current_response.token;
        let current_wallet = current_response.wallet;

        assert_ne!(previous_token, current_token);
        assert_eq!(previous_wallet.id, current_wallet.id);
        assert_eq!(previous_wallet.solana, current_wallet.solana);
    }
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::http::error::HttpError;
use crate::http::state::AppState;
use axum::extract::{Request, State};
use axum::http;
use axum::middleware::Next;
use axum::response::Response;
use base::model::AuthenticatedUser;
use base::service::AuthService;

pub async fn auth(State(state): State<AppState>, mut req: Request, next: Next) -> Result<Response, HttpError> {
    let user = authenticate_user(state.auth_service(), req.headers()).await?;
    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

async fn authenticate_user(auth_service: AuthService, headers: &http::HeaderMap) -> Result<AuthenticatedUser, HttpError> {
    if let Some(auth_header) = headers.get("Authorization") {
        let token = auth_header.to_str().ok().ok_or(HttpError::not_found("User not found"))?;

        if !token.starts_with("Bearer ") {
            return Err(HttpError::not_found("User not found"));
        }

        let token = token.replace("Bearer ", "");
        return match auth_service.get_by_token(token).await {
            Ok(user) => Ok(user),
            Err(err) => Err(err.into()),
        };
    }
    Err(HttpError::not_found("User not found"))
}

#[cfg(test)]
mod tests {
    use crate::http::error::HttpError;
    use crate::http::middleware::auth::authenticate_user;
    use axum::http::{HeaderMap, HeaderValue};
    use base::repo::AuthRepo;
    use base::service::AuthService;
    use testing::auth::create_auth;
    use testing::run_test_with_pool_on_empty_db;
    use testing::user::get_or_create_test_user;

    #[tokio::test]
    async fn test_ok() {
        run_test_with_pool_on_empty_db(|pool| async move {
            let mut headers = HeaderMap::new();
            headers.insert("Authorization", HeaderValue::from_static("Bearer token"));

            let service = AuthService::new(pool.clone(), AuthRepo::new());

            let mut tx = pool.begin().await.unwrap();
            get_or_create_test_user(&mut tx).await;
            let _ = create_auth(&mut tx, 1, "token").await;
            let _ = tx.commit().await;

            let result = authenticate_user(service, &headers).await.unwrap();
            assert_eq!(result.id, 1);
        })
        .await;
    }

    #[tokio::test]
    async fn test_token_does_not_exists() {
        run_test_with_pool_on_empty_db(|pool| async move {
            let mut headers = HeaderMap::new();
            headers.insert("Authorization", HeaderValue::from_static("Bearer token"));

            let service = AuthService::new(pool.clone(), AuthRepo::new());

            let mut tx = pool.begin().await.unwrap();
            get_or_create_test_user(&mut tx).await;
            let _ = tx.commit().await;

            let result = authenticate_user(service, &headers).await;
            assert_eq!(result.err().unwrap(), HttpError::not_found("User not found"));
        })
        .await;
    }

    #[tokio::test]
    async fn test_missing_bearer_prefix() {
        run_test_with_pool_on_empty_db(|pool| async move {
            let mut headers = HeaderMap::new();
            headers.insert("Authorization", HeaderValue::from_static("token"));

            let service = AuthService::new(pool.clone(), AuthRepo::new());

            let mut tx = pool.begin().await.unwrap();
            get_or_create_test_user(&mut tx).await;
            let _ = create_auth(&mut tx, 1, "token").await;
            let _ = tx.commit().await;

            let result = authenticate_user(service, &headers).await;
            assert_eq!(result.err().unwrap(), HttpError::not_found("User not found"));
        })
        .await;
    }

    #[tokio::test]
    async fn test_missing_header() {
        run_test_with_pool_on_empty_db(|pool| async move {
            let headers = HeaderMap::new();
            let service = AuthService::new(pool.clone(), AuthRepo::new());

            let mut tx = pool.begin().await.unwrap();
            get_or_create_test_user(&mut tx).await;
            let _ = create_auth(&mut tx, 1, "token").await;
            let _ = tx.commit().await;

            let result = authenticate_user(service, &headers).await;
            assert_eq!(result.err().unwrap(), HttpError::not_found("User not found"));
        })
        .await;
    }
}

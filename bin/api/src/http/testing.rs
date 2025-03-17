// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

#[cfg(test)]
use crate::config::{Config, TelegramConfig, WalletConfig};
#[cfg(test)]
use crate::http::error::HttpErrorResponse;
#[cfg(test)]
use crate::http::state::{AppState, AppStateInner, Service};
#[cfg(test)]
use crate::router;
#[cfg(test)]
use axum::body::{to_bytes, Body};
#[cfg(test)]
use axum::response::Response;
#[cfg(test)]
use axum::{http, Router};
#[cfg(test)]
use base::service::AuthService;
#[cfg(test)]
use base::service::{RuleService, UserService};
#[cfg(test)]
use common::crypt::SecretKey;
#[cfg(test)]
use common::repo::Tx;
#[cfg(test)]
use common::ConfigValue;
#[cfg(test)]
use serde::de::DeserializeOwned;
#[cfg(test)]
use sqlx::PgPool;
#[cfg(test)]
use std::future::Future;
#[cfg(test)]
use std::sync::Arc;
#[cfg(test)]
use testing::get_test_pool;
#[cfg(test)]
use testing::initialise_database;
#[cfg(test)]
use tower::ServiceExt;

#[cfg(test)]
pub(crate) struct Test {
    router: Router,
    pool: PgPool,
}

#[cfg(test)]
impl Test {
    pub(crate) async fn new_empty_db() -> Self {
        let pool = get_test_pool().await;
        Self::setup(pool)
    }

    pub(crate) async fn new() -> Self {
        let pool = get_test_pool().await;

        let mut tx = pool.begin().await.unwrap();
        initialise_database(&mut tx).await;
        tx.commit().await.unwrap();

        Self::setup(pool)
    }

    fn setup(pool: PgPool) -> Self {
        Self {
            router: router::setup_v1(AppState(Arc::new(AppStateInner {
                config: Config {
                    server: Default::default(),
                    postgres: Default::default(),
                    telegram: TelegramConfig {
                        token: ConfigValue::Value(
                            "7212584558:AAFyZo37lw4VPHPIdbynqKtMacHPwF0uMGE".to_string(),
                        ),
                    },
                    wallet: WalletConfig {
                        secret: ConfigValue::Value(
                            "c004a55d744672f98c9e996fe4b8c1b33cea79e9afeafca918a6a36e09777b7e"
                                .to_string(),
                        ),
                    },
                },
                service: Service {
                    auth: AuthService::testing(pool.clone()),
                    rule: RuleService::testing(pool.clone()),
                    user: UserService::new(
                        pool.clone(),
                        SecretKey::from(
                            "276b49cc192cc66ab939de3892eba683152edab76c2162b21049d8fb0d9e7e5f",
                        ),
                    ),
                },
            }))),
            pool,
        }
    }

    pub async fn tx<'a, T, TFut>(&self, func: T)
    where
        T: FnOnce(Tx<'a>) -> TFut + Send + 'static,
        TFut: Future + Send,
    {
        let tx = self.pool.begin().await.unwrap();
        func(tx).await;
    }

    pub(crate) async fn get_unauthenticated(&self, url: &str) -> Response {
        let req = axum::http::Request::builder()
            .uri(url)
            .method("GET")
            .body(Body::empty())
            .unwrap();
        self.router.clone().oneshot(req).await.unwrap()
    }

    pub(crate) async fn get_as_test_user(&self, url: &str) -> Response {
        let req = axum::http::Request::builder()
            .uri(url)
            .method("GET")
            .header("Authorization", "Bearer TestUserToken")
            .body(Body::empty())
            .unwrap();

        self.router.clone().oneshot(req).await.unwrap()
    }

    pub(crate) async fn get_as_another_user(&self, url: &str) -> Response {
        let req = axum::http::Request::builder()
            .uri(url)
            .method("GET")
            .header("Authorization", "Bearer AnotherUserToken")
            .body(Body::empty())
            .unwrap();

        self.router.clone().oneshot(req).await.unwrap()
    }

    pub(crate) async fn post_no_content_unauthenticated(&self, url: &str) -> Response {
        let req = axum::http::Request::builder()
            .uri(url)
            .method("POST")
            .body(Body::empty())
            .unwrap();
        self.router.clone().oneshot(req).await.unwrap()
    }

    pub(crate) async fn post_no_content_as_test_user(&self, url: &str) -> Response {
        let req = axum::http::Request::builder()
            .uri(url)
            .header("Authorization", "Bearer TestUserToken")
            .method("POST")
            .body(Body::empty())
            .unwrap();
        self.router.clone().oneshot(req).await.unwrap()
    }

    pub(crate) async fn post_json_as_test_user(
        &self,
        url: &str,
        json: impl Into<String>,
    ) -> Response {
        let req = axum::http::Request::builder()
            .uri(url)
            .method("POST")
            .header("content-type", "application/json")
            .header("Authorization", "Bearer TestUserToken")
            .body(Body::new(json.into()))
            .unwrap();

        self.router.clone().oneshot(req).await.unwrap()
    }

    pub(crate) async fn post_unauthenticated_json(
        &self,
        url: &str,
        json: impl Into<String>,
    ) -> Response {
        let req = axum::http::Request::builder()
            .uri(url)
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::new(json.into()))
            .unwrap();

        self.router.clone().oneshot(req).await.unwrap()
    }

    pub(crate) async fn patch_no_content_as_test_user(&self, url: &str) -> Response {
        let req = axum::http::Request::builder()
            .uri(url)
            .header("Authorization", "Bearer TestUserToken")
            .method("PATCH")
            .body(Body::empty())
            .unwrap();
        self.router.clone().oneshot(req).await.unwrap()
    }

    pub(crate) async fn patch_json_as_test_user(
        &self,
        url: &str,
        json: impl Into<String>,
    ) -> Response {
        let req = axum::http::Request::builder()
            .uri(url)
            .method("PATCH")
            .header("content-type", "application/json")
            .header("Authorization", "Bearer TestUserToken")
            .body(Body::new(json.into()))
            .unwrap();

        self.router.clone().oneshot(req).await.unwrap()
    }

    pub(crate) async fn patch_unauthenticated_json(
        &self,
        url: &str,
        json: impl Into<String>,
    ) -> Response {
        let req = axum::http::Request::builder()
            .uri(url)
            .method("PATCH")
            .header("content-type", "application/json")
            .body(Body::new(json.into()))
            .unwrap();

        self.router.clone().oneshot(req).await.unwrap()
    }
}

#[cfg(test)]
pub(crate) async fn extract<T>(
    response: http::Response<Body>,
) -> Result<T, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    let body_bytes = to_bytes(response.into_body(), usize::MAX).await?;
    let json: T = serde_json::from_slice(&body_bytes)?;
    Ok(json)
}

#[cfg(test)]
pub(crate) async fn extract_error(response: http::Response<Body>) -> HttpErrorResponse {
    extract::<HttpErrorResponse>(response).await.unwrap()
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

#[cfg(test)]
use crate::config::{Config, TelegramConfig};
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
use base::repo::RuleRepo;
#[cfg(test)]
use base::service::{RuleService, UserService};
#[cfg(test)]
use common::ConfigValue;
#[cfg(test)]
use serde::de::DeserializeOwned;
#[cfg(test)]
use std::sync::Arc;
#[cfg(test)]
use testing::get_test_pool;
#[cfg(test)]
use tower::ServiceExt;

#[cfg(test)]
pub(crate) struct Test {
    router: Router,
}

#[cfg(test)]
impl Test {
    pub(crate) async fn new() -> Self {
        let pool = get_test_pool().await;
        Self {
            router: router::setup_v1(AppState(Arc::new(AppStateInner {
                config: Config {
                    server: Default::default(),
                    postgres: Default::default(),
                    telegram: TelegramConfig {
                        token: ConfigValue::Value("7212584558:AAFyZo37lw4VPHPIdbynqKtMacHPwF0uMGE".to_string()),
                    },
                },
                service: Service {
                    rule: RuleService::new(pool.clone(), RuleRepo::new()),
                    user: UserService::new(pool.clone()),
                },
            }))),
        }
    }

    pub(crate) async fn get(&self, url: &str) -> Response {
        let req = axum::http::Request::builder().uri(url).method("GET").body(Body::empty()).unwrap();

        self.router.clone().oneshot(req).await.unwrap()
    }

    pub(crate) async fn post_no_content(&self, url: &str) -> Response {
        let req = axum::http::Request::builder().uri(url).method("POST").body(Body::empty()).unwrap();
        self.router.clone().oneshot(req).await.unwrap()
    }

    pub(crate) async fn post_json(&self, url: &str, json: impl Into<String>) -> Response {
        let req = axum::http::Request::builder()
            .uri(url)
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::new(json.into()))
            .unwrap();

        self.router.clone().oneshot(req).await.unwrap()
    }
}

#[cfg(test)]
pub(crate) async fn extract<T>(response: http::Response<Body>) -> Result<T, Box<dyn std::error::Error>>
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

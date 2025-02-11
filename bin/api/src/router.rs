// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::str::FromStr;

use axum::http::HeaderName;
use axum::middleware::from_fn_with_state;
use axum::routing::{any, get, post};
use axum::Router;
use tower_http::cors::{AllowHeaders, Any, CorsLayer};

use crate::http::state::AppState;
use crate::http::v1;
use crate::{http, ws};

pub fn setup_v1(app_state: AppState) -> Router {
    Router::new()
        .route("/health", any(http::v1::health))
        .route("/ws", any(ws::handler).layer(from_fn_with_state(app_state.clone(), ws::middleware::auth)))
        .nest(
            "/v1/auth",
            Router::new()
                .route("/metamask", post(v1::auth::metamask))
                .route("/telegram", post(v1::auth::telegram)),
        )
        .nest(
            "/v1",
            Router::new()
                .route("/rules", get(v1::rule::list))
                .route("/rules/{id}", get(v1::rule::get))
                .route("/rules", post(v1::rule::create))
                .route("/wallets", post(v1::wallet::create))
                .route("/wallets/{id}", get(v1::wallet::get))
                .route("/wallets/{id}/quote", post(v1::wallet::quote))
                .route("/wallets/{id}/swap", post(v1::wallet::swap))
                .route("/wallets/{id}/send", post(v1::wallet::send))
                .layer(from_fn_with_state(app_state.clone(), http::middleware::auth)),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(AllowHeaders::list(vec![
                    HeaderName::from_str("Authorization").unwrap(),
                    HeaderName::from_str("Content-Type").unwrap(),
                ]))
                .allow_methods(Any),
        )
        .with_state(app_state)
}

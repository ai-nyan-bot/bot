// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use axum::extract::State;
use axum::{Form, Json};
use log::debug;
use serde::{Deserialize, Serialize};

use crate::http::error::HttpError;
use crate::http::json::JsonReq;
use crate::http::state::AppState;

#[derive(Deserialize, Debug)]
pub struct TokenRequest {
    address: String,
    signature: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenResponse {
    token: String,
    user: UserResponse,
    wallet: WalletResponse,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WalletResponse {
    pub(crate) solana: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub(crate) id: String,
}

pub async fn metamask(State(_state): State<AppState>, JsonReq(req): JsonReq<TokenRequest>) -> Result<Json<TokenResponse>, HttpError> {
    debug!("POST /v1/auth/metamask {:?}", req);

    // if user not exists yet
    // create user

    Ok(Json(TokenResponse {
        token: "token".to_string(),
        user: UserResponse { id: "user_id".to_string() },
        wallet: WalletResponse {
            solana: "Bp65Vdx5o5THggj1ZHYsVwaKPhp999mRmAeKyFG9FVnT".to_string(),
        },
    }))
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::http::error::HttpError;
use crate::http::json::JsonReq;
use crate::http::model::auth::{MetamaskAuthRequest, MetamaskAuthResponse, Telegram, TelegramAuthResponse, User, Wallet};
use crate::http::state::AppState;
use axum::extract::State;
use axum::Json;
use base::service::AuthenticateUserTelegramCmd;
use log::debug;
use serde::{Deserialize, Serialize};

pub async fn metamask(State(state): State<AppState>, JsonReq(req): JsonReq<MetamaskAuthRequest>) -> Result<Json<MetamaskAuthResponse>, HttpError> {
    debug!("POST /v1/auth/metamask {:?}", req);

    // FIXME
    let (user, auth, wallet) = state
        .user_service()
        .authenticate_and_create_telegram_user_if_not_exists(AuthenticateUserTelegramCmd { telegram_id: "0".into() })
        .await?;

    debug!("user {} authenticated via metamask", user.id);

    Ok(Json(MetamaskAuthResponse {
        token: auth.token,
        user: User { id: user.id },
        wallet: Wallet {
            id: wallet.id,
            solana: wallet.solana_public_key,
        },
    }))
}

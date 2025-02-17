// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::PublicKey;
use base::model::{AuthToken, UserId, WalletId};
use common::model::TelegramId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct MetamaskAuthRequest {
    // address: String,
    // signature: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetamaskAuthResponse {
    pub token: AuthToken,
    pub user: User,
    pub wallet: Wallet,
}

#[derive(Deserialize, Debug)]
pub struct TelegramAuthRequest {
    pub query: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: UserId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Telegram {
    pub id: TelegramId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Wallet {
    pub id: WalletId,
    pub solana: PublicKey,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TelegramAuthResponse {
    pub token: AuthToken,
    pub user: User,
    pub telegram: Telegram,
    pub wallet: Wallet,
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::UserId;
use crate::model::{PrivateKey, PublicKey};
use common::crypt::Nonce;
use common::model::{CreatedAt, UpdatedAt};
use serde::{Deserialize, Serialize};

#[derive(Eq, Hash, Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct WalletId(pub i64);

impl PartialEq<i64> for WalletId {
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

impl From<i64> for WalletId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug)]
pub struct Wallet {
    pub id: WalletId,
    pub user_id: UserId,
    pub solana_public_key: PublicKey,
    pub created_at: CreatedAt,
    pub updated_at: UpdatedAt,
}

#[derive(Clone, Debug)]
pub struct WalletUnsafe {
    pub id: WalletId,
    pub user_id: UserId,
    pub solana_public_key: PublicKey,
    pub solana_private_key: PrivateKey,
    pub created_at: CreatedAt,
    pub updated_at: UpdatedAt,
    pub nonce: Nonce,
}

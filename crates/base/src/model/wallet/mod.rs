// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

use crate::model::UserId;
use common::model::{CreatedAt, UpdatedAt};
use common::model::{PrivateKey, PublicKey};

#[derive(Eq, Hash, Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct WalletId(pub i32);

impl PartialEq<i32> for WalletId {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl From<i32> for WalletId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug)]
pub struct Wallet {
    pub id: WalletId,
    pub user_id: UserId,
    pub solana_public_key: PublicKey,
    pub solana_private_key: PrivateKey,
    pub created_at: CreatedAt,
    pub updated_at: UpdatedAt,
}

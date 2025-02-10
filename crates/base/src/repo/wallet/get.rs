// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use sqlx::{query, Row};

use crate::repo::wallet::WalletRepo;
use crate::model::{UserId, Wallet, WalletId};
use common::model::{PrivateKey, PublicKey};
use common::model::{CreatedAt, UpdatedAt};
use common::repo::{RepoResult, Tx};

impl WalletRepo {
    pub async fn get_by_id<'a>(&self, tx: &mut Tx<'a>, id: impl Into<WalletId> + Send) -> RepoResult<Wallet> {
        Ok(query("select * from nyanbot.wallet where id = $1;")
            .bind(id.into())
            .fetch_one(&mut **tx)
            .await
            .map(|r| Wallet {
                id: r.get::<WalletId, _>("id"),
                user_id: r.get::<UserId, _>("user_id"),
                solana_public_key: r.get::<PublicKey, _>("solana_public_key"),
                solana_private_key: r.get::<PrivateKey, _>("solana_private_key"),
                created_at: r.get::<CreatedAt, _>("created_at"),
                updated_at: r.get::<UpdatedAt, _>("updated_at"),
            })?)
    }
}

impl WalletRepo {
    pub async fn get_by_user_id<'a>(&self, tx: &mut Tx<'a>, id: impl Into<UserId> + Send) -> RepoResult<Wallet> {
        Ok(query("select * from nyanbot.wallet where user_id = $1;")
            .bind(id.into())
            .fetch_one(&mut **tx)
            .await
            .map(|r| Wallet {
                id: r.get::<WalletId, _>("id"),
                user_id: r.get::<UserId, _>("user_id"),
                solana_public_key: r.get::<PublicKey, _>("solana_public_key"),
                solana_private_key: r.get::<PrivateKey, _>("solana_private_key"),
                created_at: r.get::<CreatedAt, _>("created_at"),
                updated_at: r.get::<UpdatedAt, _>("updated_at"),
            })?)
    }
}

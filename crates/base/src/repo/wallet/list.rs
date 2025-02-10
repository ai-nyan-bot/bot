// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use sqlx::Row;

use crate::model::{PrivateKey, PublicKey};
use crate::model::{UserId, Wallet, WalletId};
use crate::repo::wallet::{WalletQueryAll, WalletRepo};
use common::model::{CreatedAt, UpdatedAt};
use common::repo::{RepoResult, Tx};

impl WalletRepo {
    pub async fn list<'a>(&self, tx: &mut Tx<'a>, query: WalletQueryAll) -> RepoResult<Box<[Wallet]>> {
        Ok(sqlx::query("select * from nyanbot.wallet order by id desc limit $1;")
            .bind(query.limit)
            .fetch_all(&mut **tx)
            .await?
            .iter()
            .map(|r| Wallet {
                id: r.get::<WalletId, _>("id"),
                user_id: r.get::<UserId, _>("user_id"),
                solana_public_key: r.get::<PublicKey, _>("solana_public_key"),
                solana_private_key: r.get::<PrivateKey, _>("solana_private_key"),
                created_at: r.get::<CreatedAt, _>("created_at"),
                updated_at: r.get::<UpdatedAt, _>("updated_at"),
            })
            .collect::<Vec<_>>()
            .into_boxed_slice())
    }
}

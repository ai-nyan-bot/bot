// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{PrivateKey, PublicKey};
use crate::model::{UserId, Wallet, WalletId};
use crate::repo::wallet::WalletRepo;
use common::crypt::{decrypt_string, Nonce};
use common::model::{CreatedAt, UpdatedAt};
use common::repo::{RepoResult, Tx};
use sqlx::{query, Row};

impl WalletRepo {
    pub async fn get_by_id<'a>(
        &self,
        tx: &mut Tx<'a>,
        id: impl Into<WalletId> + Send,
    ) -> RepoResult<Wallet> {
        Ok(query("select * from solana.wallet where id = $1;")
            .bind(id.into())
            .fetch_one(&mut **tx)
            .await
            .map(|r| Wallet {
                id: r.get::<WalletId, _>("id"),
                user_id: r.get::<UserId, _>("user_id"),
                public_key: r.get::<PublicKey, _>("public_key"),
                created_at: r.get::<CreatedAt, _>("created_at"),
                updated_at: r.get::<UpdatedAt, _>("updated_at"),
            })?)
    }

    pub async fn get_by_user_id<'a>(
        &self,
        tx: &mut Tx<'a>,
        id: impl Into<UserId> + Send,
    ) -> RepoResult<Wallet> {
        Ok(query("select * from solana.wallet where user_id = $1;")
            .bind(id.into())
            .fetch_one(&mut **tx)
            .await
            .map(|r| Wallet {
                id: r.get::<WalletId, _>("id"),
                user_id: r.get::<UserId, _>("user_id"),
                public_key: r.get::<PublicKey, _>("public_key"),
                created_at: r.get::<CreatedAt, _>("created_at"),
                updated_at: r.get::<UpdatedAt, _>("updated_at"),
            })?)
    }

    pub async fn get_private_key<'a>(
        &self,
        tx: &mut Tx<'a>,
        id: impl Into<WalletId> + Send,
    ) -> RepoResult<PrivateKey> {
        let row = query("select nonce, private_key from solana.wallet where id = $1;")
            .bind(id.into())
            .fetch_one(&mut **tx)
            .await?;

        let nonce = Nonce::from(row.get::<String, _>("nonce"));
        let encrypted_key = row.get::<String, _>("private_key");

        Ok(PrivateKey(
            decrypt_string(&self.secret, &nonce, encrypted_key).unwrap(),
        ))
    }
}

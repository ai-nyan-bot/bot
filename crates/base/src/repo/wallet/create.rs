// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{PrivateKey, PublicKey};
use crate::model::{UserId, Wallet, WalletId};
use crate::repo::wallet::WalletRepo;
use common::crypt::{encrypt_string, Nonce};
use common::repo::error::RepoError;
use common::repo::{RepoResult, Tx};
use log::error;
use sqlx::{query, Row};

pub struct WalletCreateCmd {
    pub user_id: UserId,
    pub solana_public_key: PublicKey,
    pub solana_private_key: PrivateKey,
    pub nonce: Nonce,
}

impl WalletRepo {
    pub async fn create<'a>(&self, tx: &mut Tx<'a>, cmd: WalletCreateCmd) -> RepoResult<Wallet> {
        if let Some(encrypted) = encrypt_string(
            &self.secret,
            &cmd.nonce,
            cmd.solana_private_key.0.as_bytes(),
        ) {
            let user_id = query("insert into nyanbot.wallet (user_id, solana_public_key, solana_private_key, nonce) values ($1, $2, $3,$4) returning id")
            .bind(cmd.user_id)
            .bind(cmd.solana_public_key)
            .bind::<String>(encrypted)
            .bind::<String>(cmd.nonce.into())
            .fetch_one(&mut **tx)
            .await
            .map(|r| r.get::<WalletId, _>("id"))?;

            self.get_by_id(tx, user_id).await
        } else {
            error!("unable to encrypt private key");
            Err(RepoError::Serde)
        }
    }
}

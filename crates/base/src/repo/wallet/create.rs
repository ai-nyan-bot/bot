// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use sqlx::{query, Row};

use crate::repo::wallet::WalletRepo;
use crate::model::{UserId, Wallet, WalletId};
use common::model::{PrivateKey, PublicKey};
use common::repo::{RepoResult, Tx};

pub struct WalletCreateCmd {
    pub user_id: UserId,
    pub solana_public_key: PublicKey,
    pub solana_private_key: PrivateKey,
}

impl WalletRepo {
    pub async fn create<'a>(&self, tx: &mut Tx<'a>, cmd: WalletCreateCmd) -> RepoResult<Wallet> {
        let user_id = query("insert into nyanbot.wallet (user_id, solana_public_key, solana_private_key) values ($1, $2, $3) returning id")
            .bind(cmd.user_id)
            .bind(cmd.solana_public_key)
            .bind(cmd.solana_private_key)
            .fetch_one(&mut **tx)
            .await
            .map(|r| r.get::<WalletId, _>("id"))?;

        self.get_by_id(tx, user_id).await
    }
}

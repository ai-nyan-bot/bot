// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use sqlx::Row;

use common::model::Count;

use crate::repo::wallet::WalletRepo;
use common::repo::{RepoResult, Tx};

impl WalletRepo {
    pub async fn count<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<Count> {
        Ok(sqlx::query("select count(*) from solana.wallet;")
            .fetch_one(&mut **tx)
            .await?
            .get::<Count, _>("count"))
    }
}

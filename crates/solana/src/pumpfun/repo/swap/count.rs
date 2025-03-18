// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::pumpfun::repo::ReadSwapRepo;
use common::model::Count;
use common::repo::{RepoResult, Tx};
use sqlx::Row;

impl ReadSwapRepo {
    pub async fn count_swap<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<Count> {
        Ok(sqlx::query("select count(*) from pumpfun.swap;")
            .fetch_one(&mut **tx)
            .await?
            .get::<Count, _>("count"))
    }

    pub async fn count_micro_swap<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<Count> {
        Ok(sqlx::query("select count(*) from pumpfun.micro_swap;")
            .fetch_one(&mut **tx)
            .await?
            .get::<Count, _>("count"))
    }
}

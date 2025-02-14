// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::repo::pumpfun::{SummaryQueryAll, SummaryRepo};
use common::model::Count;
use common::repo::{RepoResult, Tx};
use sqlx::Row;

impl SummaryRepo {
    pub async fn count<'a>(&self, tx: &mut Tx<'a>, query: SummaryQueryAll) -> RepoResult<Count> {
        Ok(sqlx::query("select count(*) from pumpfun.summary_1m;")
            .fetch_one(&mut **tx)
            .await?
            .get::<Count, _>("count"))
    }
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::pumpfun::repo::CurveRepo;
use common::model::Count;
use common::repo::{RepoResult, Tx};
use sqlx::Row;

impl CurveRepo {
    pub async fn count_all<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<Count> {
        Ok(sqlx::query("select count(*) from pumpfun.curve;")
            .fetch_one(&mut **tx)
            .await?
            .get::<Count, _>("count"))
    }
}

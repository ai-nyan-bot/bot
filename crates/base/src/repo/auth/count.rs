// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::model::Count;
use sqlx::Row;

use crate::repo::auth::AuthQueryAll;
use crate::repo::AuthRepo;
use common::repo::{RepoResult, Tx};

impl AuthRepo {
    pub async fn count<'a>(&self, tx: &mut Tx<'a>, query: AuthQueryAll) -> RepoResult<Count> {
        Ok(sqlx::query("select count(*) from nyanbot.auth")
            .fetch_one(&mut **tx)
            .await?
            .get::<Count, _>("count"))
    }
}

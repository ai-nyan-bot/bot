// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use sqlx::Row;

use common::model::Count;

use crate::repo::user::UserQueryAll;
use crate::repo::UserRepo;
use common::repo::{RepoResult, Tx};

impl UserRepo {
    pub async fn count<'a>(&self, tx: &mut Tx<'a>, query: UserQueryAll) -> RepoResult<Count> {
        Ok(sqlx::query("select count(*) from nyanbot.user;")
            .fetch_one(&mut **tx)
            .await?
            .get::<Count, _>("count"))
    }
}

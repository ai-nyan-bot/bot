// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use sqlx::{query, Row};

use crate::model::{Auth, AuthId, AuthToken, UserId};
use crate::repo::AuthRepo;
use common::repo::{RepoResult, Tx};

pub struct AuthCreateCmd {
    pub user_id: UserId,
    pub token: AuthToken,
}

impl AuthRepo {
    pub async fn create<'a>(&self, tx: &mut Tx<'a>, cmd: AuthCreateCmd) -> RepoResult<Auth> {
        let auth_id = query("insert into nyanbot.auth (user_id, token) values ($1, $2) returning id")
            .bind(cmd.user_id)
            .bind(cmd.token)
            .fetch_one(&mut **tx)
            .await
            .map(|r| r.get::<AuthId, _>("id"))?;

        self.get_by_id(tx, auth_id).await
    }
}

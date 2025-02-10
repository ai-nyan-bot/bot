// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use sqlx::Row;

use common::model::{CreatedAt, UpdatedAt};

use crate::model::{Auth, AuthId, AuthToken, AuthenticatedUser, UserId};
use crate::repo::auth::AuthQueryAll;
use crate::repo::AuthRepo;
use common::repo::{RepoResult, Tx};

impl AuthRepo {
    pub async fn list<'a>(&self, tx: &mut Tx<'a>, query: AuthQueryAll) -> RepoResult<Box<[Auth]>> {
        Ok(sqlx::query(
            r#"
select
    a.id as id,
    a.user_id as user_id,
    a.token as token,
    a.created_at as created_at,
    a.updated_at as updated_at
from nyanbot.auth a
left join nyanbot."user" u on a.user_id = u.id
order by a.id desc
limit $1
            "#,
        )
        .bind(query.limit)
        .fetch_all(&mut **tx)
        .await?
        .iter()
        .map(|r| Auth {
            id: r.get::<AuthId, _>("id"),
            user: AuthenticatedUser {
                id: r.get::<UserId, _>("user_id"),
            },
            token: r.get::<AuthToken, _>("token"),
            created_at: r.get::<CreatedAt, _>("created_at"),
            updated_at: r.get::<UpdatedAt, _>("updated_at"),
        })
        .collect::<Vec<_>>()
        .into_boxed_slice())
    }
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use sqlx::{query, Row};

use common::model::{CreatedAt, UpdatedAt};

use crate::model::{Auth, AuthId, AuthToken, AuthenticatedUser, UserId};
use crate::repo::AuthRepo;
use common::repo::{RepoResult, Tx};

impl AuthRepo {
    pub async fn get_by_id<'a>(&self, tx: &mut Tx<'a>, auth_id: impl Into<AuthId> + Send) -> RepoResult<Auth> {
        Ok(query(
            r#"
select
    a.id as id,
    a.user_id as user_id,
    a.token as token,
    a.created_at as created_at,
    a.updated_at as updated_at
from nyanbot.auth a
left join nyanbot."user" u on a.user_id = u.id
where a.id = $1;
            "#,
        )
        .bind(auth_id.into())
        .fetch_one(&mut **tx)
        .await
        .map(|r| Auth {
            id: r.get::<AuthId, _>("id"),
            user: AuthenticatedUser {
                id: r.get::<UserId, _>("user_id"),
            },
            token: r.get::<AuthToken, _>("token"),
            created_at: r.get::<CreatedAt, _>("created_at"),
            updated_at: r.get::<UpdatedAt, _>("updated_at"),
        })?)
    }
}

impl AuthRepo {
    pub async fn get_by_token<'a>(&self, tx: &mut Tx<'a>, token: impl Into<AuthToken> + Send) -> RepoResult<Auth> {
        Ok(query(
            r#"
select
    a.id as id,
    a.user_id as user_id,
    a.token as token,
    a.created_at as created_at,
    a.updated_at as updated_at
from nyanbot.auth a
left join nyanbot."user" u on a.user_id = u.id
where a.token = $1;
            "#,
        )
        .bind(token.into())
        .fetch_one(&mut **tx)
        .await
        .map(|r| Auth {
            id: r.get::<AuthId, _>("id"),
            user: AuthenticatedUser {
                id: r.get::<UserId, _>("user_id"),
            },
            token: r.get::<AuthToken, _>("token"),
            created_at: r.get::<CreatedAt, _>("created_at"),
            updated_at: r.get::<UpdatedAt, _>("updated_at"),
        })?)
    }
}

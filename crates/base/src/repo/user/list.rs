// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use sqlx::Row;

use crate::model::{User, UserId};
use common::model::{CreatedAt, TelegramId, UpdatedAt};

use crate::repo::user::{UserQueryAll, UserRepo};
use common::repo::{RepoResult, Tx};

impl UserRepo {
    pub async fn list<'a>(&self, tx: &mut Tx<'a>, query: UserQueryAll) -> RepoResult<Box<[User]>> {
        Ok(sqlx::query("select * from nyanbot.user order by id desc limit $1;")
            .bind(query.limit)
            .fetch_all(&mut **tx)
            .await?
            .iter()
            .map(|r| User {
                id: r.get::<UserId, _>("id"),
                telegram_id: r.try_get::<TelegramId, _>("telegram_id").ok(),
                created_at: r.get::<CreatedAt, _>("created_at"),
                updated_at: r.get::<UpdatedAt, _>("updated_at"),
            })
            .collect::<Vec<_>>()
            .into_boxed_slice())
    }
}

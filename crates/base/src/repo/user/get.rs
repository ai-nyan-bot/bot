// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use sqlx::{query, Row};

use common::model::{CreatedAt, TelegramId, UpdatedAt};

use crate::model::{User, UserId};
use crate::repo::user::UserRepo;
use common::repo::{RepoResult, Tx};

impl UserRepo {
    pub async fn get_by_id<'a>(&self, tx: &mut Tx<'a>, id: impl Into<UserId> + Send) -> RepoResult<User> {
        Ok(query("select * from nyanbot.user where id = $1;")
            .bind(id.into())
            .fetch_one(&mut **tx)
            .await
            .map(|r| User {
                id: r.get::<UserId, _>("id"),
                telegram_id: r.try_get::<TelegramId, _>("telegram_id").ok(),
                created_at: r.get::<CreatedAt, _>("created_at"),
                updated_at: r.get::<UpdatedAt, _>("updated_at"),
            })?)
    }
}

impl UserRepo {
    pub async fn get_by_telegram_id<'a>(&self, tx: &mut Tx<'a>, id: impl Into<TelegramId> + Send) -> RepoResult<User> {
        Ok(query("select * from nyanbot.user where telegram_id = $1;")
            .bind(id.into())
            .fetch_one(&mut **tx)
            .await
            .map(|r| User {
                id: r.get::<UserId, _>("id"),
                telegram_id: r.try_get::<TelegramId, _>("telegram_id").ok(),
                created_at: r.get::<CreatedAt, _>("created_at"),
                updated_at: r.get::<UpdatedAt, _>("updated_at"),
            })?)
    }
}

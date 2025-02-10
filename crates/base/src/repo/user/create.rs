// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use sqlx::{query, Row};

use common::model::TelegramId;

use crate::model::{User, UserId};
use crate::repo::user::UserRepo;
use common::repo::{RepoResult, Tx};

pub struct UserCreateTelegramCmd {
    pub telegram_id: TelegramId,
}

impl UserRepo {
    pub async fn create_telegram<'a>(&self, tx: &mut Tx<'a>, cmd: UserCreateTelegramCmd) -> RepoResult<User> {
        let user_id = query("insert into nyanbot.user (telegram_id) values ($1) returning id")
            .bind(cmd.telegram_id)
            .fetch_one(&mut **tx)
            .await
            .map(|r| r.get::<UserId, _>("id"))?;

        self.get_by_id(tx, user_id).await
    }
}

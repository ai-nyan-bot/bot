// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

use common::model::{CreatedAt, TelegramId, UpdatedAt};

#[derive(Eq, Hash, Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserId(pub i32);

impl PartialEq<i32> for UserId {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl From<i32> for UserId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: UserId,
    pub telegram_id: Option<TelegramId>,
    pub created_at: CreatedAt,
    pub updated_at: UpdatedAt,
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

use common::model::{CreatedAt, TelegramId, UpdatedAt};

#[derive(Eq, Hash, Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserId(pub i64);

impl PartialEq<i64> for UserId {
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

impl From<i64> for UserId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl Display for UserId{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: UserId,
    pub telegram_id: Option<TelegramId>,
    pub created_at: CreatedAt,
    pub updated_at: UpdatedAt,
}

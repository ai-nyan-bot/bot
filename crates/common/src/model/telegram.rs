// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct TelegramId(pub i64);

impl From<i64> for TelegramId {
    fn from(value: i64) -> Self {
        TelegramId(value)
    }
}

impl PartialEq<i64> for TelegramId {
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

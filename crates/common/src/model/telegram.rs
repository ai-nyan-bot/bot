// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct TelegramId(pub String);

impl From<i64> for TelegramId {
    fn from(value: i64) -> Self {
        TelegramId(value.to_string())
    }
}

impl From<u64> for TelegramId {
    fn from(value: u64) -> Self {
        TelegramId(value.to_string())
    }
}

impl From<&str> for TelegramId {
    fn from(value: &str) -> Self {
        TelegramId(value.to_string())
    }
}

impl PartialEq<&str> for TelegramId {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

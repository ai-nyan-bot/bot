// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Trades(pub i32);

impl From<i32> for Trades {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<u64> for Trades {
    fn from(value: u64) -> Self {
        Self(value as i32)
    }
}

impl PartialEq<i32> for Trades {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

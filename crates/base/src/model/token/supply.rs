// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Supply(pub i64);

impl Supply {
    pub fn infinite() -> Self {
        Self(-1)
    }
}

impl From<i32> for Supply {
    fn from(value: i32) -> Self {
        Self(value as i64)
    }
}

impl From<i64> for Supply {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<u64> for Supply {
    fn from(value: u64) -> Self {
        Self(value as i64)
    }
}

impl PartialEq<i64> for Supply {
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

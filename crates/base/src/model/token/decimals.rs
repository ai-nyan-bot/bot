// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Decimals(pub i16);

impl From<i16> for Decimals {
    fn from(value: i16) -> Self {
        Self(value)
    }
}

impl From<u8> for Decimals {
    fn from(value: u8) -> Self {
        Self(value as i16)
    }
}

impl From<i32> for Decimals {
    fn from(value: i32) -> Self {
        Self(value as i16)
    }
}

impl From<i64> for Decimals {
    fn from(value: i64) -> Self {
        Self(value as i16)
    }
}

impl PartialEq<i16> for Decimals {
    fn eq(&self, other: &i16) -> bool {
        self.0 == *other
    }
}

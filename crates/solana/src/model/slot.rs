// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use fmt::Display;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Hash, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Slot(pub i64);

impl Slot {
    pub fn increment(&mut self) -> Self {
        self.0 += 1;
        Self(self.0)
    }
}

impl Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i32> for Slot {
    fn from(value: i32) -> Self {
        Self(value as i64)
    }
}

impl From<i64> for Slot {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<u64> for Slot {
    fn from(value: u64) -> Self {
        Self(value as i64)
    }
}

impl PartialEq<i32> for Slot {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other as i64
    }
}

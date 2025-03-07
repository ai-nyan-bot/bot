// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use fmt::Display;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;

#[derive(
    Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Deserialize, Serialize, sqlx::Type,
)]
#[sqlx(transparent)]
pub struct SwapId(pub i64);

impl Display for SwapId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i32> for SwapId {
    fn from(value: i32) -> Self {
        Self(value as i64)
    }
}

impl From<i64> for SwapId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<u64> for SwapId {
    fn from(value: u64) -> Self {
        Self(value as i64)
    }
}

impl PartialEq<i32> for SwapId {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other as i64
    }
}

impl PartialEq<i64> for SwapId {
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<i64> for SwapId {
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

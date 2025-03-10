// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::model::BlockId;
use fmt::Display;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;

#[derive(
    Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Deserialize, Serialize, sqlx::Type,
)]
#[sqlx(transparent)]
pub struct Slot(pub i64);

impl Slot {
    pub fn increment(&mut self) -> Self {
        self.0 += 1;
        Self(self.0)
    }
}

impl Into<BlockId> for Slot {
    fn into(self) -> BlockId {
        BlockId(self.0)
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

impl PartialEq<i64> for Slot {
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<i64> for Slot {
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

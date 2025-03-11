// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Timestamp;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct BlockId(pub i64);

impl From<i32> for BlockId {
    fn from(value: i32) -> Self {
        Self(value as i64)
    }
}

impl PartialEq<i64> for BlockId {
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

impl Display for BlockId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Ord, PartialOrd, Eq, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct BlockTime(pub Timestamp);

impl Deref for BlockTime {
    type Target = Timestamp;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BlockTime {
    pub fn now() -> Self {
        BlockTime(Timestamp::now())
    }
}

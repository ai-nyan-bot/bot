// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

#[derive(Eq, Hash, Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct StrategyId(pub i32);

impl AsRef<StrategyId> for StrategyId {
    fn as_ref(&self) -> &StrategyId {
        &self
    }
}

impl PartialEq<i32> for StrategyId {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl From<i32> for StrategyId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

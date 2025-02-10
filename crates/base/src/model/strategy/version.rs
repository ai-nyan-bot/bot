// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

#[derive(Eq, Hash, Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct StrategyVersion(pub i16);

impl AsRef<StrategyVersion> for StrategyVersion {
    fn as_ref(&self) -> &StrategyVersion {
        &self
    }
}

impl PartialEq<i16> for StrategyVersion {
    fn eq(&self, other: &i16) -> bool {
        self.0 == *other
    }
}

impl From<i16> for StrategyVersion {
    fn from(value: i16) -> Self {
        Self(value)
    }
}

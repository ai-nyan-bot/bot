// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Eq, Hash, Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct RuleId(pub i32);

impl AsRef<RuleId> for RuleId {
    fn as_ref(&self) -> &RuleId {
        self
    }
}

impl PartialEq<i32> for RuleId {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl From<i32> for RuleId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<String> for RuleId {
    fn from(value: String) -> Self {
        Self(value.parse::<i32>().unwrap_or(0))
    }
}

impl Display for RuleId{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
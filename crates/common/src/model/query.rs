// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent, no_pg_array)]
pub struct Limit(pub i64);

impl Default for Limit {
    fn default() -> Self {
        Self(50)
    }
}

impl Limit {
    pub fn new(val: i64) -> Self {
        Self(val.clamp(1, 1000))
    }

    pub fn min() -> Self {
        Self(1)
    }

    pub fn max() -> Self {
        Self(1000)
    }

    pub fn unlimited() -> Self {
        Self(i64::MAX)
    }
}

impl From<i64> for Limit {
    fn from(value: i64) -> Self {
        Limit::new(value)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent, no_pg_array)]
pub struct Count(pub i64);

impl PartialEq<i64> for Count {
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

impl Display for Count {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

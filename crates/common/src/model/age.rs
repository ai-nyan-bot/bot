// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent, no_pg_array)]
pub struct AgeRelativeToLatestInSeconds(pub i64);

impl From<i32> for AgeRelativeToLatestInSeconds {
    fn from(value: i32) -> Self {
        Self(value as i64)
    }
}

impl PartialEq<i64> for AgeRelativeToLatestInSeconds {
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

impl Display for AgeRelativeToLatestInSeconds {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

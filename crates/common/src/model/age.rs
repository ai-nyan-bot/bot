// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct AgeInSeconds(pub i64);

impl From<i64> for AgeInSeconds {
    fn from(value: i64) -> Self {
        Self(value as i64)
    }
}

impl PartialEq<i64> for AgeInSeconds {
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<i64> for AgeInSeconds {
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl Display for AgeInSeconds {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent, no_pg_array)]
pub struct AgeRelativeToLatestInSeconds(pub i64);

impl From<i64> for AgeRelativeToLatestInSeconds {
    fn from(value: i64) -> Self {
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

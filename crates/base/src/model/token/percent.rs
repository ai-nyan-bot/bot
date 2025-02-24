// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Percent(pub f32);

impl From<f32> for Percent {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<i64> for Percent {
    fn from(value: i64) -> Self {
        Self(value as f32)
    }
}

impl PartialEq<f32> for Percent {
    fn eq(&self, other: &f32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<i64> for Percent {
    fn eq(&self, other: &i64) -> bool {
        self.0 == Percent::from(*other).0
    }
}

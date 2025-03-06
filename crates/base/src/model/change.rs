// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Change(pub f64);

impl From<f64> for Change {
    fn from(value: f64) -> Self {
        Self(value as f64)
    }
}

impl PartialEq<f64> for Change {
    fn eq(&self, other: &f64) -> bool {
        self.0 == *other as f64
    }
}

impl From<i32> for Change {
    fn from(value: i32) -> Self {
        Self(value as f64)
    }
}

impl PartialEq<i32> for Change {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other as f64
    }
}

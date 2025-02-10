// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Trades(pub i64);

impl From<i64> for Trades {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<u64> for Trades {
    fn from(value: u64) -> Self {
        Self(value as i64)
    }
}

impl PartialEq<i64> for Trades {
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct TradesChange(pub f64);

impl From<f64> for TradesChange {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl PartialEq<f64> for TradesChange {
    fn eq(&self, other: &f64) -> bool {
        self.0 == *other
    }
}

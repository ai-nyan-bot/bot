// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct PriceAvgQuote(pub f64);

impl From<f64> for PriceAvgQuote {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl PartialEq<f64> for PriceAvgQuote {
    fn eq(&self, other: &f64) -> bool {
        self.0 == *other
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct PriceChangeChange(pub f64);

impl From<f64> for PriceChangeChange {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl PartialEq<f64> for PriceChangeChange {
    fn eq(&self, other: &f64) -> bool {
        self.0 == *other
    }
}

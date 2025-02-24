// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Decimals;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Amount(pub i64);

impl From<i32> for Amount {
    fn from(value: i32) -> Self {
        Self(value as i64)
    }
}

impl From<i64> for Amount {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<u64> for Amount {
    fn from(value: u64) -> Self {
        Self(value as i64)
    }
}

impl PartialEq<i64> for Amount {
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct AmountChangeChange(pub f64);

impl From<f64> for AmountChangeChange {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl PartialEq<f64> for AmountChangeChange {
    fn eq(&self, other: &f64) -> bool {
        self.0 == *other
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct DecimalAmount(pub f64);

impl DecimalAmount {
    pub fn new(amount: impl Into<Amount>, decimals: impl Into<Decimals>) -> Self {
        let amount = amount.into();
        let decimals = decimals.into();
        Self(amount.0 as f64 / 10f64.powf(decimals.0 as f64))
    }
}

impl From<i64> for DecimalAmount {
    fn from(value: i64) -> Self {
        Self(value as f64)
    }
}

impl From<u64> for DecimalAmount {
    fn from(value: u64) -> Self {
        Self(value as f64)
    }
}

impl From<f64> for DecimalAmount {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl PartialEq<f64> for DecimalAmount {
    fn eq(&self, other: &f64) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<f64> for DecimalAmount {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct DecimalAmountChange(pub f64);

impl From<f64> for DecimalAmountChange {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl PartialEq<f64> for DecimalAmountChange {
    fn eq(&self, other: &f64) -> bool {
        self.0 == *other
    }
}

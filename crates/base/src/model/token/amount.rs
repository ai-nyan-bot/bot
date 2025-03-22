// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Decimals;
use bigdecimal::ToPrimitive;
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;
use sqlx::Type;
use std::cmp::Ordering;
use std::ops::{Div, Sub};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Amount(pub BigDecimal);

impl From<i32> for Amount {
    fn from(value: i32) -> Self {
        Self(BigDecimal::from(value))
    }
}

impl From<i64> for Amount {
    fn from(value: i64) -> Self {
        Self(BigDecimal::from(value))
    }
}

impl From<u64> for Amount {
    fn from(value: u64) -> Self {
        Self(BigDecimal::from(value))
    }
}

impl From<BigDecimal> for Amount {
    fn from(value: BigDecimal) -> Self {
        Self(value)
    }
}

impl PartialEq<i64> for Amount {
    fn eq(&self, other: &i64) -> bool {
        self.0.to_i64().unwrap() == *other
    }
}

impl PartialOrd<i64> for Amount {
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        self.0.to_i64().unwrap().partial_cmp(other)
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Type)]
#[sqlx(transparent)]
pub struct DecimalAmount(pub BigDecimal);

impl DecimalAmount {
    pub fn new(amount: impl Into<Amount>, decimals: impl Into<Decimals>) -> Self {
        let amount = BigDecimal::from(amount.into().0);
        let divisor = 10i64.pow(decimals.into().0 as u32);
        Self(amount.div(BigDecimal::from(divisor)))
    }
}

impl From<i64> for DecimalAmount {
    fn from(value: i64) -> Self {
        Self(BigDecimal::from(value))
    }
}

impl From<u64> for DecimalAmount {
    fn from(value: u64) -> Self {
        Self(BigDecimal::from(value))
    }
}

impl PartialEq<i32> for DecimalAmount {
    fn eq(&self, other: &i32) -> bool {
        Self(BigDecimal::from(*other)).0.eq(&self.0)
    }
}

impl PartialOrd<i32> for DecimalAmount {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.0.partial_cmp(&BigDecimal::from(*other))
    }
}

impl PartialEq<BigDecimal> for DecimalAmount {
    fn eq(&self, other: &BigDecimal) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<BigDecimal> for DecimalAmount {
    fn partial_cmp(&self, other: &BigDecimal) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl PartialEq<&str> for DecimalAmount {
    fn eq(&self, other: &&str) -> bool {
        self.eq(&BigDecimal::from_str(other).unwrap())
    }
}

impl Sub for DecimalAmount {
    type Output = DecimalAmount;

    fn sub(self, rhs: Self) -> Self::Output {
        DecimalAmount(self.0.sub(rhs.0))
    }
}

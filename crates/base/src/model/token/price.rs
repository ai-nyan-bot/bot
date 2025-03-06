// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Type)]
#[sqlx(transparent)]
pub struct PriceQuote(pub BigDecimal);

impl From<i64> for PriceQuote {
    fn from(value: i64) -> Self {
        Self(BigDecimal::from(value))
    }
}

impl From<u64> for PriceQuote {
    fn from(value: u64) -> Self {
        Self(BigDecimal::from(value))
    }
}

impl PartialEq<i32> for PriceQuote {
    fn eq(&self, other: &i32) -> bool {
        Self(BigDecimal::from(*other)).0.eq(&self.0)
    }
}

impl PartialOrd<i32> for PriceQuote {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.0.partial_cmp(&BigDecimal::from(*other))
    }
}

impl PartialEq<BigDecimal> for PriceQuote {
    fn eq(&self, other: &BigDecimal) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<BigDecimal> for PriceQuote {
    fn partial_cmp(&self, other: &BigDecimal) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl PartialEq<&str> for PriceQuote {
    fn eq(&self, other: &&str) -> bool {
        self.eq(&BigDecimal::
        from_str(other).unwrap())
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Type)]
#[sqlx(transparent)]
pub struct PriceAvgQuote(pub BigDecimal);

impl From<i64> for PriceAvgQuote {
    fn from(value: i64) -> Self {
        Self(BigDecimal::from(value))
    }
}

impl From<u64> for PriceAvgQuote {
    fn from(value: u64) -> Self {
        Self(BigDecimal::from(value))
    }
}

impl PartialEq<i32> for PriceAvgQuote {
    fn eq(&self, other: &i32) -> bool {
        Self(BigDecimal::from(*other)).0.eq(&self.0)
    }
}

impl PartialOrd<i32> for PriceAvgQuote {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.0.partial_cmp(&BigDecimal::from(*other))
    }
}

impl PartialEq<BigDecimal> for PriceAvgQuote {
    fn eq(&self, other: &BigDecimal) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<BigDecimal> for PriceAvgQuote {
    fn partial_cmp(&self, other: &BigDecimal) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl PartialEq<&str> for PriceAvgQuote {
    fn eq(&self, other: &&str) -> bool {
        self.eq(&BigDecimal::from_str(other).unwrap())
    }
}

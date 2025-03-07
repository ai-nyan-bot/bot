// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Type)]
#[sqlx(transparent)]
pub struct Volume(pub BigDecimal);

impl From<i32> for Volume {
    fn from(value: i32) -> Self {
        Self(BigDecimal::from(value))
    }
}

impl From<i64> for Volume {
    fn from(value: i64) -> Self {
        Self(BigDecimal::from(value))
    }
}

impl From<u64> for Volume {
    fn from(value: u64) -> Self {
        Self(BigDecimal::from(value))
    }
}

impl PartialEq<i32> for Volume {
    fn eq(&self, other: &i32) -> bool {
        Self(BigDecimal::from(*other)).0.eq(&self.0)
    }
}

impl PartialOrd<i32> for Volume {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.0.partial_cmp(&BigDecimal::from(*other))
    }
}

impl PartialEq<BigDecimal> for Volume {
    fn eq(&self, other: &BigDecimal) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<BigDecimal> for Volume {
    fn partial_cmp(&self, other: &BigDecimal) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl PartialEq<&str> for Volume {
    fn eq(&self, other: &&str) -> bool {
        self.eq(&BigDecimal::from_str(other).unwrap())
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Type)]
#[sqlx(transparent)]
pub struct VolumeUsd(pub BigDecimal);

impl From<i32> for VolumeUsd {
    fn from(value: i32) -> Self {
        Self(BigDecimal::from(value))
    }
}

impl From<i64> for VolumeUsd {
    fn from(value: i64) -> Self {
        Self(BigDecimal::from(value))
    }
}

impl From<u64> for VolumeUsd {
    fn from(value: u64) -> Self {
        Self(BigDecimal::from(value))
    }
}

impl From<&str> for VolumeUsd {
    fn from(value: &str) -> Self {
        Self(BigDecimal::from_str(value).unwrap())
    }
}

impl PartialEq<i32> for VolumeUsd {
    fn eq(&self, other: &i32) -> bool {
        Self(BigDecimal::from(*other)).0.eq(&self.0)
    }
}

impl PartialOrd<i32> for VolumeUsd {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.0.partial_cmp(&BigDecimal::from(*other))
    }
}

impl PartialEq<BigDecimal> for VolumeUsd {
    fn eq(&self, other: &BigDecimal) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<BigDecimal> for VolumeUsd {
    fn partial_cmp(&self, other: &BigDecimal) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl PartialEq<&str> for VolumeUsd {
    fn eq(&self, other: &&str) -> bool {
        self.eq(&BigDecimal::from_str(other).unwrap())
    }
}

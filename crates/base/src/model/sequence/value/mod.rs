// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{PriceAvgQuote, Trades};
use common::model::TimeUnit;
pub(crate) use compare::*;
use ::serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

mod compare;
mod serde;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ValueType {
    Boolean,
    Count,
    Duration,
    Quote,
    Percent,
    Sol,
    String,
    Usd,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Value {
    Boolean { value: bool },
    Count { value: i64 },
    Duration { value: i64, unit: TimeUnit },
    Percent { value: f64 },
    Quote { value: f64 },
    Sol { value: f64 },
    String { value: String },
    Usd { value: f64 },
}

impl Value {

    pub fn boolean(value: bool) -> Self { Self::Boolean { value }}
    pub fn count(value: i64) -> Self { Self::Count { value }}
    pub fn duration(value: i64, unit: TimeUnit) -> Self { Self::Duration { value, unit } }
    pub fn percent(value: f64) -> Self { Self::Percent { value }}
    pub fn quote(value: f64) -> Self { Self::Quote { value }}
    pub fn sol(value: f64) -> Self { Self::Sol { value }}
    pub fn string(value: impl Into<String>) -> Self { Self::String { value: value.into() }}
    pub fn usd(value: f64) -> Self { Self::Usd { value }}

}

impl From<PriceAvgQuote> for Value {
    fn from(value: PriceAvgQuote) -> Self {
        Self::Quote { value: value.0 }
    }
}

impl From<Trades> for Value {
    fn from(value: Trades) -> Self {
        Self::Count {
            value: value.0 as i64,
        }
    }
}

impl Value {
    pub fn value_type(&self) -> ValueType {
        match self {
            Value::Boolean { .. } => ValueType::Boolean,
            Value::Count { .. } => ValueType::Count,
            Value::Duration { .. } => ValueType::Duration,
            Value::Percent { .. } => ValueType::Percent,
            Value::Quote { .. } => ValueType::Quote,
            Value::Sol { .. } => ValueType::Sol,
            Value::String { .. } => ValueType::String,
            Value::Usd { .. } => ValueType::Usd,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Boolean { value } => Display::fmt(value, f),
            Value::Count { value } => Display::fmt(value, f),
            Value::Duration { value, unit } => f.write_fmt(format_args!("{}{}", value, unit)),
            Value::Percent { value } => f.write_fmt(format_args!("{value} %")),
            Value::Quote { value } => f.write_fmt(format_args!("{value} QUOTE")),
            Value::Sol { value } => f.write_fmt(format_args!("{value} SOL")),
            Value::String { value } => Display::fmt(value, f),
            Value::Usd { value } => f.write_fmt(format_args!("{value} USD")),
        }
    }
}

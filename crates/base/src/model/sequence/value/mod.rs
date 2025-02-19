// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{PriceAvgQuote, Trades};
pub(crate) use compare::*;
pub(crate) use serde::*;
use ::serde::{Deserialize, Serialize};

mod compare;
mod serde;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ValueType {
    Boolean,
    Count,
    Quote,
    Percent,
    String,
    Usd,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Value {
    #[serde(serialize_with = "serialize_boolean", deserialize_with = "deserialize_boolean")]
    Boolean(bool),
    #[serde(serialize_with = "serialize_count", deserialize_with = "deserialize_count")]
    Count(i64),
    #[serde(serialize_with = "serialize_percent", deserialize_with = "deserialize_percent")]
    Percent(f64),
    #[serde(serialize_with = "serialize_quote", deserialize_with = "deserialize_quote")]
    Quote(f64),
    #[serde(serialize_with = "serialize_string", deserialize_with = "deserialize_string")]
    String(String),
    #[serde(serialize_with = "serialize_usd", deserialize_with = "deserialize_usd")]
    Usd(f64),
}

impl From<PriceAvgQuote> for Value {
    fn from(value: PriceAvgQuote) -> Self {
        Self::Quote(value.0)
    }
}

impl From<Trades> for Value {
    fn from(value: Trades) -> Self {
        Self::Count(value.0 as i64)
    }
}

impl Value {
    pub fn value_type(&self) -> ValueType {
        match self {
            Value::Boolean(_) => ValueType::Boolean,
            Value::Count(_) => ValueType::Count,
            Value::Percent(_) => ValueType::Percent,
            Value::Quote(_) => ValueType::Quote,
            Value::String(_) => ValueType::String,
            Value::Usd(_) => ValueType::Usd,
        }
    }
}

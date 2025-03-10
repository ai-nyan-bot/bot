// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Field, Operator, Value};
use common::model::Timeframe;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

mod applicable;
mod test;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct ComposeType(pub String);

impl From<String> for ComposeType {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ComposeType {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl PartialEq<&str> for ComposeType {
    fn eq(&self, other: &&str) -> bool {
        self.0.as_str() == *other
    }
}

impl ComposeType {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl Display for ComposeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Condition {
    Compose {
        composition: ComposeType,
        condition: Box<Condition>,
    },
    Compare {
        field: Field,
        operator: Operator,
        value: Option<Value>,
        timeframe: Option<Timeframe>,
    },
    And {
        conditions: Vec<Condition>,
    },
    Or {
        conditions: Vec<Condition>,
    },
    AndNot {
        conditions: Vec<Condition>,
    },
}

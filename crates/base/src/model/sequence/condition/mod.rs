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
pub struct ComposeId(pub String);

impl From<String> for ComposeId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ComposeId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl PartialEq<&str> for ComposeId {
    fn eq(&self, other: &&str) -> bool {
        self.0.as_str() == *other
    }
}

impl ComposeId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl Display for ComposeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Condition {
    Compose {
        id: ComposeId,
        condition: Box<Condition>,
    },
    Compare {
        field: Field,
        operator: Operator,
        value: Value,
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

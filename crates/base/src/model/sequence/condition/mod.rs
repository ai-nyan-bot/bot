// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Field, Operator, Value};
use common::model::Timeframe;
use serde::{Deserialize, Serialize};

mod test;
mod applicable;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Condition {
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

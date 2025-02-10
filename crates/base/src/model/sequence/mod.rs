// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

pub use action::*;
pub use condition::*;
pub use fact::*;
pub use operator::*;
use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
pub use value::*;

mod action;
mod condition;
mod fact;
mod operator;
mod value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sequence {
    pub condition: Condition,
    pub action: Action,
}

impl From<Sequence> for JsonValue {
    fn from(value: Sequence) -> Self {
        let json_value = serde_json::to_value(value).expect("Failed to serialize Sequence");
        JsonValue::from(json_value)
    }
}

impl From<JsonValue> for Sequence {
    fn from(value: JsonValue) -> Self {
        serde_json::from_value(value).expect("Failed to deserialize Sequence")
    }
}

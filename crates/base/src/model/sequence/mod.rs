// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

pub use action::*;
pub use condition::*;
pub use error::*;
pub use fact::*;
pub use facts::*;
pub use field::*;
pub use operator::*;
use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
pub use value::*;

mod action;
mod condition;
mod error;
mod fact;
mod facts;
mod field;
mod operator;
mod value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sequence {
    pub condition: Condition,
    pub action: Action,
}

impl From<Sequence> for JsonValue {
    fn from(value: Sequence) -> Self {
        
        serde_json::to_value(value).expect("Failed to serialize Sequence")
    }
}

impl From<JsonValue> for Sequence {
    fn from(value: JsonValue) -> Self {
        serde_json::from_value(value).expect("Failed to deserialize Sequence")
    }
}

impl Sequence {
    pub fn applicable(&self) -> bool {
        self.condition.applicable()
    }
}

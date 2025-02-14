// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Condition, Fact, ValueType};

#[derive(Debug, Clone, PartialEq)]
pub enum FactError {
    ValueTypeMismatch { expected: ValueType, got: ValueType },
    TimeframeRequired(Fact),
    TimeframeNotAllowed(Fact),
    UnableToDeriveFact(Condition),
}

impl std::fmt::Display for FactError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FactError::ValueTypeMismatch { expected, got: found } => {
                write!(f, "Value type mismatch: expected {:?}, found {:?}", expected, found)
            }
            FactError::TimeframeRequired(fact) => {
                write!(f, "{:?} requires a timeframe, but none was provided", fact)
            }
            FactError::TimeframeNotAllowed(fact) => {
                write!(f, "{:?} does not support a timeframe, but one was provided", fact)
            }
            FactError::UnableToDeriveFact(condition) => {
                write!(f, "unable to derive fact from {:?}", condition)
            }
        }
    }
}

impl std::error::Error for FactError {}

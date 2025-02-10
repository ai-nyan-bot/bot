// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Operator, Price};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ValueKind {
    Boolean,
    Duration,
    Number,
    String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Boolean(bool),
    Duration(Duration),
    Number(f64),
    String(String),
}

impl From<Price> for Value {
    fn from(value: Price) -> Self {
        Self::Number(value.0)
    }
}

impl Value {
    pub fn kind(&self) -> ValueKind {
        match self {
            Value::Boolean(_) => ValueKind::Boolean,
            Value::Duration(_) => ValueKind::Duration,
            Value::Number(_) => ValueKind::Number,
            Value::String(_) => ValueKind::String,
        }
    }
}

pub(crate) fn compare(fact: &Value, operator: &Operator, rule: &Value) -> bool {
    match (fact, rule) {
        (Value::Boolean(fact), Value::Boolean(rule)) => match operator {
            Operator::Equal => fact == rule,
            Operator::NotEqual => fact != rule,
            _ => false,
        },
        (Value::Duration(fact), Value::Duration(rule)) => match operator {
            Operator::Equal => fact == rule,
            Operator::GreaterThan => fact > rule,
            Operator::GreaterThanEqual => fact >= rule,
            Operator::LessThan => fact < rule,
            Operator::LessThanEqual => fact <= rule,
            Operator::NotEqual => fact != rule,
            _ => false,
        },
        (Value::Number(fact), Value::Number(rule)) => match operator {
            Operator::Equal => fact == rule,
            Operator::GreaterThan => fact > rule,
            Operator::GreaterThanEqual => fact >= rule,
            Operator::LessThan => fact < rule,
            Operator::LessThanEqual => fact <= rule,
            Operator::NotEqual => fact != rule,
            _ => false,
        },
        (Value::String(fact), Value::String(rule)) => match operator {
            Operator::Equal => fact == rule,
            Operator::NotEqual => fact != rule,
            _ => false,
        },
        _ => false,
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::time::Duration;
	use Operator::{Equal, GreaterThan, GreaterThanEqual, LessThan, LessThanEqual, NotEqual};
	use Value::{Boolean, Number};

	#[test]
    fn test_boolean_comparisons() {
        assert!(compare(&Boolean(true), &Equal, &Boolean(true)));
        assert!(!compare(&Boolean(true), &Equal, &Boolean(false)));
        assert!(compare(&Boolean(true), &NotEqual, &Boolean(false)));
        assert!(!compare(&Boolean(true), &NotEqual, &Boolean(true)));
    }

    #[test]
    fn test_duration_comparisons() {
        assert!(compare(
            &Value::Duration(Duration::from_secs(10)),
            &Equal,
            &Value::Duration(Duration::from_secs(10))
        ));
        assert!(compare(
            &Value::Duration(Duration::from_secs(10)),
            &GreaterThan,
            &Value::Duration(Duration::from_secs(5))
        ));
        assert!(compare(
            &Value::Duration(Duration::from_secs(10)),
            &GreaterThanEqual,
            &Value::Duration(Duration::from_secs(10))
        ));
        assert!(compare(
            &Value::Duration(Duration::from_secs(5)),
            &LessThan,
            &Value::Duration(Duration::from_secs(10))
        ));
        assert!(compare(
            &Value::Duration(Duration::from_secs(5)),
            &LessThanEqual,
            &Value::Duration(Duration::from_secs(5))
        ));
        assert!(compare(
            &Value::Duration(Duration::from_secs(5)),
            &NotEqual,
            &Value::Duration(Duration::from_secs(10))
        ));
    }

    #[test]
    fn test_number_comparisons() {
        assert!(compare(&Number(5.0), &Equal, &Number(5.0)));
        assert!(compare(&Number(5.0), &GreaterThan, &Number(3.0)));
        assert!(compare(&Number(5.0), &GreaterThanEqual, &Number(5.0)));
        assert!(compare(&Number(3.0), &LessThan, &Number(5.0)));
        assert!(compare(&Number(3.0), &LessThanEqual, &Number(3.0)));
        assert!(compare(&Number(3.0), &NotEqual, &Number(5.0)));
    }

    #[test]
    fn test_string_comparisons() {
        assert!(compare(&Value::String("hello".to_string()), &Equal, &Value::String("hello".to_string())));
        assert!(!compare(&Value::String("hello".to_string()), &Equal, &Value::String("world".to_string())));
        assert!(compare(&Value::String("hello".to_string()), &NotEqual, &Value::String("world".to_string())));
    }
}

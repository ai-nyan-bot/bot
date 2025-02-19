// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::ops::Neg;
use crate::model::{Operator, Value};

pub(crate) fn compare(fact: &Value, operator: &Operator, rule: &Value) -> bool {
    match (fact, rule) {
        (Value::Boolean(fact), Value::Boolean(rule)) => match operator {
            Operator::Equal => fact == rule,
            Operator::NotEqual => fact != rule,
            _ => false,
        },
        (Value::Count(fact), Value::Count(rule)) => match operator {
            Operator::Equal => fact == rule,
            Operator::NotEqual => fact != rule,

            Operator::IncreasedByMoreThan => *fact > 0 && fact > rule,
            Operator::IncreasedByMoreThanEqual => *fact > 0 && fact >= rule,
            Operator::IncreasedByLessThan => *fact > 0 && fact < rule,
            Operator::IncreasedByLessThanEqual => *fact > 0 && fact <= rule,

            Operator::DecreasedByMoreThan => *fact < 0 && *fact > rule.neg(),
            Operator::DecreasedByMoreThanEqual => *fact < 0 && *fact >= rule.neg(),
            Operator::DecreasedByLessThan => *fact < 0 && *fact < rule.neg(),
            Operator::DecreasedByLessThanEqual => *fact < 0 && *fact <= rule.neg(),

            Operator::MoreThan => fact > rule,
            Operator::MoreThanEqual => fact >= rule,
            Operator::LessThan => fact < rule,
            Operator::LessThanEqual => fact <= rule,
            _ => false,
        },
        (Value::Percent(fact), Value::Percent(rule)) => match operator {
            Operator::Equal => fact == rule,
            Operator::NotEqual => fact != rule,

            Operator::IncreasedByMoreThan => *fact > 0.0 && fact > rule,
            Operator::IncreasedByMoreThanEqual => *fact > 0.0 && fact >= rule,
            Operator::IncreasedByLessThan => *fact > 0.0 && fact < rule,
            Operator::IncreasedByLessThanEqual => *fact > 0.0 && fact <= rule,

            Operator::DecreasedByMoreThan => *fact < 0.0 && *fact > rule.neg(),
            Operator::DecreasedByMoreThanEqual => *fact < 0.0 && *fact >= rule.neg(),
            Operator::DecreasedByLessThan => *fact < 0.0 && *fact < rule.neg(),
            Operator::DecreasedByLessThanEqual => *fact < 0.0 && *fact <= rule.neg(),

            Operator::MoreThan => fact > rule,
            Operator::MoreThanEqual => fact >= rule,
            Operator::LessThan => fact < rule,
            Operator::LessThanEqual => fact <= rule,
            _ => false,
        },
        (Value::Quote(fact), Value::Quote(rule)) => match operator {
            Operator::Equal => fact == rule,
            Operator::NotEqual => fact != rule,

            Operator::IncreasedByMoreThan => *fact > 0.0 && fact > rule,
            Operator::IncreasedByMoreThanEqual => *fact > 0.0 && fact >= rule,
            Operator::IncreasedByLessThan => *fact > 0.0 && fact < rule,
            Operator::IncreasedByLessThanEqual => *fact > 0.0 && fact <= rule,

            Operator::DecreasedByMoreThan => *fact < 0.0 && *fact > rule.neg(),
            Operator::DecreasedByMoreThanEqual => *fact < 0.0 && *fact >= rule.neg(),
            Operator::DecreasedByLessThan => *fact < 0.0 && *fact < rule.neg(),
            Operator::DecreasedByLessThanEqual => *fact < 0.0 && *fact <= rule.neg(),

            Operator::MoreThan => fact > rule,
            Operator::MoreThanEqual => fact >= rule,
            Operator::LessThan => fact < rule,
            Operator::LessThanEqual => fact <= rule,
            _ => false,
        },
        (Value::String(fact), Value::String(rule)) => match operator {
            Operator::Equal => fact == rule,
            Operator::NotEqual => fact != rule,
            _ => false,
        },
        (Value::Usd(fact), Value::Usd(rule)) => match operator {
            Operator::Equal => fact == rule,
            Operator::NotEqual => fact != rule,

            Operator::IncreasedByMoreThan => *fact > 0.0 && fact > rule,
            Operator::IncreasedByMoreThanEqual => *fact > 0.0 && fact >= rule,
            Operator::IncreasedByLessThan => *fact > 0.0 && fact < rule,
            Operator::IncreasedByLessThanEqual => *fact > 0.0 && fact <= rule,

            Operator::DecreasedByMoreThan => *fact < 0.0 && *fact > rule.neg(),
            Operator::DecreasedByMoreThanEqual => *fact < 0.0 && *fact >= rule.neg(),
            Operator::DecreasedByLessThan => *fact < 0.0 && *fact < rule.neg(),
            Operator::DecreasedByLessThanEqual => *fact < 0.0 && *fact <= rule.neg(),

            Operator::MoreThan => fact > rule,
            Operator::MoreThanEqual => fact >= rule,
            Operator::LessThan => fact < rule,
            Operator::LessThanEqual => fact <= rule,
            _ => false,
        },
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Operator::{
        DecreasedByLessThan, DecreasedByLessThanEqual, DecreasedByMoreThan, DecreasedByMoreThanEqual, IncreasedByLessThan, IncreasedByLessThanEqual,
        IncreasedByMoreThan, IncreasedByMoreThanEqual,
    };
    use crate::model::Value::{Count, Quote, Usd};
    use Operator::{Equal, LessThan, LessThanEqual, MoreThan, MoreThanEqual, NotEqual};
    use Value::{Boolean, Percent};

    #[test]
    fn test_boolean_comparisons() {
        assert!(compare(&Boolean(true), &Equal, &Boolean(true)));
        assert!(!compare(&Boolean(true), &Equal, &Boolean(false)));
        assert!(compare(&Boolean(true), &NotEqual, &Boolean(false)));
        assert!(!compare(&Boolean(true), &NotEqual, &Boolean(true)));
    }

    #[test]
    fn test_count_comparisons() {
        assert!(compare(&Count(5), &Equal, &Count(5)));

        assert!(compare(&Count(5), &IncreasedByMoreThan, &Count(3)));
        assert!(compare(&Count(5), &IncreasedByMoreThanEqual, &Count(5)));
        assert!(compare(&Count(3), &IncreasedByLessThan, &Count(5)));
        assert!(compare(&Count(3), &IncreasedByLessThanEqual, &Count(3)));

        assert!(compare(&Count(-3), &DecreasedByMoreThan, &Count(5)));
        assert!(compare(&Count(-5), &DecreasedByMoreThanEqual, &Count(5)));
        assert!(compare(&Count(-5), &DecreasedByLessThan, &Count(3)));
        assert!(compare(&Count(-3), &DecreasedByLessThanEqual, &Count(3)));

        assert!(compare(&Count(5), &MoreThan, &Count(3)));
        assert!(compare(&Count(5), &MoreThanEqual, &Count(5)));
        assert!(compare(&Count(3), &LessThan, &Count(5)));
        assert!(compare(&Count(3), &LessThanEqual, &Count(3)));
        assert!(compare(&Count(3), &NotEqual, &Count(5)));
    }

    #[test]
    fn test_percent_comparisons() {
        assert!(compare(&Percent(5.0), &Equal, &Percent(5.0)));
        assert!(compare(&Percent(3.0), &NotEqual, &Percent(5.0)));

        assert!(compare(&Percent(5.0), &IncreasedByMoreThan, &Percent(3.0)));
        assert!(compare(&Percent(5.0), &IncreasedByMoreThanEqual, &Percent(5.0)));
        assert!(compare(&Percent(3.0), &IncreasedByLessThan, &Percent(5.0)));
        assert!(compare(&Percent(3.0), &IncreasedByLessThanEqual, &Percent(3.0)));

        assert!(compare(&Percent(-3.0), &DecreasedByMoreThan, &Percent(5.0)));
        assert!(compare(&Percent(-5.0), &DecreasedByMoreThanEqual, &Percent(5.0)));
        assert!(compare(&Percent(-5.0), &DecreasedByLessThan, &Percent(3.0)));
        assert!(compare(&Percent(-3.0), &DecreasedByLessThanEqual, &Percent(3.0)));

        assert!(compare(&Percent(5.0), &MoreThan, &Percent(3.0)));
        assert!(compare(&Percent(5.0), &MoreThanEqual, &Percent(5.0)));
        assert!(compare(&Percent(3.0), &LessThan, &Percent(5.0)));
        assert!(compare(&Percent(3.0), &LessThanEqual, &Percent(3.0)));
    }

    #[test]
    fn test_quote_comparisons() {
        assert!(compare(&Quote(5.0), &Equal, &Quote(5.0)));
        assert!(compare(&Quote(3.0), &NotEqual, &Quote(5.0)));

        assert!(compare(&Quote(5.0), &IncreasedByMoreThan, &Quote(3.0)));
        assert!(compare(&Quote(5.0), &IncreasedByMoreThanEqual, &Quote(5.0)));
        assert!(compare(&Quote(3.0), &IncreasedByLessThan, &Quote(5.0)));
        assert!(compare(&Quote(3.0), &IncreasedByLessThanEqual, &Quote(3.0)));

        assert!(compare(&Quote(-3.0), &DecreasedByMoreThan, &Quote(5.0)));
        assert!(compare(&Quote(-5.0), &DecreasedByMoreThanEqual, &Quote(5.0)));
        assert!(compare(&Quote(-5.0), &DecreasedByLessThan, &Quote(3.0)));
        assert!(compare(&Quote(-3.0), &DecreasedByLessThanEqual, &Quote(3.0)));

        assert!(compare(&Quote(5.0), &MoreThan, &Quote(3.0)));
        assert!(compare(&Quote(5.0), &MoreThanEqual, &Quote(5.0)));
        assert!(compare(&Quote(3.0), &LessThan, &Quote(5.0)));
        assert!(compare(&Quote(3.0), &LessThanEqual, &Quote(3.0)));
    }

    #[test]
    fn test_string_comparisons() {
        assert!(compare(&Value::String("hello".to_string()), &Equal, &Value::String("hello".to_string())));
        assert!(!compare(&Value::String("hello".to_string()), &Equal, &Value::String("world".to_string())));
        assert!(compare(&Value::String("hello".to_string()), &NotEqual, &Value::String("world".to_string())));
    }

    #[test]
    fn test_usd_comparisons() {
        assert!(compare(&Usd(5.0), &Equal, &Usd(5.0)));
        assert!(compare(&Usd(3.0), &NotEqual, &Usd(5.0)));

        assert!(compare(&Usd(5.0), &IncreasedByMoreThan, &Usd(3.0)));
        assert!(compare(&Usd(5.0), &IncreasedByMoreThanEqual, &Usd(5.0)));
        assert!(compare(&Usd(3.0), &IncreasedByLessThan, &Usd(5.0)));
        assert!(compare(&Usd(3.0), &IncreasedByLessThanEqual, &Usd(3.0)));

        assert!(compare(&Usd(-3.0), &DecreasedByMoreThan, &Usd(5.0)));
        assert!(compare(&Usd(-5.0), &DecreasedByMoreThanEqual, &Usd(5.0)));
        assert!(compare(&Usd(-5.0), &DecreasedByLessThan, &Usd(3.0)));
        assert!(compare(&Usd(-3.0), &DecreasedByLessThanEqual, &Usd(3.0)));

        assert!(compare(&Usd(5.0), &MoreThan, &Usd(3.0)));
        assert!(compare(&Usd(5.0), &MoreThanEqual, &Usd(5.0)));
        assert!(compare(&Usd(3.0), &LessThan, &Usd(5.0)));
        assert!(compare(&Usd(3.0), &LessThanEqual, &Usd(3.0)));
    }
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Operator, Value};
use std::ops::Neg;

pub(crate) fn compare(fact: &Value, operator: &Operator, rule: &Value) -> bool {
    match (fact, rule) {
        (Value::Boolean { value: fact }, Value::Boolean { value: rule }) => match operator {
            Operator::Equal => fact == rule,
            Operator::NotEqual => fact != rule,
            _ => false,
        },
        (Value::Count { value: fact }, Value::Count { value: rule }) => match operator {
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
        },
        (Value::Percent { value: fact }, Value::Percent { value: rule }) => match operator {
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
        },
        (Value::Quote { value: fact }, Value::Quote { value: rule }) => match operator {
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
        },
        (Value::String { value: fact }, Value::String { value: rule }) => match operator {
            Operator::Equal => fact == rule,
            Operator::NotEqual => fact != rule,
            _ => false,
        },
        (Value::Usd { value: fact }, Value::Usd { value: rule }) => match operator {
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
        },
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Operator::{
        DecreasedByLessThan, DecreasedByLessThanEqual, DecreasedByMoreThan,
        DecreasedByMoreThanEqual, IncreasedByLessThan, IncreasedByLessThanEqual,
        IncreasedByMoreThan, IncreasedByMoreThanEqual,
    };
    use crate::model::Value::{Count, Quote, Usd};
    use Operator::{Equal, LessThan, LessThanEqual, MoreThan, MoreThanEqual, NotEqual};
    use Value::{Boolean, Percent};

    #[test]
    fn test_boolean_comparisons() {
        assert!(compare(&Value::boolean(true), &Equal, &Value::boolean(true)));
        assert!(!compare(&Value::boolean(true), &Equal, &Value::boolean(false)));
        assert!(compare(&Value::boolean(true), &NotEqual, &Value::boolean(false)));
        assert!(!compare(&Value::boolean(true), &NotEqual, &Value::boolean(true)));
    }

    #[test]
    fn test_count_comparisons() {
        assert!(compare(&Value::count(5), &Equal, &Value::count(5)));

        assert!(compare(&Value::count(5), &IncreasedByMoreThan, &Value::count(3)));
        assert!(compare(&Value::count(5), &IncreasedByMoreThanEqual, &Value::count(5)));
        assert!(compare(&Value::count(3), &IncreasedByLessThan, &Value::count(5)));
        assert!(compare(&Value::count(3), &IncreasedByLessThanEqual, &Value::count(3)));

        assert!(compare(&Value::count(-3), &DecreasedByMoreThan, &Value::count(5)));
        assert!(compare(&Value::count(-5), &DecreasedByMoreThanEqual, &Value::count(5)));
        assert!(compare(&Value::count(-5), &DecreasedByLessThan, &Value::count(3)));
        assert!(compare(&Value::count(-3), &DecreasedByLessThanEqual, &Value::count(3)));

        assert!(compare(&Value::count(5), &MoreThan, &Value::count(3)));
        assert!(compare(&Value::count(5), &MoreThanEqual, &Value::count(5)));
        assert!(compare(&Value::count(3), &LessThan, &Value::count(5)));
        assert!(compare(&Value::count(3), &LessThanEqual, &Value::count(3)));
        assert!(compare(&Value::count(3), &NotEqual, &Value::count(5)));
    }

    #[test]
    fn test_percent_comparisons() {
        assert!(compare(&Value::percent(5.0), &Equal, &Value::percent(5.0)));
        assert!(compare(&Value::percent(3.0), &NotEqual, &Value::percent(5.0)));

        assert!(compare(&Value::percent(5.0), &IncreasedByMoreThan, &Value::percent(3.0)));
        assert!(compare(
            &Value::percent(5.0),
            &IncreasedByMoreThanEqual,
            &Value::percent(5.0)
        ));
        assert!(compare(&Value::percent(3.0), &IncreasedByLessThan, &Value::percent(5.0)));
        assert!(compare(
            &Value::percent(3.0),
            &IncreasedByLessThanEqual,
            &Value::percent(3.0)
        ));

        assert!(compare(&Value::percent(-3.0), &DecreasedByMoreThan, &Value::percent(5.0)));
        assert!(compare(
            &Value::percent(-5.0),
            &DecreasedByMoreThanEqual,
            &Value::percent(5.0)
        ));
        assert!(compare(&Value::percent(-5.0), &DecreasedByLessThan, &Value::percent(3.0)));
        assert!(compare(
            &Value::percent(-3.0),
            &DecreasedByLessThanEqual,
            &Value::percent(3.0)
        ));

        assert!(compare(&Value::percent(5.0), &MoreThan, &Value::percent(3.0)));
        assert!(compare(&Value::percent(5.0), &MoreThanEqual, &Value::percent(5.0)));
        assert!(compare(&Value::percent(3.0), &LessThan, &Value::percent(5.0)));
        assert!(compare(&Value::percent(3.0), &LessThanEqual, &Value::percent(3.0)));
    }

    #[test]
    fn test_quote_comparisons() {
        assert!(compare(&Value::quote(5.0), &Equal, &Value::quote(5.0)));
        assert!(compare(&Value::quote(3.0), &NotEqual, &Value::quote(5.0)));

        assert!(compare(&Value::quote(5.0), &IncreasedByMoreThan, &Value::quote(3.0)));
        assert!(compare(&Value::quote(5.0), &IncreasedByMoreThanEqual, &Value::quote(5.0)));
        assert!(compare(&Value::quote(3.0), &IncreasedByLessThan, &Value::quote(5.0)));
        assert!(compare(&Value::quote(3.0), &IncreasedByLessThanEqual, &Value::quote(3.0)));

        assert!(compare(&Value::quote(-3.0), &DecreasedByMoreThan, &Value::quote(5.0)));
        assert!(compare(
            &Value::quote(-5.0),
            &DecreasedByMoreThanEqual,
            &Value::quote(5.0)
        ));
        assert!(compare(&Value::quote(-5.0), &DecreasedByLessThan, &Value::quote(3.0)));
        assert!(compare(
            &Value::quote(-3.0),
            &DecreasedByLessThanEqual,
            &Value::quote(3.0)
        ));

        assert!(compare(&Value::quote(5.0), &MoreThan, &Value::quote(3.0)));
        assert!(compare(&Value::quote(5.0), &MoreThanEqual, &Value::quote(5.0)));
        assert!(compare(&Value::quote(3.0), &LessThan, &Value::quote(5.0)));
        assert!(compare(&Value::quote(3.0), &LessThanEqual, &Value::quote(3.0)));
    }

    #[test]
    fn test_string_comparisons() {
        assert!(compare(
            &Value::string("hello".to_string()),
            &Equal,
            &Value::string("hello".to_string())
        ));
        assert!(!compare(
            &Value::string("hello".to_string()),
            &Equal,
            &Value::string("world".to_string())
        ));
        assert!(compare(
            &Value::string("hello".to_string()),
            &NotEqual,
            &Value::string("world".to_string())
        ));
    }

    #[test]
    fn test_usd_comparisons() {
        assert!(compare(&Value::usd(5.0), &Equal, &Value::usd(5.0)));
        assert!(compare(&Value::usd(3.0), &NotEqual, &Value::usd(5.0)));

        assert!(compare(&Value::usd(5.0), &IncreasedByMoreThan, &Value::usd(3.0)));
        assert!(compare(&Value::usd(5.0), &IncreasedByMoreThanEqual, &Value::usd(5.0)));
        assert!(compare(&Value::usd(3.0), &IncreasedByLessThan, &Value::usd(5.0)));
        assert!(compare(&Value::usd(3.0), &IncreasedByLessThanEqual, &Value::usd(3.0)));

        assert!(compare(&Value::usd(-3.0), &DecreasedByMoreThan, &Value::usd(5.0)));
        assert!(compare(&Value::usd(-5.0), &DecreasedByMoreThanEqual, &Value::usd(5.0)));
        assert!(compare(&Value::usd(-5.0), &DecreasedByLessThan, &Value::usd(3.0)));
        assert!(compare(&Value::usd(-3.0), &DecreasedByLessThanEqual, &Value::usd(3.0)));

        assert!(compare(&Value::usd(5.0), &MoreThan, &Value::usd(3.0)));
        assert!(compare(&Value::usd(5.0), &MoreThanEqual, &Value::usd(5.0)));
        assert!(compare(&Value::usd(3.0), &LessThan, &Value::usd(5.0)));
        assert!(compare(&Value::usd(3.0), &LessThanEqual, &Value::usd(3.0)));
    }
}

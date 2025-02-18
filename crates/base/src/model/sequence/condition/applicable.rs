// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Condition, Operator, Value};

impl Condition {
    /// Determines whether a condition can be applied.
    /// E.g. the user provides an empty AND condition, which would match everything
    pub fn applicable(&self) -> bool {
        match self {
            Condition::Compare { value, operator, .. } => match value {
                Value::Boolean(_) => match operator {
                    Operator::Equal | Operator::NotEqual => true,

                    Operator::IncreasedByMoreThan
                    | Operator::IncreasedByMoreThanEqual
                    | Operator::IncreasedByLessThan
                    | Operator::IncreasedByLessThanEqual
                    | Operator::DecreasedByMoreThan
                    | Operator::DecreasedByMoreThanEqual
                    | Operator::DecreasedByLessThan
                    | Operator::DecreasedByLessThanEqual
                    | Operator::MoreThan
                    | Operator::MoreThanEqual
                    | Operator::LessThan
                    | Operator::LessThanEqual => false,
                },
                Value::Count(value) => match operator {
                    Operator::Equal => true,
                    Operator::IncreasedByMoreThan => *value > 0,
                    Operator::IncreasedByMoreThanEqual => *value != 0,
                    Operator::IncreasedByLessThan => *value < u64::MAX,
                    Operator::IncreasedByLessThanEqual => *value != u64::MAX,
                    Operator::DecreasedByMoreThan => *value > 0,
                    Operator::DecreasedByMoreThanEqual => *value != 0,
                    Operator::DecreasedByLessThan => *value < u64::MAX,
                    Operator::DecreasedByLessThanEqual => *value != u64::MAX,
                    Operator::MoreThan => *value > 0,
                    Operator::MoreThanEqual => *value != 0,
                    Operator::LessThan => *value < u64::MAX,
                    Operator::LessThanEqual => *value != u64::MAX,
                    Operator::NotEqual => true,
                },
                Value::Percent(value) => match operator {
                    Operator::Equal => false,
                    Operator::IncreasedByMoreThan => *value != 0.0f64,
                    Operator::IncreasedByMoreThanEqual => *value != 0.0f64,
                    Operator::IncreasedByLessThan => *value != 0.0f64,
                    Operator::IncreasedByLessThanEqual => *value != 0.0f64,
                    Operator::DecreasedByMoreThan => *value != 0.0f64,
                    Operator::DecreasedByMoreThanEqual => *value != 0.0f64,
                    Operator::DecreasedByLessThan => *value != 0.0f64,
                    Operator::DecreasedByLessThanEqual => *value != 0.0f64,
                    Operator::MoreThan => *value != 0.0f64,
                    Operator::MoreThanEqual => *value != 0.0f64,
                    Operator::LessThan => *value != 0.0f64,
                    Operator::LessThanEqual => *value != 0.0f64,
                    Operator::NotEqual => false,
                },
                Value::Quote(value) => match operator {
                    Operator::Equal => false,
                    Operator::IncreasedByMoreThan => *value != 0.0f64,
                    Operator::IncreasedByMoreThanEqual => *value != 0.0f64,
                    Operator::IncreasedByLessThan => *value != 0.0f64,
                    Operator::IncreasedByLessThanEqual => *value != 0.0f64,
                    Operator::DecreasedByMoreThan => *value != 0.0f64,
                    Operator::DecreasedByMoreThanEqual => *value != 0.0f64,
                    Operator::DecreasedByLessThan => *value != 0.0f64,
                    Operator::DecreasedByLessThanEqual => *value != 0.0f64,
                    Operator::MoreThan => *value != 0.0f64,
                    Operator::MoreThanEqual => *value != 0.0f64,
                    Operator::LessThan => *value != 0.0f64,
                    Operator::LessThanEqual => *value != 0.0f64,
                    Operator::NotEqual => false,
                },
                Value::String(value) => match operator {
                    Operator::Equal => value.trim() != "",
                    Operator::NotEqual
                    | Operator::IncreasedByMoreThan
                    | Operator::IncreasedByMoreThanEqual
                    | Operator::IncreasedByLessThan
                    | Operator::IncreasedByLessThanEqual
                    | Operator::DecreasedByMoreThan
                    | Operator::DecreasedByMoreThanEqual
                    | Operator::DecreasedByLessThan
                    | Operator::DecreasedByLessThanEqual
                    | Operator::MoreThan
                    | Operator::MoreThanEqual
                    | Operator::LessThan
                    | Operator::LessThanEqual => false,
                },
                Value::Usd(value) => match operator {
                    Operator::Equal => false,
                    Operator::IncreasedByMoreThan => *value != 0.0f64,
                    Operator::IncreasedByMoreThanEqual => *value != 0.0f64,
                    Operator::IncreasedByLessThan => *value != 0.0f64,
                    Operator::IncreasedByLessThanEqual => *value != 0.0f64,
                    Operator::DecreasedByMoreThan => *value != 0.0f64,
                    Operator::DecreasedByMoreThanEqual => *value != 0.0f64,
                    Operator::DecreasedByLessThan => *value != 0.0f64,
                    Operator::DecreasedByLessThanEqual => *value != 0.0f64,
                    Operator::MoreThan => *value != 0.0f64,
                    Operator::MoreThanEqual => *value != 0.0f64,
                    Operator::LessThan => *value != 0.0f64,
                    Operator::LessThanEqual => *value != 0.0f64,
                    Operator::NotEqual => false,
                },
            },
            Condition::And { conditions } => !conditions.is_empty() && conditions.iter().find(|c| c.applicable() == false).is_none(),
            Condition::Or { conditions } => !conditions.is_empty() && conditions.iter().find(|c| c.applicable() == false).is_none(),
            Condition::AndNot { conditions } => !conditions.is_empty() && conditions.iter().find(|c| c.applicable() == false).is_none(),
        }
    }
}

#[cfg(test)]
mod tests {

    mod compare {
        mod bool {
            use crate::model::Condition::Compare;
            use crate::model::Field::TwitterExists;
            use crate::model::Operator::{
                DecreasedByLessThan, DecreasedByLessThanEqual, DecreasedByMoreThan, DecreasedByMoreThanEqual, Equal, IncreasedByMoreThan,
                IncreasedByMoreThanEqual, LessThan, LessThanEqual, MoreThan, MoreThanEqual, NotEqual,
            };
            use crate::model::Value::Boolean;

            #[test]
            fn equal_ready() {
                let test_instance = Compare {
                    field: TwitterExists,
                    operator: Equal,
                    value: Boolean(false),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), true);

                let test_instance = Compare {
                    field: TwitterExists,
                    operator: Equal,
                    value: Boolean(true),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn not_equal_ready() {
                let test_instance = Compare {
                    field: TwitterExists,
                    operator: NotEqual,
                    value: Boolean(false),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), true);

                let test_instance = Compare {
                    field: TwitterExists,
                    operator: NotEqual,
                    value: Boolean(true),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn increased_by_more_than_not_ready() {
                let test_instance = Compare {
                    field: TwitterExists,
                    operator: IncreasedByMoreThan,
                    value: Boolean(false),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TwitterExists,
                    operator: IncreasedByMoreThan,
                    value: Boolean(true),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn increased_by_more_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TwitterExists,
                    operator: IncreasedByMoreThanEqual,
                    value: Boolean(false),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TwitterExists,
                    operator: IncreasedByMoreThanEqual,
                    value: Boolean(true),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn increased_by_less_than_not_ready() {
                let test_instance = Compare {
                    field: TwitterExists,
                    operator: DecreasedByLessThan,
                    value: Boolean(false),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TwitterExists,
                    operator: DecreasedByLessThan,
                    value: Boolean(true),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn increased_by_less_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TwitterExists,
                    operator: DecreasedByLessThanEqual,
                    value: Boolean(false),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TwitterExists,
                    operator: DecreasedByLessThanEqual,
                    value: Boolean(true),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn decreased_by_more_than_not_ready() {
                let test_instance = Compare {
                    field: TwitterExists,
                    operator: DecreasedByMoreThan,
                    value: Boolean(false),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TwitterExists,
                    operator: DecreasedByMoreThan,
                    value: Boolean(true),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn decreased_by_more_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TwitterExists,
                    operator: DecreasedByMoreThanEqual,
                    value: Boolean(false),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TwitterExists,
                    operator: DecreasedByMoreThanEqual,
                    value: Boolean(true),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn decreased_by_less_than_not_ready() {
                let test_instance = Compare {
                    field: TwitterExists,
                    operator: DecreasedByLessThan,
                    value: Boolean(false),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TwitterExists,
                    operator: DecreasedByLessThan,
                    value: Boolean(true),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn decreased_by_less_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TwitterExists,
                    operator: DecreasedByLessThanEqual,
                    value: Boolean(false),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TwitterExists,
                    operator: DecreasedByLessThanEqual,
                    value: Boolean(true),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn more_than_not_ready() {
                let test_instance = Compare {
                    field: TwitterExists,
                    operator: MoreThan,
                    value: Boolean(false),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TwitterExists,
                    operator: MoreThan,
                    value: Boolean(true),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn more_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TwitterExists,
                    operator: MoreThanEqual,
                    value: Boolean(false),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TwitterExists,
                    operator: MoreThanEqual,
                    value: Boolean(true),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn less_than_not_ready() {
                let test_instance = Compare {
                    field: TwitterExists,
                    operator: LessThan,
                    value: Boolean(false),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TwitterExists,
                    operator: LessThan,
                    value: Boolean(true),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn less_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TwitterExists,
                    operator: LessThanEqual,
                    value: Boolean(false),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TwitterExists,
                    operator: LessThanEqual,
                    value: Boolean(true),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false)
            }
        }

        mod count {
            use crate::model::Operator::{
                DecreasedByLessThan, DecreasedByLessThanEqual, DecreasedByMoreThan, DecreasedByMoreThanEqual, Equal, IncreasedByLessThan,
                IncreasedByLessThanEqual, IncreasedByMoreThan, IncreasedByMoreThanEqual, NotEqual,
            };
            use crate::model::{Condition, Field, Operator, Value};
            use common::model::Timeframe::M15;
            use Condition::Compare;
            use Field::TradesBuy;
            use Operator::{LessThan, LessThanEqual, MoreThan, MoreThanEqual};
            use Value::Count;

            #[test]
            fn increased_by_more_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByMoreThan,
                    value: Count(1),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn increased_by_more_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByMoreThan,
                    value: Count(0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn increased_by_more_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByMoreThanEqual,
                    value: Count(1),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn increased_by_more_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByMoreThanEqual,
                    value: Count(0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn increased_by_less_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByLessThan,
                    value: Count(u64::MAX - 1),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn increased_by_less_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByLessThan,
                    value: Count(u64::MAX),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn increased_by_less_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByLessThanEqual,
                    value: Count(u64::MAX - 1),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn increased_by_less_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByLessThanEqual,
                    value: Count(u64::MAX),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn decreased_by_more_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByMoreThan,
                    value: Count(1),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn decreased_by_more_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByMoreThan,
                    value: Count(0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn decreased_by_more_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByMoreThanEqual,
                    value: Count(1),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn decreased_by_more_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByMoreThanEqual,
                    value: Count(0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn decreased_by_less_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByLessThan,
                    value: Count(u64::MAX - 1),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn decreased_by_less_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByLessThan,
                    value: Count(u64::MAX),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn decreased_by_less_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByLessThanEqual,
                    value: Count(u64::MAX - 1),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn decreased_by_less_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByLessThanEqual,
                    value: Count(u64::MAX),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn more_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: MoreThan,
                    value: Count(1),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn more_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: MoreThan,
                    value: Count(0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn more_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: MoreThanEqual,
                    value: Count(1),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn more_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: MoreThanEqual,
                    value: Count(0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn less_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: LessThan,
                    value: Count(u64::MAX - 1),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn less_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: LessThan,
                    value: Count(u64::MAX),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn less_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: LessThanEqual,
                    value: Count(u64::MAX - 1),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn less_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: LessThanEqual,
                    value: Count(u64::MAX),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: Equal,
                    value: Count(0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true);

                let test_instance = Compare {
                    field: TradesBuy,
                    operator: Equal,
                    value: Count(1),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true);

                let test_instance = Compare {
                    field: TradesBuy,
                    operator: Equal,
                    value: Count(u64::MAX),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true);
            }

            #[test]
            fn not_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: NotEqual,
                    value: Count(0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true);

                let test_instance = Compare {
                    field: TradesBuy,
                    operator: NotEqual,
                    value: Count(1),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true);

                let test_instance = Compare {
                    field: TradesBuy,
                    operator: NotEqual,
                    value: Count(u64::MAX),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true);
            }
        }

        mod percent {
            use crate::model::Operator::{
                DecreasedByLessThan, DecreasedByLessThanEqual, DecreasedByMoreThan, DecreasedByMoreThanEqual, Equal, IncreasedByLessThan,
                IncreasedByLessThanEqual, IncreasedByMoreThan, IncreasedByMoreThanEqual, NotEqual,
            };
            use crate::model::{Condition, Field, Operator, Value};
            use common::model::Timeframe::M15;
            use Condition::Compare;
            use Field::TradesBuy;
            use Operator::{LessThan, LessThanEqual, MoreThan, MoreThanEqual};
            use Value::Percent;

            #[test]
            fn increased_by_more_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByMoreThan,
                    value: Percent(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn increased_by_more_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByMoreThan,
                    value: Percent(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn increased_by_more_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByMoreThanEqual,
                    value: Percent(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn increased_by_more_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByMoreThanEqual,
                    value: Percent(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn increased_by_less_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByLessThan,
                    value: Percent(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn increased_by_less_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByLessThan,
                    value: Percent(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn increased_by_less_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByLessThanEqual,
                    value: Percent(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn increased_by_less_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByLessThanEqual,
                    value: Percent(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn decreased_by_more_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByMoreThan,
                    value: Percent(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn decreased_by_more_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByMoreThan,
                    value: Percent(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn decreased_by_more_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByMoreThanEqual,
                    value: Percent(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn decreased_by_more_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByMoreThanEqual,
                    value: Percent(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn decreased_by_less_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByLessThan,
                    value: Percent(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn decreased_by_less_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByLessThan,
                    value: Percent(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn decreased_by_less_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByLessThanEqual,
                    value: Percent(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn decreased_by_less_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByLessThanEqual,
                    value: Percent(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn more_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: MoreThan,
                    value: Percent(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn more_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: MoreThan,
                    value: Percent(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn more_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: MoreThanEqual,
                    value: Percent(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn more_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: MoreThanEqual,
                    value: Percent(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn less_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: LessThan,
                    value: Percent(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn less_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: LessThan,
                    value: Percent(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn less_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: LessThanEqual,
                    value: Percent(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn less_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: LessThanEqual,
                    value: Percent(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: Equal,
                    value: Percent(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TradesBuy,
                    operator: Equal,
                    value: Percent(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TradesBuy,
                    operator: Equal,
                    value: Percent(f64::MAX),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false);
            }

            #[test]
            fn not_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: NotEqual,
                    value: Percent(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TradesBuy,
                    operator: NotEqual,
                    value: Percent(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TradesBuy,
                    operator: NotEqual,
                    value: Percent(f64::MAX),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false);
            }
        }

        mod quote {
            use crate::model::Operator::{
                DecreasedByLessThan, DecreasedByLessThanEqual, DecreasedByMoreThan, DecreasedByMoreThanEqual, Equal, IncreasedByLessThan,
                IncreasedByLessThanEqual, IncreasedByMoreThan, IncreasedByMoreThanEqual, NotEqual,
            };
            use crate::model::{Condition, Field, Operator, Value};
            use common::model::Timeframe::M15;
            use Condition::Compare;
            use Field::TradesBuy;
            use Operator::{LessThan, LessThanEqual, MoreThan, MoreThanEqual};
            use Value::Quote;

            #[test]
            fn increased_by_more_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByMoreThan,
                    value: Quote(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn increased_by_more_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByMoreThan,
                    value: Quote(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn increased_by_more_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByMoreThanEqual,
                    value: Quote(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn increased_by_more_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByMoreThanEqual,
                    value: Quote(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn increased_by_less_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByLessThan,
                    value: Quote(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn increased_by_less_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByLessThan,
                    value: Quote(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn increased_by_less_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByLessThanEqual,
                    value: Quote(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn increased_by_less_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByLessThanEqual,
                    value: Quote(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn decreased_by_more_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByMoreThan,
                    value: Quote(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn decreased_by_more_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByMoreThan,
                    value: Quote(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn decreased_by_more_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByMoreThanEqual,
                    value: Quote(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn decreased_by_more_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByMoreThanEqual,
                    value: Quote(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn decreased_by_less_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByLessThan,
                    value: Quote(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn decreased_by_less_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByLessThan,
                    value: Quote(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn decreased_by_less_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByLessThanEqual,
                    value: Quote(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn decreased_by_less_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByLessThanEqual,
                    value: Quote(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn more_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: MoreThan,
                    value: Quote(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn more_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: MoreThan,
                    value: Quote(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn more_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: MoreThanEqual,
                    value: Quote(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn more_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: MoreThanEqual,
                    value: Quote(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn less_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: LessThan,
                    value: Quote(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn less_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: LessThan,
                    value: Quote(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn less_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: LessThanEqual,
                    value: Quote(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn less_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: LessThanEqual,
                    value: Quote(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: Equal,
                    value: Quote(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TradesBuy,
                    operator: Equal,
                    value: Quote(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TradesBuy,
                    operator: Equal,
                    value: Quote(f64::MAX),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false);
            }

            #[test]
            fn not_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: NotEqual,
                    value: Quote(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TradesBuy,
                    operator: NotEqual,
                    value: Quote(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TradesBuy,
                    operator: NotEqual,
                    value: Quote(f64::MAX),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false);
            }
        }

        mod string {
            use crate::model::Condition::Compare;
            use crate::model::Field::TwitterHandle;
            use crate::model::Operator::{
                DecreasedByLessThan, DecreasedByLessThanEqual, DecreasedByMoreThan, DecreasedByMoreThanEqual, Equal, IncreasedByLessThan,
                IncreasedByLessThanEqual, IncreasedByMoreThan, IncreasedByMoreThanEqual, LessThan, LessThanEqual, MoreThan, MoreThanEqual, NotEqual,
            };
            use crate::model::Value::String;

            #[test]
            fn equal_ready() {
                let test_instance = Compare {
                    field: TwitterHandle,
                    operator: Equal,
                    value: String("AI_nyanbot".to_string()),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), true);
            }

            #[test]
            fn equal_not_ready() {
                let test_instance = Compare {
                    field: TwitterHandle,
                    operator: Equal,
                    value: String("   ".to_string()),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false);
            }

            #[test]
            fn not_equal_not_ready() {
                let test_instance = Compare {
                    field: TwitterHandle,
                    operator: NotEqual,
                    value: String("AI_nyanbot".to_string()),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TwitterHandle,
                    operator: NotEqual,
                    value: String("   ".to_string()),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false);
            }

            #[test]
            fn increased_by_more_than_not_ready() {
                let test_instance = Compare {
                    field: TwitterHandle,
                    operator: IncreasedByMoreThan,
                    value: String("AI_nyanbot".to_string()),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn increased_by_more_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TwitterHandle,
                    operator: IncreasedByMoreThanEqual,
                    value: String("AI_nyanbot".to_string()),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false);
            }

            #[test]
            fn increased_by_less_than_not_ready() {
                let test_instance = Compare {
                    field: TwitterHandle,
                    operator: IncreasedByLessThan,
                    value: String("AI_nyanbot".to_string()),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn increased_by_less_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TwitterHandle,
                    operator: IncreasedByLessThanEqual,
                    value: String("AI_nyanbot".to_string()),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false);
            }

            #[test]
            fn decreased_by_more_than_not_ready() {
                let test_instance = Compare {
                    field: TwitterHandle,
                    operator: DecreasedByMoreThan,
                    value: String("AI_nyanbot".to_string()),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn decreased_by_more_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TwitterHandle,
                    operator: DecreasedByMoreThanEqual,
                    value: String("AI_nyanbot".to_string()),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false);
            }

            #[test]
            fn decreased_by_less_than_not_ready() {
                let test_instance = Compare {
                    field: TwitterHandle,
                    operator: DecreasedByLessThan,
                    value: String("AI_nyanbot".to_string()),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn decreased_by_less_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TwitterHandle,
                    operator: DecreasedByLessThanEqual,
                    value: String("AI_nyanbot".to_string()),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false);
            }

            #[test]
            fn more_than_not_ready() {
                let test_instance = Compare {
                    field: TwitterHandle,
                    operator: MoreThan,
                    value: String("AI_nyanbot".to_string()),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn more_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TwitterHandle,
                    operator: MoreThanEqual,
                    value: String("AI_nyanbot".to_string()),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false);
            }

            #[test]
            fn less_than_not_ready() {
                let test_instance = Compare {
                    field: TwitterHandle,
                    operator: LessThan,
                    value: String("AI_nyanbot".to_string()),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn less_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TwitterHandle,
                    operator: LessThanEqual,
                    value: String("AI_nyanbot".to_string()),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false);
            }
        }

        mod usd {
            use crate::model::Operator::{
                DecreasedByLessThan, DecreasedByLessThanEqual, DecreasedByMoreThan, DecreasedByMoreThanEqual, Equal, IncreasedByLessThan,
                IncreasedByLessThanEqual, IncreasedByMoreThan, IncreasedByMoreThanEqual, NotEqual,
            };
            use crate::model::{Condition, Field, Operator, Value};
            use common::model::Timeframe::M15;
            use Condition::Compare;
            use Field::TradesBuy;
            use Operator::{LessThan, LessThanEqual, MoreThan, MoreThanEqual};
            use Value::Usd;

            #[test]
            fn increased_by_more_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByMoreThan,
                    value: Usd(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn increased_by_more_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByMoreThan,
                    value: Usd(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn increased_by_more_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByMoreThanEqual,
                    value: Usd(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn increased_by_more_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByMoreThanEqual,
                    value: Usd(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn increased_by_less_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByLessThan,
                    value: Usd(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn increased_by_less_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByLessThan,
                    value: Usd(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn increased_by_less_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByLessThanEqual,
                    value: Usd(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn increased_by_less_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: IncreasedByLessThanEqual,
                    value: Usd(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn decreased_by_more_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByMoreThan,
                    value: Usd(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn decreased_by_more_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByMoreThan,
                    value: Usd(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn decreased_by_more_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByMoreThanEqual,
                    value: Usd(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn decreased_by_more_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByMoreThanEqual,
                    value: Usd(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn decreased_by_less_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByLessThan,
                    value: Usd(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn decreased_by_less_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByLessThan,
                    value: Usd(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn decreased_by_less_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByLessThanEqual,
                    value: Usd(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn decreased_by_less_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: DecreasedByLessThanEqual,
                    value: Usd(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn more_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: MoreThan,
                    value: Usd(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn more_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: MoreThan,
                    value: Usd(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn more_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: MoreThanEqual,
                    value: Usd(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn more_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: MoreThanEqual,
                    value: Usd(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn less_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: LessThan,
                    value: Usd(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn less_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: LessThan,
                    value: Usd(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn less_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: LessThanEqual,
                    value: Usd(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn less_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: LessThanEqual,
                    value: Usd(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: Equal,
                    value: Usd(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TradesBuy,
                    operator: Equal,
                    value: Usd(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TradesBuy,
                    operator: Equal,
                    value: Usd(f64::MAX),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false);
            }

            #[test]
            fn not_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: NotEqual,
                    value: Usd(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TradesBuy,
                    operator: NotEqual,
                    value: Usd(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TradesBuy,
                    operator: NotEqual,
                    value: Usd(f64::MAX),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false);
            }
        }
    }

    mod and {
        use crate::model::Condition;
        use crate::model::Condition::Compare;
        use crate::model::Field::{TradesBuy, TwitterHandle};
        use crate::model::Operator::{Equal, MoreThan, NotEqual};
        use crate::model::Value::{Percent, String};
        use common::model::Timeframe::M15;

        #[test]
        fn empty() {
            let test_instance = Condition::And { conditions: vec![] };
            assert_eq!(test_instance.applicable(), false)
        }

        #[test]
        fn single_child_applicable() {
            let test_instance = Condition::And {
                conditions: vec![Compare {
                    field: TwitterHandle,
                    operator: Equal,
                    value: String("AI_nyanbot".to_string()),
                    timeframe: None,
                }],
            };
            assert_eq!(test_instance.applicable(), true)
        }

        #[test]
        fn single_child_not_applicable() {
            let test_instance = Condition::And {
                conditions: vec![Compare {
                    field: TwitterHandle,
                    operator: NotEqual,
                    value: String("AI_nyanbot".to_string()),
                    timeframe: None,
                }],
            };
            assert_eq!(test_instance.applicable(), false)
        }

        #[test]
        fn children_applicable() {
            let test_instance = Condition::And {
                conditions: vec![
                    Compare {
                        field: TradesBuy,
                        operator: MoreThan,
                        value: Percent(1.0),
                        timeframe: Some(M15),
                    },
                    Compare {
                        field: TwitterHandle,
                        operator: Equal,
                        value: String("AI_nyanbot".to_string()),
                        timeframe: None,
                    },
                ],
            };
            assert_eq!(test_instance.applicable(), true)
        }

        #[test]
        fn children_partial_applicable() {
            let test_instance = Condition::And {
                conditions: vec![
                    // applicable
                    Compare {
                        field: TradesBuy,
                        operator: MoreThan,
                        value: Percent(1.0),
                        timeframe: Some(M15),
                    },
                    // Not applicable
                    Compare {
                        field: TwitterHandle,
                        operator: NotEqual,
                        value: String("AI_nyanbot".to_string()),
                        timeframe: None,
                    },
                ],
            };
            assert_eq!(test_instance.applicable(), false)
        }

        #[test]
        fn children_not_applicable() {
            let test_instance = Condition::And {
                conditions: vec![
                    Condition::And { conditions: vec![] },
                    Condition::Or { conditions: vec![] },
                    Condition::AndNot { conditions: vec![] },
                ],
            };
            assert_eq!(test_instance.applicable(), false)
        }
    }

    mod or {
        use crate::model::Condition;
        use crate::model::Condition::Compare;
        use crate::model::Field::{TradesBuy, TwitterHandle};
        use crate::model::Operator::{Equal, MoreThan, NotEqual};
        use crate::model::Value::{Percent, String};
        use common::model::Timeframe::M15;
        use Condition::Or;

        #[test]
        fn empty() {
            let test_instance = Or { conditions: vec![] };
            assert_eq!(test_instance.applicable(), false)
        }

        #[test]
        fn single_child_applicable() {
            let test_instance = Or {
                conditions: vec![Compare {
                    field: TwitterHandle,
                    operator: Equal,
                    value: String("AI_nyanbot".to_string()),
                    timeframe: None,
                }],
            };
            assert_eq!(test_instance.applicable(), true)
        }

        #[test]
        fn single_child_not_applicable() {
            let test_instance = Or {
                conditions: vec![Compare {
                    field: TwitterHandle,
                    operator: NotEqual,
                    value: String("AI_nyanbot".to_string()),
                    timeframe: None,
                }],
            };
            assert_eq!(test_instance.applicable(), false)
        }

        #[test]
        fn children_applicable() {
            let test_instance = Or {
                conditions: vec![
                    Compare {
                        field: TradesBuy,
                        operator: MoreThan,
                        value: Percent(1.0),
                        timeframe: Some(M15),
                    },
                    Compare {
                        field: TwitterHandle,
                        operator: Equal,
                        value: String("AI_nyanbot".to_string()),
                        timeframe: None,
                    },
                ],
            };
            assert_eq!(test_instance.applicable(), true)
        }

        #[test]
        fn children_partial_applicable() {
            let test_instance = Or {
                conditions: vec![
                    // applicable
                    Compare {
                        field: TradesBuy,
                        operator: MoreThan,
                        value: Percent(1.0),
                        timeframe: Some(M15),
                    },
                    // Not applicable
                    Compare {
                        field: TwitterHandle,
                        operator: NotEqual,
                        value: String("AI_nyanbot".to_string()),
                        timeframe: None,
                    },
                ],
            };
            assert_eq!(test_instance.applicable(), false)
        }

        #[test]
        fn children_not_applicable() {
            let test_instance = Or {
                conditions: vec![
                    Condition::And { conditions: vec![] },
                    Or { conditions: vec![] },
                    Condition::AndNot { conditions: vec![] },
                ],
            };
            assert_eq!(test_instance.applicable(), false)
        }
    }

    mod and_not {
        use crate::model::Condition;
        use crate::model::Condition::{And, Compare, Or};
        use crate::model::Field::{TradesBuy, TwitterHandle};
        use crate::model::Operator::{Equal, MoreThan, NotEqual};
        use crate::model::Value::{Percent, String};
        use common::model::Timeframe::M15;
        use Condition::AndNot;

        #[test]
        fn empty() {
            let test_instance = AndNot { conditions: vec![] };
            assert_eq!(test_instance.applicable(), false)
        }

        #[test]
        fn single_child_applicable() {
            let test_instance = AndNot {
                conditions: vec![Compare {
                    field: TwitterHandle,
                    operator: Equal,
                    value: String("AI_nyanbot".to_string()),
                    timeframe: None,
                }],
            };
            assert_eq!(test_instance.applicable(), true)
        }

        #[test]
        fn single_child_not_applicable() {
            let test_instance = AndNot {
                conditions: vec![Compare {
                    field: TwitterHandle,
                    operator: NotEqual,
                    value: String("AI_nyanbot".to_string()),
                    timeframe: None,
                }],
            };
            assert_eq!(test_instance.applicable(), false)
        }

        #[test]
        fn children_applicable() {
            let test_instance = AndNot {
                conditions: vec![
                    Compare {
                        field: TradesBuy,
                        operator: MoreThan,
                        value: Percent(1.0),
                        timeframe: Some(M15),
                    },
                    Compare {
                        field: TwitterHandle,
                        operator: Equal,
                        value: String("AI_nyanbot".to_string()),
                        timeframe: None,
                    },
                ],
            };
            assert_eq!(test_instance.applicable(), true)
        }

        #[test]
        fn children_partial_applicable() {
            let test_instance = AndNot {
                conditions: vec![
                    // applicable
                    Compare {
                        field: TradesBuy,
                        operator: MoreThan,
                        value: Percent(1.0),
                        timeframe: Some(M15),
                    },
                    // Not applicable
                    Compare {
                        field: TwitterHandle,
                        operator: NotEqual,
                        value: String("AI_nyanbot".to_string()),
                        timeframe: None,
                    },
                ],
            };
            assert_eq!(test_instance.applicable(), false)
        }

        #[test]
        fn children_not_applicable() {
            let test_instance = AndNot {
                conditions: vec![And { conditions: vec![] }, Or { conditions: vec![] }, AndNot { conditions: vec![] }],
            };
            assert_eq!(test_instance.applicable(), false)
        }
    }
}

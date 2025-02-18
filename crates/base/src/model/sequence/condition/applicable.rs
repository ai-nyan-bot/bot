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
                    Operator::GreaterThan | Operator::GreaterThanEqual | Operator::LessThan | Operator::LessThanEqual => false,
                },
                Value::Count(value) => match operator {
                    Operator::Equal => true,
                    Operator::GreaterThan => *value > 0,
                    Operator::GreaterThanEqual => *value != 0,
                    Operator::LessThan => *value < u64::MAX,
                    Operator::LessThanEqual => *value != u64::MAX,
                    Operator::NotEqual => true,
                },
                Value::Percent(value) => match operator {
                    Operator::Equal => false,
                    Operator::GreaterThan => *value != 0.0f64,
                    Operator::GreaterThanEqual => *value != 0.0f64,
                    Operator::LessThan => *value != 0.0f64,
                    Operator::LessThanEqual => *value != 0.0f64,
                    Operator::NotEqual => false,
                },
                Value::Quote(value) => match operator {
                    Operator::Equal => false,
                    Operator::GreaterThan => *value != 0.0f64,
                    Operator::GreaterThanEqual => *value != 0.0f64,
                    Operator::LessThan => *value != 0.0f64,
                    Operator::LessThanEqual => *value != 0.0f64,
                    Operator::NotEqual => false,
                },
                Value::String(value) => match operator {
                    Operator::Equal => value.trim() != "",
                    Operator::NotEqual => false,
                    Operator::GreaterThan | Operator::GreaterThanEqual | Operator::LessThan | Operator::LessThanEqual => false,
                },
                Value::Usd(value) => match operator {
                    Operator::Equal => false,
                    Operator::GreaterThan => *value != 0.0f64,
                    Operator::GreaterThanEqual => *value != 0.0f64,
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
            use crate::model::Operator::{Equal, GreaterThan, GreaterThanEqual, LessThan, LessThanEqual, NotEqual};
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
            fn greater_than_not_ready() {
                let test_instance = Compare {
                    field: TwitterExists,
                    operator: GreaterThan,
                    value: Boolean(false),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TwitterExists,
                    operator: GreaterThan,
                    value: Boolean(true),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn greater_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TwitterExists,
                    operator: GreaterThanEqual,
                    value: Boolean(false),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false);

                let test_instance = Compare {
                    field: TwitterExists,
                    operator: GreaterThanEqual,
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
            use crate::model::Operator::{Equal, NotEqual};
            use crate::model::{Condition, Field, Operator, Value};
            use common::model::Timeframe::M15;
            use Condition::Compare;
            use Field::TradesBuy;
            use Operator::{GreaterThan, GreaterThanEqual, LessThan, LessThanEqual};
            use Value::Count;

            #[test]
            fn greater_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: GreaterThan,
                    value: Count(1),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn greater_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: GreaterThan,
                    value: Count(0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn greater_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: GreaterThanEqual,
                    value: Count(1),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn greater_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: GreaterThanEqual,
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
            use crate::model::Operator::{Equal, NotEqual};
            use crate::model::{Condition, Field, Operator, Value};
            use common::model::Timeframe::M15;
            use Condition::Compare;
            use Field::TradesBuy;
            use Operator::{GreaterThan, GreaterThanEqual, LessThan, LessThanEqual};
            use Value::Percent;

            #[test]
            fn greater_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: GreaterThan,
                    value: Percent(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn greater_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: GreaterThan,
                    value: Percent(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn greater_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: GreaterThanEqual,
                    value: Percent(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn greater_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: GreaterThanEqual,
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
            use crate::model::Operator::{Equal, NotEqual};
            use crate::model::{Condition, Field, Operator, Value};
            use common::model::Timeframe::M15;
            use Condition::Compare;
            use Field::TradesBuy;
            use Operator::{GreaterThan, GreaterThanEqual, LessThan, LessThanEqual};
            use Value::Quote;

            #[test]
            fn greater_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: GreaterThan,
                    value: Quote(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn greater_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: GreaterThan,
                    value: Quote(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn greater_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: GreaterThanEqual,
                    value: Quote(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn greater_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: GreaterThanEqual,
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
            use crate::model::Operator::{Equal, GreaterThan, GreaterThanEqual, LessThan, LessThanEqual, NotEqual};
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
            fn greater_than_not_ready() {
                let test_instance = Compare {
                    field: TwitterHandle,
                    operator: GreaterThan,
                    value: String("AI_nyanbot".to_string()),
                    timeframe: None,
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn greater_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TwitterHandle,
                    operator: GreaterThanEqual,
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
            use crate::model::Operator::{Equal, NotEqual};
            use crate::model::{Condition, Field, Operator, Value};
            use common::model::Timeframe::M15;
            use Condition::Compare;
            use Field::TradesBuy;
            use Operator::{GreaterThan, GreaterThanEqual, LessThan, LessThanEqual};
            use Value::Usd;

            #[test]
            fn greater_than_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: GreaterThan,
                    value: Usd(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn greater_than_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: GreaterThan,
                    value: Usd(0.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), false)
            }

            #[test]
            fn greater_than_equal_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: GreaterThanEqual,
                    value: Usd(1.0),
                    timeframe: Some(M15),
                };
                assert_eq!(test_instance.applicable(), true)
            }

            #[test]
            fn greater_than_equal_not_ready() {
                let test_instance = Compare {
                    field: TradesBuy,
                    operator: GreaterThanEqual,
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
        use crate::model::Operator::{Equal, GreaterThan, NotEqual};
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
                        operator: GreaterThan,
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
                        operator: GreaterThan,
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
        use crate::model::Operator::{Equal, GreaterThan, NotEqual};
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
                        operator: GreaterThan,
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
                        operator: GreaterThan,
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
        use crate::model::Operator::{Equal, GreaterThan, NotEqual};
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
                        operator: GreaterThan,
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
                        operator: GreaterThan,
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

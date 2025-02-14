// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{compare, Fact, Facts, Field, Operator, Value};
use common::model::Timeframe;
use serde::{Deserialize, Serialize};

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

impl Condition {
    pub fn test(&self, facts: &Facts) -> bool {
        match self {
            Condition::Compare {
                timeframe, operator, value, ..
            } => {
                let fact = Fact::try_from(self).unwrap(); // FIXME

                let result = match timeframe {
                    None => facts.get(&fact),
                    Some(timeframe) => facts.get_with_timeframe(&fact, timeframe),
                };

                if let Some(fact_value) = result {
                    compare(fact_value, operator, value)
                } else {
                    false
                }
            }
            Condition::And { conditions } => {
                for condition in conditions {
                    if !condition.test(facts) {
                        return false;
                    }
                }
                true
            }
            Condition::Or { conditions } => {
                for condition in conditions {
                    if condition.test(facts) {
                        return true;
                    }
                }
                false
            }
            Condition::AndNot { conditions } => {
                for condition in conditions {
                    if condition.test(facts) {
                        return false;
                    }
                }
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn facts() -> Facts {
        Facts::new()
            .with_value(Fact::TokenPriceQuote, Value::Quote(1.0))
            .unwrap()
            .with_value(Fact::TokenPriceUsd, Value::Usd(2.0))
            .unwrap()
            .with_timeframe_value(Fact::TokenVolumeChangeQuote, Value::Quote(3.0), Timeframe::S1)
            .unwrap()
            .with_timeframe_value(Fact::TokenVolumeChangeQuote, Value::Quote(4.0), Timeframe::M1)
            .unwrap()
    }

    mod without_timeframe {
        use crate::model::sequence::condition::tests::facts;
        use crate::model::Field::Price;
        use crate::model::Value::Usd;
        use crate::model::{Condition, Field, Operator, Value};
        use Condition::{And, AndNot, Compare, Or};
        use Field::Volume;
        use Operator::Equal;
        use Value::Quote;

        #[test]
        fn test_equal_true() {
            assert_eq!(
                Compare {
                    field: Price,
                    operator: Equal,
                    value: Quote(1.0),
                    timeframe: None,
                }
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_equal_false() {
            assert_eq!(
                Compare {
                    field: Price,
                    operator: Equal,
                    value: Quote(1337.0),
                    timeframe: None,
                }
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_both_true() {
            assert_eq!(
                And {
                    conditions: vec![
                        Compare {
                            field: Price,
                            operator: Equal,
                            value: Quote(1.0),
                            timeframe: None,
                        },
                        Compare {
                            field: Price,
                            operator: Equal,
                            value: Usd(2.0),
                            timeframe: None,
                        }
                    ]
                }
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_and_left_true() {
            assert_eq!(
                And {
                    conditions: vec![
                        Compare {
                            field: Price,
                            operator: Equal,
                            value: Quote(1.0),
                            timeframe: None
                        },
                        Compare {
                            field: Volume,
                            operator: Equal,
                            value: Quote(22222.0),
                            timeframe: None,
                        }
                    ]
                }
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_right_true() {
            assert_eq!(
                And {
                    conditions: vec![
                        Compare {
                            field: Price,
                            operator: Equal,
                            value: Quote(111111.0),
                            timeframe: None,
                        },
                        Compare {
                            field: Volume,
                            operator: Equal,
                            value: Quote(2.0),
                            timeframe: None,
                        }
                    ]
                }
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_both_false() {
            assert_eq!(
                And {
                    conditions: vec![
                        Compare {
                            field: Price,
                            operator: Equal,
                            value: Quote(111111.0),
                            timeframe: None,
                        },
                        Compare {
                            field: Volume,
                            operator: Equal,
                            value: Quote(22222.0),
                            timeframe: None,
                        }
                    ]
                }
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_or_both_true() {
            assert_eq!(
                Or {
                    conditions: vec![
                        Compare {
                            field: Price,
                            operator: Equal,
                            value: Quote(1.0),
                            timeframe: None,
                        },
                        Compare {
                            field: Volume,
                            operator: Equal,
                            value: Quote(2.0),
                            timeframe: None,
                        }
                    ]
                }
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_or_left_true() {
            assert_eq!(
                Or {
                    conditions: vec![
                        Compare {
                            field: Price,
                            operator: Equal,
                            value: Quote(1.0),
                            timeframe: None,
                        },
                        Compare {
                            field: Volume,
                            operator: Equal,
                            value: Quote(22222.0),
                            timeframe: None,
                        }
                    ]
                }
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_or_right_true() {
            assert_eq!(
                Or {
                    conditions: vec![
                        Compare {
                            field: Price,
                            operator: Equal,
                            value: Quote(111111.0),
                            timeframe: None,
                        },
                        Compare {
                            field: Price,
                            operator: Equal,
                            value: Usd(2.0),
                            timeframe: None,
                        }
                    ]
                }
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_or_both_false() {
            assert_eq!(
                Or {
                    conditions: vec![
                        Compare {
                            field: Price,
                            operator: Equal,
                            value: Quote(111111.0),
                            timeframe: None,
                        },
                        Compare {
                            field: Volume,
                            operator: Equal,
                            value: Quote(22222.0),
                            timeframe: None,
                        }
                    ]
                }
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_not_both_true() {
            assert_eq!(
                AndNot {
                    conditions: vec![
                        Compare {
                            field: Price,
                            operator: Equal,
                            value: Quote(1.0),
                            timeframe: None,
                        },
                        Compare {
                            field: Volume,
                            operator: Equal,
                            value: Quote(2.0),
                            timeframe: None,
                        }
                    ]
                }
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_not_left_true() {
            assert_eq!(
                AndNot {
                    conditions: vec![
                        Compare {
                            field: Price,
                            operator: Equal,
                            value: Quote(1.0),
                            timeframe: None
                        },
                        Compare {
                            field: Volume,
                            operator: Equal,
                            value: Quote(22222.0),
                            timeframe: None,
                        }
                    ]
                }
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_not_right_true() {
            assert_eq!(
                AndNot {
                    conditions: vec![
                        Compare {
                            field: Price,
                            operator: Equal,
                            value: Quote(111111.0),
                            timeframe: None,
                        },
                        Compare {
                            field: Price,
                            operator: Equal,
                            value: Usd(2.0),
                            timeframe: None,
                        }
                    ]
                }
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_not_both_false() {
            assert_eq!(
                AndNot {
                    conditions: vec![
                        Compare {
                            field: Price,
                            operator: Equal,
                            value: Quote(111111.0),
                            timeframe: None,
                        },
                        Compare {
                            field: Volume,
                            operator: Equal,
                            value: Quote(22222.0),
                            timeframe: None,
                        }
                    ]
                }
                .test(&facts()),
                true
            )
        }
    }

    mod with_timeframe {
        use crate::model::sequence::condition::tests::facts;
        use crate::model::Condition::{And, AndNot, Or};
        use crate::model::{Condition, Field, Operator, Value};
        use common::model::Timeframe;
        use Condition::Compare;
        use Operator::Equal;
        use Timeframe::{M1, S1};
        use Value::Quote;

        #[test]
        fn test_equal_true() {
            assert_eq!(
                Compare {
                    field: Field::Volume,
                    operator: Equal,
                    value: Quote(3.0),
                    timeframe: Some(S1),
                }
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_equal_false() {
            assert_eq!(
                Compare {
                    field: Field::Volume,
                    operator: Equal,
                    value: Quote(1337.0),
                    timeframe: Some(M1),
                }
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_both_true() {
            assert_eq!(
                And {
                    conditions: vec![
                        Compare {
                            field: Field::Volume,
                            operator: Equal,
                            value: Quote(3.0),
                            timeframe: Some(S1),
                        },
                        Compare {
                            field: Field::Volume,
                            operator: Equal,
                            value: Quote(4.0),
                            timeframe: Some(M1),
                        }
                    ]
                }
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_and_left_true() {
            assert_eq!(
                And {
                    conditions: vec![
                        Compare {
                            field: Field::Volume,
                            operator: Equal,
                            value: Quote(3.0),
                            timeframe: Some(S1),
                        },
                        Compare {
                            field: Field::Volume,
                            operator: Equal,
                            value: Quote(444444.0),
                            timeframe: Some(M1),
                        }
                    ]
                }
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_right_true() {
            assert_eq!(
                And {
                    conditions: vec![
                        Compare {
                            field: Field::Volume,
                            operator: Equal,
                            value: Quote(33333.0),
                            timeframe: Some(S1),
                        },
                        Compare {
                            field: Field::Volume,
                            operator: Equal,
                            value: Quote(4.0),
                            timeframe: Some(M1),
                        }
                    ]
                }
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_both_false() {
            assert_eq!(
                And {
                    conditions: vec![
                        Compare {
                            field: Field::Volume,
                            operator: Equal,
                            value: Quote(33333.0),
                            timeframe: Some(S1),
                        },
                        Compare {
                            field: Field::Volume,
                            operator: Equal,
                            value: Quote(44444.0),
                            timeframe: Some(M1),
                        }
                    ]
                }
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_or_both_true() {
            assert_eq!(
                Or {
                    conditions: vec![
                        Compare {
                            field: Field::Volume,
                            operator: Equal,
                            value: Quote(3.0),
                            timeframe: Some(S1),
                        },
                        Compare {
                            field: Field::Volume,
                            operator: Equal,
                            value: Quote(4.0),
                            timeframe: Some(M1),
                        }
                    ]
                }
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_or_left_true() {
            assert_eq!(
                Or {
                    conditions: vec![
                        Compare {
                            field: Field::Volume,
                            operator: Equal,
                            value: Quote(3.0),
                            timeframe: Some(S1),
                        },
                        Compare {
                            field: Field::Volume,
                            operator: Equal,
                            value: Quote(444444.0),
                            timeframe: Some(M1),
                        }
                    ]
                }
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_or_right_true() {
            assert_eq!(
                Or {
                    conditions: vec![
                        Compare {
                            field: Field::Volume,
                            operator: Equal,
                            value: Quote(33333.0),
                            timeframe: Some(S1),
                        },
                        Compare {
                            field: Field::Volume,
                            operator: Equal,
                            value: Quote(4.0),
                            timeframe: Some(M1),
                        }
                    ]
                }
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_or_both_false() {
            assert_eq!(
                Or {
                    conditions: vec![
                        Compare {
                            field: Field::Volume,
                            operator: Equal,
                            value: Quote(33333.0),
                            timeframe: Some(S1),
                        },
                        Compare {
                            field: Field::Volume,
                            operator: Equal,
                            value: Quote(44444.0),
                            timeframe: Some(M1),
                        }
                    ]
                }
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_not_both_true() {
            assert_eq!(
                AndNot {
                    conditions: vec![
                        Compare {
                            field: Field::Volume,
                            operator: Equal,
                            value: Quote(3.0),
                            timeframe: Some(S1),
                        },
                        Compare {
                            field: Field::Volume,
                            operator: Equal,
                            value: Quote(4.0),
                            timeframe: Some(M1),
                        }
                    ]
                }
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_not_left_true() {
            assert_eq!(
                AndNot {
                    conditions: vec![
                        Compare {
                            field: Field::Volume,
                            operator: Equal,
                            value: Quote(3.0),
                            timeframe: Some(S1),
                        },
                        Compare {
                            field: Field::Volume,
                            operator: Equal,
                            value: Quote(444444.0),
                            timeframe: Some(M1),
                        }
                    ]
                }
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_not_right_true() {
            assert_eq!(
                AndNot {
                    conditions: vec![
                        Compare {
                            field: Field::Volume,
                            operator: Equal,
                            value: Quote(33333.0),
                            timeframe: Some(S1),
                        },
                        Compare {
                            field: Field::Volume,
                            operator: Equal,
                            value: Quote(4.0),
                            timeframe: Some(M1),
                        }
                    ]
                }
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_not_both_false() {
            assert_eq!(
                AndNot {
                    conditions: vec![
                        Compare {
                            field: Field::Volume,
                            operator: Equal,
                            value: Quote(33333.0),
                            timeframe: Some(S1),
                        },
                        Compare {
                            field: Field::Volume,
                            operator: Equal,
                            value: Quote(44444.0),
                            timeframe: Some(M1),
                        }
                    ]
                }
                .test(&facts()),
                true
            )
        }
    }
}

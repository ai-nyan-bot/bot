// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3-or-later.

use crate::model::{compare, Condition, Fact, Facts};

impl Condition {
    pub fn test(&self, facts: &Facts) -> bool {
        match self {
            Condition::Compare {
                timeframe,
                operator,
                value,
                ..
            } => {
                // If the Compare condition has no value, it should never match
                let Some(value) = value else {
                    return false;
                };

                // Should always be Ok, as this is enforced during creation / update
                let Ok(fact) = Fact::try_from(self) else {
                    return false;
                };

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
            Condition::Compose { condition, .. } => condition.test(facts),
            Condition::And { conditions } => {
                let mut has_applicable_condition = false;

                for condition in conditions {
                    if condition.applicable() {
                        has_applicable_condition = true;
                        if !condition.test(facts) {
                            return false;
                        }
                    }
                }

                has_applicable_condition
            }
            Condition::Or { conditions } => {
                for condition in conditions {
                    if condition.applicable() {
                        if condition.test(facts) {
                            return true;
                        }
                    }
                }

                false
            }
            Condition::AndNot { conditions } => {
                let mut has_applicable_condition = false;

                for condition in conditions {
                    if condition.applicable() {
                        has_applicable_condition = true;
                        if condition.test(facts) {
                            return false;
                        }
                    }
                }

                has_applicable_condition
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Value;
    use common::model::Timeframe;
    use Fact::{PriceQuote, PriceUsd, VolumeChangeQuoteAggregate};
    use Timeframe::{M1, S1};

    fn facts() -> Facts {
        Facts::new()
            .with_value(PriceQuote, Value::quote(1))
            .with_value(PriceUsd, Value::usd(2))
            .with_timeframe_value(VolumeChangeQuoteAggregate, Value::quote(3), S1)
            .with_timeframe_value(VolumeChangeQuoteAggregate, Value::quote(4), M1)
    }

    mod compose {
        use crate::model::sequence::condition::test::tests::facts;
        use crate::model::Condition::{And, AndNot, Compare, Compose, Or};
        use crate::model::Field::Price;
        use crate::model::Operator::{LessThanEqual, MoreThanEqual};
        use crate::model::Value;

        #[test]
        fn test_true() {
            assert!(Compose {
                composition: "SomeComposition".into(),
                condition: Box::new(Compare {
                    field: Price,
                    operator: MoreThanEqual,
                    value: Value::quote(1).into(),
                    timeframe: None,
                }),
            }
            .test(&facts()))
        }

        #[test]
        fn test_false() {
            assert!(!Compose {
                composition: "SomeComposition".into(),
                condition: Box::new(Compare {
                    field: Price,
                    operator: MoreThanEqual,
                    value: Value::quote(22222).into(),
                    timeframe: None,
                }),
            }
            .test(&facts()))
        }

        #[test]
        fn test_no_value() {
            assert!(!Compose {
                composition: "SomeComposition".into(),
                condition: Box::new(Compare {
                    field: Price,
                    operator: MoreThanEqual,
                    value: None,
                    timeframe: None,
                }),
            }
            .test(&facts()))
        }

        #[test]
        fn test_and_multiple_no_value() {
            assert!(!Compose {
                composition: "SomeComposition".into(),
                condition: Box::new(And {
                    conditions: vec![
                        Compare {
                            field: Price,
                            operator: MoreThanEqual,
                            value: None,
                            timeframe: None,
                        },
                        Compare {
                            field: Price,
                            operator: LessThanEqual,
                            value: None,
                            timeframe: None,
                        }
                    ]
                }),
            }
            .test(&facts()))
        }

        #[test]
        fn test_or_multiple_no_value() {
            assert!(!Compose {
                composition: "SomeComposition".into(),
                condition: Box::new(Or {
                    conditions: vec![
                        Compare {
                            field: Price,
                            operator: MoreThanEqual,
                            value: None,
                            timeframe: None,
                        },
                        Compare {
                            field: Price,
                            operator: LessThanEqual,
                            value: None,
                            timeframe: None,
                        }
                    ]
                }),
            }
            .test(&facts()))
        }

        #[test]
        fn test_and_not_multiple_no_value() {
            assert!(!Compose {
                composition: "SomeComposition".into(),
                condition: Box::new(AndNot {
                    conditions: vec![
                        Compare {
                            field: Price,
                            operator: MoreThanEqual,
                            value: None,
                            timeframe: None,
                        },
                        Compare {
                            field: Price,
                            operator: LessThanEqual,
                            value: None,
                            timeframe: None,
                        }
                    ]
                }),
            }
            .test(&facts()))
        }

        #[test]
        fn test_and_partial() {
            assert!(Compose {
                composition: "SomeComposition".into(),
                condition: Box::new(And {
                    conditions: vec![
                        Compare {
                            field: Price,
                            operator: MoreThanEqual,
                            value: None,
                            timeframe: None,
                        },
                        Compare {
                            field: Price,
                            operator: MoreThanEqual,
                            value: Value::quote(1).into(),
                            timeframe: None,
                        }
                    ]
                }),
            }
            .test(&facts()))
        }

        #[test]
        fn test_or_partial() {
            assert!(Compose {
                composition: "SomeComposition".into(),
                condition: Box::new(Or {
                    conditions: vec![
                        Compare {
                            field: Price,
                            operator: MoreThanEqual,
                            value: None,
                            timeframe: None,
                        },
                        Compare {
                            field: Price,
                            operator: MoreThanEqual,
                            value: Value::quote(1).into(),
                            timeframe: None,
                        }
                    ]
                }),
            }
            .test(&facts()))
        }

        #[test]
        fn test_and_not_partial() {
            assert!(Compose {
                composition: "SomeComposition".into(),
                condition: Box::new(AndNot {
                    conditions: vec![
                        Compare {
                            field: Price,
                            operator: MoreThanEqual,
                            value: None,
                            timeframe: None,
                        },
                        Compare {
                            field: Price,
                            operator: MoreThanEqual,
                            value: Value::quote(2222).into(),
                            timeframe: None,
                        }
                    ]
                }),
            }
            .test(&facts()))
        }
    }

    mod compare {

        mod without_timeframe {
            use crate::model::sequence::condition::test::tests::facts;
            use crate::model::Field::Price;
            use crate::model::Operator::MoreThanEqual;
            use crate::model::{Condition, Value};
            use Condition::Compare;

            #[test]
            fn test_no_value() {
                assert!(!Compare {
                    field: Price,
                    operator: MoreThanEqual,
                    value: None,
                    timeframe: None,
                }
                .test(&facts()))
            }

            #[test]
            fn test_equal_true() {
                assert!(Compare {
                    field: Price,
                    operator: MoreThanEqual,
                    value: Some(Value::quote(1)),
                    timeframe: None,
                }
                .test(&facts()))
            }

            #[test]
            fn test_equal_false() {
                assert!(!Compare {
                    field: Price,
                    operator: MoreThanEqual,
                    value: Value::quote(1337).into(),
                    timeframe: None,
                }
                .test(&facts()))
            }
        }

        mod with_timeframe {
            use crate::model::sequence::condition::test::tests::facts;
            use crate::model::Field::Price;
            use crate::model::Operator::{IncreasedByMoreThanEqual, MoreThanEqual};
            use crate::model::{Condition, Field, Value};
            use common::model::Timeframe;
            use Condition::Compare;
            use Field::Volume;
            use Timeframe::{M1, S1};

            #[test]
            fn test_no_value() {
                assert!(!Compare {
                    field: Price,
                    operator: MoreThanEqual,
                    value: None,
                    timeframe: Some(M1),
                }
                .test(&facts()))
            }

            #[test]
            fn test_equal_true() {
                assert!(Compare {
                    field: Volume,
                    operator: IncreasedByMoreThanEqual,
                    value: Value::quote(3).into(),
                    timeframe: Some(S1),
                }
                .test(&facts()))
            }

            #[test]
            fn test_equal_false() {
                assert!(!Compare {
                    field: Volume,
                    operator: IncreasedByMoreThanEqual,
                    value: Value::quote(1337).into(),
                    timeframe: Some(M1),
                }
                .test(&facts()))
            }
        }
    }

    mod and {
        use crate::model::sequence::condition::test::tests::facts;
        use crate::model::Condition::{And, Compare};
        use crate::model::Field::{Price, Volume};
        use crate::model::Operator::{LessThanEqual, MoreThanEqual};
        use crate::model::Value;

        #[test]
        fn test_both_true() {
            assert!(And {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(1).into(),
                        timeframe: None,
                    },
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::usd(2).into(),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_left_true() {
            assert!(!And {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(1).into(),
                        timeframe: None
                    },
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::usd(22222).into(),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_right_true() {
            assert!(!And {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(111111).into(),
                        timeframe: None,
                    },
                    Compare {
                        field: Volume,
                        operator: MoreThanEqual,
                        value: Value::quote(2).into(),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_both_false() {
            assert!(!And {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(111111).into(),
                        timeframe: None,
                    },
                    Compare {
                        field: Volume,
                        operator: MoreThanEqual,
                        value: Value::quote(22222).into(),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_no_value() {
            assert!(!And {
                conditions: vec![Compare {
                    field: Price,
                    operator: MoreThanEqual,
                    value: None,
                    timeframe: None,
                },]
            }
            .test(&facts()))
        }

        #[test]
        fn test_multiple_no_value() {
            assert!(!And {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: None,
                        timeframe: None,
                    },
                    Compare {
                        field: Price,
                        operator: LessThanEqual,
                        value: None,
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_and_partial() {
            assert!(And {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: None,
                        timeframe: None,
                    },
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(1).into(),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }
    }

    mod or {
        use crate::model::sequence::condition::test::tests::facts;
        use crate::model::Condition::{Compare, Or};
        use crate::model::Field::{Price, Volume};
        use crate::model::Operator::{LessThanEqual, MoreThanEqual};
        use crate::model::Value;

        #[test]
        fn test_both_true() {
            assert!(Or {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(1).into(),
                        timeframe: None,
                    },
                    Compare {
                        field: Volume,
                        operator: MoreThanEqual,
                        value: Value::quote(2).into(),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_left_true() {
            assert!(Or {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(1).into(),
                        timeframe: None,
                    },
                    Compare {
                        field: Volume,
                        operator: MoreThanEqual,
                        value: Value::quote(22222).into(),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_right_true() {
            assert!(Or {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(111111).into(),
                        timeframe: None,
                    },
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::usd(2).into(),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_both_false() {
            assert!(!Or {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(111111).into(),
                        timeframe: None,
                    },
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::usd(22222).into(),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_no_value() {
            assert!(!Or {
                conditions: vec![Compare {
                    field: Price,
                    operator: MoreThanEqual,
                    value: None,
                    timeframe: None,
                },]
            }
            .test(&facts()))
        }

        #[test]
        fn test_multiple_no_value() {
            assert!(!Or {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: None,
                        timeframe: None,
                    },
                    Compare {
                        field: Price,
                        operator: LessThanEqual,
                        value: None,
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_and_partial() {
            assert!(Or {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: None,
                        timeframe: None,
                    },
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(1).into(),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }
    }

    mod and_not {
        use crate::model::sequence::condition::test::tests::facts;
        use crate::model::Condition::{AndNot, Compare};
        use crate::model::Field::{Price, Volume};
        use crate::model::Operator::{LessThanEqual, MoreThanEqual};
        use crate::model::Value;

        #[test]
        fn test_both_true() {
            assert!(!AndNot {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(1).into(),
                        timeframe: None,
                    },
                    Compare {
                        field: Volume,
                        operator: MoreThanEqual,
                        value: Value::quote(2).into(),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_left_true() {
            assert!(!AndNot {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(1).into(),
                        timeframe: None
                    },
                    Compare {
                        field: Volume,
                        operator: MoreThanEqual,
                        value: Value::quote(22222).into(),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_right_true() {
            assert!(!AndNot {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(111111).into(),
                        timeframe: None,
                    },
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::usd(2).into(),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_both_false() {
            assert!(AndNot {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(111111).into(),
                        timeframe: None,
                    },
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::usd(22222).into(),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_no_value() {
            assert!(!AndNot {
                conditions: vec![Compare {
                    field: Price,
                    operator: MoreThanEqual,
                    value: None,
                    timeframe: None,
                },]
            }
            .test(&facts()))
        }

        #[test]
        fn test_multiple_no_value() {
            assert!(!AndNot {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: None,
                        timeframe: None,
                    },
                    Compare {
                        field: Price,
                        operator: LessThanEqual,
                        value: None,
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_and_partial() {
            assert!(AndNot {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: None,
                        timeframe: None,
                    },
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(22222).into(),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }
    }
}

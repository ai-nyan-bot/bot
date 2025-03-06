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
    use crate::model::Value;
    use common::model::Timeframe;
    use Fact::{PriceQuote, PriceUsd, VolumeChangeQuote};
    use Timeframe::{M1, S1};

    fn facts() -> Facts {
        Facts::new()
            .with_value(PriceQuote, Value::quote(1))
            .with_value(PriceUsd, Value::usd(2))
            .with_timeframe_value(VolumeChangeQuote, Value::quote(3), S1)
            .with_timeframe_value(VolumeChangeQuote, Value::quote(4), M1)
    }

    mod without_timeframe {
        use crate::model::sequence::condition::test::tests::facts;
        use crate::model::Field::Price;
        use crate::model::Operator::MoreThanEqual;
        use crate::model::{Condition, Field, Value};
        use Condition::{And, AndNot, Compare, Compose, Or};
        use Field::Volume;

        #[test]
        fn test_equal_true() {
            assert!(Compare {
                field: Price,
                operator: MoreThanEqual,
                value: Value::quote(1),
                timeframe: None,
            }
            .test(&facts()))
        }

        #[test]
        fn test_equal_false() {
            assert!(!Compare {
                field: Price,
                operator: MoreThanEqual,
                value: Value::quote(1337),
                timeframe: None,
            }
            .test(&facts()))
        }

        #[test]
        fn test_compare_true() {
            assert!(Compose {
                ty: "SomeType".into(),
                condition: Box::new(Compare {
                    field: Price,
                    operator: MoreThanEqual,
                    value: Value::quote(1),
                    timeframe: None,
                }),
            }
            .test(&facts()))
        }

        #[test]
        fn test_compare_false() {
            assert!(!Compose {
                ty: "SomeType".into(),
                condition: Box::new(Compare {
                    field: Price,
                    operator: MoreThanEqual,
                    value: Value::quote(22222),
                    timeframe: None,
                }),
            }
            .test(&facts()))
        }

        #[test]
        fn test_and_both_true() {
            assert!(And {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(1),
                        timeframe: None,
                    },
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::usd(2),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_and_left_true() {
            assert!(!And {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(1),
                        timeframe: None
                    },
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::usd(22222),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_and_right_true() {
            assert!(!And {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(111111),
                        timeframe: None,
                    },
                    Compare {
                        field: Volume,
                        operator: MoreThanEqual,
                        value: Value::quote(2),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_and_both_false() {
            assert!(!And {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(111111),
                        timeframe: None,
                    },
                    Compare {
                        field: Volume,
                        operator: MoreThanEqual,
                        value: Value::quote(22222),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_or_both_true() {
            assert!(Or {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(1),
                        timeframe: None,
                    },
                    Compare {
                        field: Volume,
                        operator: MoreThanEqual,
                        value: Value::quote(2),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_or_left_true() {
            assert!(Or {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(1),
                        timeframe: None,
                    },
                    Compare {
                        field: Volume,
                        operator: MoreThanEqual,
                        value: Value::quote(22222),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_or_right_true() {
            assert!(Or {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(111111),
                        timeframe: None,
                    },
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::usd(2),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_or_both_false() {
            assert!(!Or {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(111111),
                        timeframe: None,
                    },
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::usd(22222),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_and_not_both_true() {
            assert!(!AndNot {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(1),
                        timeframe: None,
                    },
                    Compare {
                        field: Volume,
                        operator: MoreThanEqual,
                        value: Value::quote(2),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_and_not_left_true() {
            assert!(!AndNot {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(1),
                        timeframe: None
                    },
                    Compare {
                        field: Volume,
                        operator: MoreThanEqual,
                        value: Value::quote(22222),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_and_not_right_true() {
            assert!(!AndNot {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(111111),
                        timeframe: None,
                    },
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::usd(2),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_and_not_both_false() {
            assert!(AndNot {
                conditions: vec![
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::quote(111111),
                        timeframe: None,
                    },
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Value::usd(22222),
                        timeframe: None,
                    }
                ]
            }
            .test(&facts()))
        }
    }

    mod with_timeframe {
        use crate::model::sequence::condition::test::tests::facts;
        use crate::model::Condition::{And, AndNot, Compose, Or};
        use crate::model::Operator::IncreasedByMoreThanEqual;
        use crate::model::{Condition, Field, Value};
        use common::model::Timeframe;
        use Condition::Compare;
        use Field::Volume;
        use Timeframe::{M1, S1};

        #[test]
        fn test_equal_true() {
            assert!(Compare {
                field: Volume,
                operator: IncreasedByMoreThanEqual,
                value: Value::quote(3),
                timeframe: Some(S1),
            }
            .test(&facts()))
        }

        #[test]
        fn test_equal_false() {
            assert!(!Compare {
                field: Volume,
                operator: IncreasedByMoreThanEqual,
                value: Value::quote(1337),
                timeframe: Some(M1),
            }
            .test(&facts()))
        }

        #[test]
        fn test_compare_true() {
            assert!(Compose {
                ty: "SomeType".into(),
                condition: Box::new(Compare {
                    field: Volume,
                    operator: IncreasedByMoreThanEqual,
                    value: Value::quote(3),
                    timeframe: Some(S1),
                }),
            }
            .test(&facts()))
        }

        #[test]
        fn test_compare_false() {
            assert!(!Compose {
                ty: "SomeType".into(),
                condition: Box::new(Compare {
                    field: Volume,
                    operator: IncreasedByMoreThanEqual,
                    value: Value::quote(1337),
                    timeframe: Some(M1),
                }),
            }
            .test(&facts()))
        }

        #[test]
        fn test_and_both_true() {
            assert!(And {
                conditions: vec![
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Value::quote(3),
                        timeframe: Some(S1),
                    },
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Value::quote(4),
                        timeframe: Some(M1),
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_and_left_true() {
            assert!(!And {
                conditions: vec![
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Value::quote(3),
                        timeframe: Some(S1),
                    },
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Value::quote(444444),
                        timeframe: Some(M1),
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_and_right_true() {
            assert!(!And {
                conditions: vec![
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Value::quote(33333),
                        timeframe: Some(S1),
                    },
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Value::quote(4),
                        timeframe: Some(M1),
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_and_both_false() {
            assert!(!And {
                conditions: vec![
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Value::quote(33333),
                        timeframe: Some(S1),
                    },
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Value::quote(44444),
                        timeframe: Some(M1),
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_or_both_true() {
            assert!(Or {
                conditions: vec![
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Value::quote(3),
                        timeframe: Some(S1),
                    },
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Value::quote(4),
                        timeframe: Some(M1),
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_or_left_true() {
            assert!(Or {
                conditions: vec![
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Value::quote(3),
                        timeframe: Some(S1),
                    },
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Value::quote(444444),
                        timeframe: Some(M1),
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_or_right_true() {
            assert!(Or {
                conditions: vec![
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Value::quote(33333),
                        timeframe: Some(S1),
                    },
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Value::quote(4),
                        timeframe: Some(M1),
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_or_both_false() {
            assert!(!Or {
                conditions: vec![
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Value::quote(33333),
                        timeframe: Some(S1),
                    },
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Value::quote(44444),
                        timeframe: Some(M1),
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_and_not_both_true() {
            assert!(!AndNot {
                conditions: vec![
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Value::quote(3),
                        timeframe: Some(S1),
                    },
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Value::quote(4),
                        timeframe: Some(M1),
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_and_not_left_true() {
            assert!(!AndNot {
                conditions: vec![
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Value::quote(3),
                        timeframe: Some(S1),
                    },
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Value::quote(444444),
                        timeframe: Some(M1),
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_and_not_right_true() {
            assert!(!AndNot {
                conditions: vec![
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Value::quote(33333),
                        timeframe: Some(S1),
                    },
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Value::quote(4),
                        timeframe: Some(M1),
                    }
                ]
            }
            .test(&facts()))
        }

        #[test]
        fn test_and_not_both_false() {
            assert!(AndNot {
                conditions: vec![
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Value::quote(33333),
                        timeframe: Some(S1),
                    },
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Value::quote(44444),
                        timeframe: Some(M1),
                    }
                ]
            }
            .test(&facts()))
        }
    }
}

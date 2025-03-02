// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

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
    use Value::Quote;

    fn facts() -> Facts {
        Facts::new()
            .with_value(PriceQuote, Quote(1.0))
            .unwrap()
            .with_value(PriceUsd, Value::Usd(2.0))
            .unwrap()
            .with_timeframe_value(VolumeChangeQuote, Quote(3.0), S1)
            .unwrap()
            .with_timeframe_value(VolumeChangeQuote, Quote(4.0), M1)
            .unwrap()
    }

    mod without_timeframe {
        use crate::model::sequence::condition::test::tests::facts;
        use crate::model::Field::Price;
        use crate::model::Operator::MoreThanEqual;
        use crate::model::Value::Usd;
        use crate::model::{Condition, Field, Value};
        use Condition::{And, AndNot, Compare, Compose, Or};
        use Field::Volume;
        use Value::Quote;

        #[test]
        fn test_equal_true() {
            assert!(Compare {
                field: Price,
                operator: MoreThanEqual,
                value: Quote(1.0),
                timeframe: None,
            }
            .test(&facts()))
        }

        #[test]
        fn test_equal_false() {
            assert!(!Compare {
                field: Price,
                operator: MoreThanEqual,
                value: Quote(1337.0),
                timeframe: None,
            }
            .test(&facts()))
        }

        #[test]
        fn test_compare_true() {
            assert!(Compose {
                id: "Id1".into(),
                condition: Box::new(Compare {
                    field: Price,
                    operator: MoreThanEqual,
                    value: Quote(1.0),
                    timeframe: None,
                }),
            }
            .test(&facts()))
        }

        #[test]
        fn test_compare_false() {
            assert!(!Compose {
                id: "Id1".into(),
                condition: Box::new(Compare {
                    field: Price,
                    operator: MoreThanEqual,
                    value: Quote(22222.0),
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
                        value: Quote(1.0),
                        timeframe: None,
                    },
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Usd(2.0),
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
                        value: Quote(1.0),
                        timeframe: None
                    },
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Usd(22222.0),
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
                        value: Quote(111111.0),
                        timeframe: None,
                    },
                    Compare {
                        field: Volume,
                        operator: MoreThanEqual,
                        value: Quote(2.0),
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
                        value: Quote(111111.0),
                        timeframe: None,
                    },
                    Compare {
                        field: Volume,
                        operator: MoreThanEqual,
                        value: Quote(22222.0),
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
                        value: Quote(1.0),
                        timeframe: None,
                    },
                    Compare {
                        field: Volume,
                        operator: MoreThanEqual,
                        value: Quote(2.0),
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
                        value: Quote(1.0),
                        timeframe: None,
                    },
                    Compare {
                        field: Volume,
                        operator: MoreThanEqual,
                        value: Quote(22222.0),
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
                        value: Quote(111111.0),
                        timeframe: None,
                    },
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Usd(2.0),
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
                        value: Quote(111111.0),
                        timeframe: None,
                    },
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Usd(22222.0),
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
                        value: Quote(1.0),
                        timeframe: None,
                    },
                    Compare {
                        field: Volume,
                        operator: MoreThanEqual,
                        value: Quote(2.0),
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
                        value: Quote(1.0),
                        timeframe: None
                    },
                    Compare {
                        field: Volume,
                        operator: MoreThanEqual,
                        value: Quote(22222.0),
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
                        value: Quote(111111.0),
                        timeframe: None,
                    },
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Usd(2.0),
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
                        value: Quote(111111.0),
                        timeframe: None,
                    },
                    Compare {
                        field: Price,
                        operator: MoreThanEqual,
                        value: Usd(22222.0),
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
        use Value::Quote;

        #[test]
        fn test_equal_true() {
            assert!(Compare {
                field: Volume,
                operator: IncreasedByMoreThanEqual,
                value: Quote(3.0),
                timeframe: Some(S1),
            }
            .test(&facts()))
        }

        #[test]
        fn test_equal_false() {
            assert!(!Compare {
                field: Volume,
                operator: IncreasedByMoreThanEqual,
                value: Quote(1337.0),
                timeframe: Some(M1),
            }
            .test(&facts()))
        }

        #[test]
        fn test_compare_true() {
            assert!(Compose {
                id: "Id1".into(),
                condition: Box::new(Compare {
                    field: Volume,
                    operator: IncreasedByMoreThanEqual,
                    value: Quote(3.0),
                    timeframe: Some(S1),
                }),
            }
            .test(&facts()))
        }

        #[test]
        fn test_compare_false() {
            assert!(!Compose {
                id: "Id1".into(),
                condition: Box::new(Compare {
                    field: Volume,
                    operator: IncreasedByMoreThanEqual,
                    value: Quote(1337.0),
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
                        value: Quote(3.0),
                        timeframe: Some(S1),
                    },
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Quote(4.0),
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
                        value: Quote(3.0),
                        timeframe: Some(S1),
                    },
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Quote(444444.0),
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
                        value: Quote(33333.0),
                        timeframe: Some(S1),
                    },
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Quote(4.0),
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
                        value: Quote(33333.0),
                        timeframe: Some(S1),
                    },
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Quote(44444.0),
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
                        value: Quote(3.0),
                        timeframe: Some(S1),
                    },
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Quote(4.0),
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
                        value: Quote(3.0),
                        timeframe: Some(S1),
                    },
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Quote(444444.0),
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
                        value: Quote(33333.0),
                        timeframe: Some(S1),
                    },
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Quote(4.0),
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
                        value: Quote(33333.0),
                        timeframe: Some(S1),
                    },
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Quote(44444.0),
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
                        value: Quote(3.0),
                        timeframe: Some(S1),
                    },
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Quote(4.0),
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
                        value: Quote(3.0),
                        timeframe: Some(S1),
                    },
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Quote(444444.0),
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
                        value: Quote(33333.0),
                        timeframe: Some(S1),
                    },
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Quote(4.0),
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
                        value: Quote(33333.0),
                        timeframe: Some(S1),
                    },
                    Compare {
                        field: Volume,
                        operator: IncreasedByMoreThanEqual,
                        value: Quote(44444.0),
                        timeframe: Some(M1),
                    }
                ]
            }
            .test(&facts()))
        }
    }
}

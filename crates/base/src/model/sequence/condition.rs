// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{compare, Fact, Facts, Operator, Value};
use common::model::Timeframe;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Condition {
    Compare {
        fact: Fact,
        operator: Operator,
        value: Value,
        timeframe: Option<Timeframe>,
    },
    Exists {
        fact: Fact,
        timeframe: Option<Timeframe>,
    },
    And(Box<[Condition]>),
    Or(Box<[Condition]>),
    AndNot(Box<[Condition]>),
}

impl Condition {
    pub fn test(&self, facts: &Facts) -> bool {
        match self {
            Condition::Compare {
                fact,
                operator,
                value,
                timeframe,
            } => {
                let result = match timeframe {
                    None => facts.get(fact),
                    Some(timeframe) => facts.get_with_timeframe(fact, timeframe),
                };

                if let Some(fact_value) = result {
                    compare(fact_value, operator, value)
                } else {
                    false
                }
            }
            Condition::Exists { fact, timeframe } => match timeframe {
                None => facts.get(fact),
                Some(timeframe) => facts.get_with_timeframe(fact, timeframe),
            }
            .is_some(),
            Condition::And(conditions) => {
                for condition in conditions {
                    if !condition.test(facts) {
                        return false;
                    }
                }
                true
            }
            Condition::Or(conditions) => {
                for condition in conditions {
                    if condition.test(facts) {
                        return true;
                    }
                }
                false
            }
            Condition::AndNot(conditions) => {
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
            .with_value(Fact::TokenPriceQuote, Value::Number(1.0))
            .unwrap()
            .with_value(Fact::TokenTotalVolumeQuote, Value::Number(2.0))
            .unwrap()
            .with_timeframe_value(Fact::TokenVolumeQuote, Value::Number(3.0), Timeframe::S1)
            .unwrap()
            .with_timeframe_value(Fact::TokenVolumeQuote, Value::Number(4.0), Timeframe::M1)
            .unwrap()
    }

    mod without_timeframe {
        use crate::model::sequence::condition::tests::facts;
        use crate::model::Condition::Exists;
        use crate::model::Fact::TokenTotalVolumeQuote;
        use crate::model::{Condition, Fact, Operator, Value};
        use Condition::{And, AndNot, Compare, Or};
        use Fact::{TokenCreationDuration, TokenPriceQuote};
        use Operator::Equal;
        use Value::Number;

        #[test]
        fn test_equal_true() {
            assert_eq!(
                Compare {
                    fact: TokenPriceQuote,
                    operator: Equal,
                    value: Number(1.0),
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
                    fact: TokenPriceQuote,
                    operator: Equal,
                    value: Number(1337.0),
                    timeframe: None,
                }
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_exists() {
            assert_eq!(
                Exists {
                    fact: TokenPriceQuote,
                    timeframe: None,
                }
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_exists_false() {
            assert_eq!(
                Exists {
                    fact: TokenCreationDuration,
                    timeframe: None,
                }
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_both_true() {
            assert_eq!(
                And(Box::new([
                    Compare {
                        fact: TokenPriceQuote,
                        operator: Equal,
                        value: Number(1.0),
                        timeframe: None,
                    },
                    Compare {
                        fact: TokenTotalVolumeQuote,
                        operator: Equal,
                        value: Number(2.0),
                        timeframe: None,
                    }
                ]))
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_and_left_true() {
            assert_eq!(
                And(Box::new([
                    Compare {
                        fact: TokenPriceQuote,
                        operator: Equal,
                        value: Number(1.0),
                        timeframe: None
                    },
                    Compare {
                        fact: TokenTotalVolumeQuote,
                        operator: Equal,
                        value: Number(22222.0),
                        timeframe: None,
                    }
                ]))
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_right_true() {
            assert_eq!(
                And(Box::new([
                    Compare {
                        fact: TokenPriceQuote,
                        operator: Equal,
                        value: Number(111111.0),
                        timeframe: None,
                    },
                    Compare {
                        fact: TokenTotalVolumeQuote,
                        operator: Equal,
                        value: Number(2.0),
                        timeframe: None,
                    }
                ]))
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_both_false() {
            assert_eq!(
                And(Box::new([
                    Compare {
                        fact: TokenPriceQuote,
                        operator: Equal,
                        value: Number(111111.0),
                        timeframe: None,
                    },
                    Compare {
                        fact: TokenTotalVolumeQuote,
                        operator: Equal,
                        value: Number(22222.0),
                        timeframe: None,
                    }
                ]))
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_or_both_true() {
            assert_eq!(
                Or(Box::new([
                    Compare {
                        fact: TokenPriceQuote,
                        operator: Equal,
                        value: Number(1.0),
                        timeframe: None,
                    },
                    Compare {
                        fact: TokenTotalVolumeQuote,
                        operator: Equal,
                        value: Number(2.0),
                        timeframe: None,
                    }
                ]))
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_or_left_true() {
            assert_eq!(
                Or(Box::new([
                    Compare {
                        fact: TokenPriceQuote,
                        operator: Equal,
                        value: Number(1.0),
                        timeframe: None,
                    },
                    Compare {
                        fact: TokenTotalVolumeQuote,
                        operator: Equal,
                        value: Number(22222.0),
                        timeframe: None,
                    }
                ]))
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_or_right_true() {
            assert_eq!(
                Or(Box::new([
                    Compare {
                        fact: TokenPriceQuote,
                        operator: Equal,
                        value: Number(111111.0),
                        timeframe: None,
                    },
                    Compare {
                        fact: TokenTotalVolumeQuote,
                        operator: Equal,
                        value: Number(2.0),
                        timeframe: None,
                    }
                ]))
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_or_both_false() {
            assert_eq!(
                Or(Box::new([
                    Compare {
                        fact: TokenPriceQuote,
                        operator: Equal,
                        value: Number(111111.0),
                        timeframe: None,
                    },
                    Compare {
                        fact: TokenTotalVolumeQuote,
                        operator: Equal,
                        value: Number(22222.0),
                        timeframe: None,
                    }
                ]))
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_not_both_true() {
            assert_eq!(
                AndNot(Box::new([
                    Compare {
                        fact: TokenPriceQuote,
                        operator: Equal,
                        value: Number(1.0),
                        timeframe: None,
                    },
                    Compare {
                        fact: TokenTotalVolumeQuote,
                        operator: Equal,
                        value: Number(2.0),
                        timeframe: None,
                    }
                ]))
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_not_left_true() {
            assert_eq!(
                AndNot(Box::new([
                    Compare {
                        fact: TokenPriceQuote,
                        operator: Equal,
                        value: Number(1.0),
                        timeframe: None
                    },
                    Compare {
                        fact: TokenTotalVolumeQuote,
                        operator: Equal,
                        value: Number(22222.0),
                        timeframe: None,
                    }
                ]))
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_not_right_true() {
            assert_eq!(
                AndNot(Box::new([
                    Compare {
                        fact: TokenPriceQuote,
                        operator: Equal,
                        value: Number(111111.0),
                        timeframe: None,
                    },
                    Compare {
                        fact: TokenTotalVolumeQuote,
                        operator: Equal,
                        value: Number(2.0),
                        timeframe: None,
                    }
                ]))
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_not_both_false() {
            assert_eq!(
                AndNot(Box::new([
                    Compare {
                        fact: TokenPriceQuote,
                        operator: Equal,
                        value: Number(111111.0),
                        timeframe: None,
                    },
                    Compare {
                        fact: TokenTotalVolumeQuote,
                        operator: Equal,
                        value: Number(22222.0),
                        timeframe: None,
                    }
                ]))
                .test(&facts()),
                true
            )
        }
    }

    mod with_timeframe {
        use crate::model::sequence::condition::tests::facts;
        use crate::model::Condition::{And, AndNot, Exists, Or};
        use crate::model::Fact::TokenVolumeQuote;
        use crate::model::{Condition, Operator, Value};
        use common::model::Timeframe;
        use common::model::Timeframe::D1;
        use Condition::Compare;
        use Operator::Equal;
        use Timeframe::{M1, S1};
        use Value::Number;

        #[test]
        fn test_equal_true() {
            assert_eq!(
                Compare {
                    fact: TokenVolumeQuote,
                    operator: Equal,
                    value: Number(3.0),
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
                    fact: TokenVolumeQuote,
                    operator: Equal,
                    value: Number(1337.0),
                    timeframe: Some(M1),
                }
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_exists() {
            assert_eq!(
                Exists {
                    fact: TokenVolumeQuote,
                    timeframe: Some(S1),
                }
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_exists_false() {
            assert_eq!(
                Exists {
                    fact: TokenVolumeQuote,
                    timeframe: Some(D1),
                }
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_both_true() {
            assert_eq!(
                And(Box::new([
                    Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(3.0),
                        timeframe: Some(S1),
                    },
                    Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(4.0),
                        timeframe: Some(M1),
                    }
                ]))
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_and_left_true() {
            assert_eq!(
                And(Box::new([
                    Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(3.0),
                        timeframe: Some(S1),
                    },
                    Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(444444.0),
                        timeframe: Some(M1),
                    }
                ]))
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_right_true() {
            assert_eq!(
                And(Box::new([
                    Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(33333.0),
                        timeframe: Some(S1),
                    },
                    Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(4.0),
                        timeframe: Some(M1),
                    }
                ]))
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_both_false() {
            assert_eq!(
                And(Box::new([
                    Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(33333.0),
                        timeframe: Some(S1),
                    },
                    Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(44444.0),
                        timeframe: Some(M1),
                    }
                ]))
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_or_both_true() {
            assert_eq!(
                Or(Box::new([
                    Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(3.0),
                        timeframe: Some(S1),
                    },
                    Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(4.0),
                        timeframe: Some(M1),
                    }
                ]))
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_or_left_true() {
            assert_eq!(
                Or(Box::new([
                    Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(3.0),
                        timeframe: Some(S1),
                    },
                    Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(444444.0),
                        timeframe: Some(M1),
                    }
                ]))
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_or_right_true() {
            assert_eq!(
                Or(Box::new([
                    Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(33333.0),
                        timeframe: Some(S1),
                    },
                    Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(4.0),
                        timeframe: Some(M1),
                    }
                ]))
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_or_both_false() {
            assert_eq!(
                Or(Box::new([
                    Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(33333.0),
                        timeframe: Some(S1),
                    },
                    Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(44444.0),
                        timeframe: Some(M1),
                    }
                ]))
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_not_both_true() {
            assert_eq!(
                AndNot(Box::new([
                    Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(3.0),
                        timeframe: Some(S1),
                    },
                    Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(4.0),
                        timeframe: Some(M1),
                    }
                ]))
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_not_left_true() {
            assert_eq!(
                AndNot(Box::new([
                    Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(3.0),
                        timeframe: Some(S1),
                    },
                    Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(444444.0),
                        timeframe: Some(M1),
                    }
                ]))
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_not_right_true() {
            assert_eq!(
                AndNot(Box::new([
                    Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(33333.0),
                        timeframe: Some(S1),
                    },
                    Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(4.0),
                        timeframe: Some(M1),
                    }
                ]))
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_not_both_false() {
            assert_eq!(
                AndNot(Box::new([
                    Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(33333.0),
                        timeframe: Some(S1),
                    },
                    Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(44444.0),
                        timeframe: Some(M1),
                    }
                ]))
                .test(&facts()),
                true
            )
        }
    }
}

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
    // FIXME replace Exists operator with Exists condition
    // Exists {
    //     fact: Fact,
    //     timeframe: Option<Timeframe>,
    // },
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Not(Box<Condition>),
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

                if *operator == Operator::Exists {
                    return result.is_some();
                }

                if let Some(fact_value) = result {
                    compare(fact_value, operator, value)
                } else {
                    false
                }
            }
            Condition::And(left, right) => left.test(facts) && right.test(facts),
            Condition::Or(left, right) => left.test(facts) || right.test(facts),
            Condition::Not(inner) => !inner.test(facts),
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
        use crate::model::Condition::Not;
        use crate::model::Fact::TokenTotalVolumeQuote;
        use crate::model::{Condition, Fact, Operator, Value};
        use std::time::Duration;
        use Condition::{And, Compare, Or};
        use Fact::TokenPriceQuote;
        use Operator::{Equal, Exists};
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
                Compare {
                    fact: TokenPriceQuote,
                    operator: Exists,
                    value: Number(0.0),
                    timeframe: None,
                }
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_exists_false() {
            assert_eq!(
                Compare {
                    fact: Fact::TokenCreationDuration,
                    operator: Exists,
                    value: Value::Duration(Duration::from_secs(23)),
                    timeframe: None,
                }
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_both_true() {
            assert_eq!(
                And(
                    Box::new(Compare {
                        fact: TokenPriceQuote,
                        operator: Equal,
                        value: Number(1.0),
                        timeframe: None,
                    }),
                    Box::new(Compare {
                        fact: TokenTotalVolumeQuote,
                        operator: Equal,
                        value: Number(2.0),
                        timeframe: None,
                    })
                )
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_and_left_true() {
            assert_eq!(
                And(
                    Box::new(Compare {
                        fact: TokenPriceQuote,
                        operator: Equal,
                        value: Number(1.0),
                        timeframe: None,
                    }),
                    Box::new(Compare {
                        fact: TokenTotalVolumeQuote,
                        operator: Equal,
                        value: Number(22222.0),
                        timeframe: None,
                    })
                )
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_right_true() {
            assert_eq!(
                And(
                    Box::new(Compare {
                        fact: TokenPriceQuote,
                        operator: Equal,
                        value: Number(111111.0),
                        timeframe: None,
                    }),
                    Box::new(Compare {
                        fact: TokenTotalVolumeQuote,
                        operator: Equal,
                        value: Number(2.0),
                        timeframe: None,
                    })
                )
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_both_false() {
            assert_eq!(
                And(
                    Box::new(Compare {
                        fact: TokenPriceQuote,
                        operator: Equal,
                        value: Number(111111.0),
                        timeframe: None,
                    }),
                    Box::new(Compare {
                        fact: TokenTotalVolumeQuote,
                        operator: Equal,
                        value: Number(22222.0),
                        timeframe: None,
                    })
                )
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_or_both_true() {
            assert_eq!(
                Or(
                    Box::new(Compare {
                        fact: TokenPriceQuote,
                        operator: Equal,
                        value: Number(1.0),
                        timeframe: None,
                    }),
                    Box::new(Compare {
                        fact: TokenTotalVolumeQuote,
                        operator: Equal,
                        value: Number(2.0),
                        timeframe: None,
                    })
                )
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_or_left_true() {
            assert_eq!(
                Or(
                    Box::new(Compare {
                        fact: TokenPriceQuote,
                        operator: Equal,
                        value: Number(1.0),
                        timeframe: None,
                    }),
                    Box::new(Compare {
                        fact: TokenTotalVolumeQuote,
                        operator: Equal,
                        value: Number(22222.0),
                        timeframe: None,
                    })
                )
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_or_right_true() {
            assert_eq!(
                Or(
                    Box::new(Compare {
                        fact: TokenPriceQuote,
                        operator: Equal,
                        value: Number(111111.0),
                        timeframe: None,
                    }),
                    Box::new(Compare {
                        fact: TokenTotalVolumeQuote,
                        operator: Equal,
                        value: Number(2.0),
                        timeframe: None,
                    })
                )
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_or_both_false() {
            assert_eq!(
                Or(
                    Box::new(Compare {
                        fact: TokenPriceQuote,
                        operator: Equal,
                        value: Number(111111.0),
                        timeframe: None,
                    }),
                    Box::new(Compare {
                        fact: TokenTotalVolumeQuote,
                        operator: Equal,
                        value: Number(22222.0),
                        timeframe: None,
                    })
                )
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_not_true() {
            assert_eq!(
                Not(Box::new(Compare {
                    fact: TokenPriceQuote,
                    operator: Equal,
                    value: Number(11111.0),
                    timeframe: None,
                }),)
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_not_false() {
            assert_eq!(
                Not(Box::new(Compare {
                    fact: TokenPriceQuote,
                    operator: Equal,
                    value: Number(1.0),
                    timeframe: None,
                }),)
                .test(&facts()),
                false
            )
        }
    }

    mod with_timeframe {
        use crate::model::sequence::condition::tests::facts;
        use crate::model::Condition::{And, Not, Or};
        use crate::model::Fact::TokenVolumeQuote;
        use crate::model::{Condition, Operator, Value};
        use common::model::Timeframe;
        use common::model::Timeframe::D1;
        use std::time::Duration;
        use Condition::Compare;
        use Operator::{Equal, Exists};
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
                Compare {
                    fact: TokenVolumeQuote,
                    operator: Exists,
                    value: Number(0.0),
                    timeframe: Some(S1),
                }
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_exists_false() {
            assert_eq!(
                Compare {
                    fact: TokenVolumeQuote,
                    operator: Exists,
                    value: Value::Duration(Duration::from_secs(23)),
                    timeframe: Some(D1),
                }
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_both_true() {
            assert_eq!(
                And(
                    Box::new(Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(3.0),
                        timeframe: Some(S1),
                    }),
                    Box::new(Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(4.0),
                        timeframe: Some(M1),
                    })
                )
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_and_left_true() {
            assert_eq!(
                And(
                    Box::new(Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(3.0),
                        timeframe: Some(S1),
                    }),
                    Box::new(Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(444444.0),
                        timeframe: Some(M1),
                    })
                )
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_right_true() {
            assert_eq!(
                And(
                    Box::new(Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(33333.0),
                        timeframe: Some(S1),
                    }),
                    Box::new(Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(4.0),
                        timeframe: Some(M1),
                    })
                )
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_and_both_false() {
            assert_eq!(
                And(
                    Box::new(Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(33333.0),
                        timeframe: Some(S1),
                    }),
                    Box::new(Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(44444.0),
                        timeframe: Some(M1),
                    })
                )
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_or_both_true() {
            assert_eq!(
                Or(
                    Box::new(Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(3.0),
                        timeframe: Some(S1),
                    }),
                    Box::new(Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(4.0),
                        timeframe: Some(M1),
                    })
                )
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_or_left_true() {
            assert_eq!(
                Or(
                    Box::new(Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(3.0),
                        timeframe: Some(S1),
                    }),
                    Box::new(Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(444444.0),
                        timeframe: Some(M1),
                    })
                )
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_or_right_true() {
            assert_eq!(
                Or(
                    Box::new(Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(33333.0),
                        timeframe: Some(S1),
                    }),
                    Box::new(Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(4.0),
                        timeframe: Some(M1),
                    })
                )
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_or_both_false() {
            assert_eq!(
                Or(
                    Box::new(Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(33333.0),
                        timeframe: Some(S1),
                    }),
                    Box::new(Compare {
                        fact: TokenVolumeQuote,
                        operator: Equal,
                        value: Number(44444.0),
                        timeframe: Some(M1),
                    })
                )
                .test(&facts()),
                false
            )
        }

        #[test]
        fn test_not_true() {
            assert_eq!(
                Not(Box::new(Compare {
                    fact: TokenVolumeQuote,
                    operator: Equal,
                    value: Number(33333.0),
                    timeframe: Some(S1),
                }),)
                .test(&facts()),
                true
            )
        }

        #[test]
        fn test_not_false() {
            assert_eq!(
                Not(Box::new(Compare {
                    fact: TokenVolumeQuote,
                    operator: Equal,
                    value: Number(3.0),
                    timeframe: Some(S1),
                }),)
                .test(&facts()),
                false
            )
        }
    }
}

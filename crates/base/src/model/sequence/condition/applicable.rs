// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Condition, Fact};

impl Condition {
    /// Determines whether a condition can be applied.
    /// E.g. the user provides an empty AND condition, which would match everything
    pub fn applicable(&self) -> bool {
        match self {
            Condition::Compare { value, .. } => {
                if let Some(_) = value {
                    Fact::try_from(self).is_ok()
                } else {
                    false
                }
            }
            Condition::Compose { condition, .. } => condition.applicable(),
            Condition::And { conditions }
            | Condition::Or { conditions }
            | Condition::AndNot { conditions } => {
                !conditions.is_empty() && conditions.iter().any(|c| c.applicable())
            }
        }
    }
}

#[cfg(test)]
mod tests {

    mod compose {
        use crate::model::Condition::Compare;
        use crate::model::Field::TwitterAccountHandle;
        use crate::model::Operator::{Equal, NotEqual};
        use crate::model::{Condition, Value};

        #[test]
        fn test_applicable() {
            let test_instance = Condition::Compose {
                composition: "SomeComposition".into(),
                condition: Box::new(Compare {
                    field: TwitterAccountHandle,
                    operator: Equal,
                    value: Value::string("AI_nyanbot").into(),
                    timeframe: None,
                }),
            };
            assert!(test_instance.applicable())
        }

        #[test]
        fn test_not_applicable() {
            let test_instance = Condition::Compose {
                composition: "SomeComposition".into(),
                condition: Box::new(Compare {
                    field: TwitterAccountHandle,
                    operator: NotEqual,
                    value: Value::string("AI_nyanbot").into(),
                    timeframe: None,
                }),
            };
            assert!(!test_instance.applicable())
        }

        #[test]
        fn test_no_value() {
            let test_instance = Condition::Compose {
                composition: "SomeComposition".into(),
                condition: Box::new(Compare {
                    field: TwitterAccountHandle,
                    operator: Equal,
                    value: None,
                    timeframe: None,
                }),
            };
            assert!(!test_instance.applicable())
        }
    }

    mod and {
        use crate::model::Condition::{And, Compare};
        use crate::model::Field::{SwapBuy, SwapTotal, TwitterAccountHandle};
        use crate::model::Operator::{Equal, LessThanEqual, MoreThan, MoreThanEqual, NotEqual};
        use crate::model::{Condition, Value};
        use common::model::Timeframe::{H1, M15};

        #[test]
        fn test_empty() {
            let test_instance = Condition::And { conditions: vec![] };
            assert!(!test_instance.applicable())
        }

        #[test]
        fn test_single_child_applicable() {
            let test_instance = Condition::And {
                conditions: vec![Compare {
                    field: TwitterAccountHandle,
                    operator: Equal,
                    value: Value::string("AI_nyanbot").into(),
                    timeframe: None,
                }],
            };
            assert!(test_instance.applicable())
        }

        #[test]
        fn test_no_value() {
            // nothing is applicable
            let test_instance = And {
                conditions: vec![Compare {
                    field: TwitterAccountHandle,
                    operator: Equal,
                    value: None,
                    timeframe: None,
                }],
            };
            assert!(!test_instance.applicable())
        }

        #[test]
        fn test_multiple_no_value() {
            // nothing is applicable
            let test_instance = Condition::Compose {
                composition: "SomeComposition".into(),
                condition: Box::new(And {
                    conditions: vec![
                        Compare {
                            field: SwapTotal,
                            operator: MoreThanEqual,
                            value: None,
                            timeframe: H1.into(),
                        },
                        Compare {
                            field: SwapTotal,
                            operator: LessThanEqual,
                            value: None,
                            timeframe: H1.into(),
                        },
                    ],
                }),
            };
            assert!(!test_instance.applicable())
        }

        #[test]
        fn test_value_and_no_value() {
            // compare 2 is applicable
            let test_instance = Condition::Compose {
                composition: "SomeComposition".into(),
                condition: Box::new(And {
                    conditions: vec![
                        Compare {
                            field: SwapTotal,
                            operator: MoreThanEqual,
                            value: None,
                            timeframe: H1.into(),
                        },
                        Compare {
                            field: SwapTotal,
                            operator: LessThanEqual,
                            value: Value::count(23).into(),
                            timeframe: H1.into(),
                        },
                    ],
                }),
            };
            assert!(test_instance.applicable())
        }

        #[test]
        fn test_multiple_value_and_no_value() {
            // compare 2 and 3 are applicable
            let test_instance = Condition::Compose {
                composition: "SomeComposition".into(),
                condition: Box::new(And {
                    conditions: vec![
                        Compare {
                            field: SwapTotal,
                            operator: MoreThanEqual,
                            value: None,
                            timeframe: H1.into(),
                        },
                        Compare {
                            field: SwapTotal,
                            operator: LessThanEqual,
                            value: Value::count(23).into(),
                            timeframe: H1.into(),
                        },
                        Compare {
                            field: SwapTotal,
                            operator: MoreThanEqual,
                            value: Value::count(42).into(),
                            timeframe: H1.into(),
                        },
                    ],
                }),
            };
            assert!(test_instance.applicable())
        }

        #[test]
        fn test_single_child_not_applicable() {
            let test_instance = Condition::And {
                conditions: vec![Compare {
                    field: TwitterAccountHandle,
                    operator: NotEqual,
                    value: Value::string("AI_nyanbot").into(),
                    timeframe: None,
                }],
            };
            assert!(!test_instance.applicable())
        }

        #[test]
        fn test_children_applicable() {
            let test_instance = Condition::And {
                conditions: vec![
                    Compare {
                        field: SwapBuy,
                        operator: MoreThan,
                        value: Value::count(1).into(),
                        timeframe: Some(M15),
                    },
                    Compare {
                        field: TwitterAccountHandle,
                        operator: Equal,
                        value: Value::string("AI_nyanbot").into(),
                        timeframe: None,
                    },
                ],
            };
            assert!(test_instance.applicable())
        }

        #[test]
        fn test_children_partial_applicable() {
            let test_instance = Condition::And {
                conditions: vec![
                    // applicable
                    Compare {
                        field: SwapBuy,
                        operator: MoreThan,
                        value: Value::percent(1.0).into(),
                        timeframe: Some(M15),
                    },
                    // Not applicable
                    Compare {
                        field: TwitterAccountHandle,
                        operator: NotEqual,
                        value: Value::string("AI_nyanbot").into(),
                        timeframe: None,
                    },
                ],
            };
            assert!(!test_instance.applicable())
        }

        #[test]
        fn test_children_not_applicable() {
            let test_instance = Condition::And {
                conditions: vec![
                    Condition::And { conditions: vec![] },
                    Condition::Or { conditions: vec![] },
                    Condition::AndNot { conditions: vec![] },
                ],
            };
            assert!(!test_instance.applicable())
        }
    }

    mod or {
        use crate::model::Condition::Compare;
        use crate::model::Field::{SwapBuy, SwapTotal, TwitterAccountHandle};
        use crate::model::Operator::{
            Equal, IncreasedByMoreThan, LessThanEqual, MoreThan, MoreThanEqual, NotEqual,
        };
        use crate::model::{Condition, Value};
        use common::model::Timeframe::{H1, M15};
        use Condition::Or;

        #[test]
        fn test_empty() {
            let test_instance = Or { conditions: vec![] };
            assert!(!test_instance.applicable())
        }

        #[test]
        fn test_single_child_applicable() {
            let test_instance = Or {
                conditions: vec![Compare {
                    field: TwitterAccountHandle,
                    operator: Equal,
                    value: Value::string("AI_nyanbot").into(),
                    timeframe: None,
                }],
            };
            assert!(test_instance.applicable())
        }

        #[test]
        fn test_no_value() {
            // nothing is applicable
            let test_instance = Or {
                conditions: vec![Compare {
                    field: TwitterAccountHandle,
                    operator: Equal,
                    value: None,
                    timeframe: None,
                }],
            };
            assert!(!test_instance.applicable())
        }

        #[test]
        fn test_multiple_no_value() {
            // nothing is applicable
            let test_instance = Condition::Compose {
                composition: "SomeComposition".into(),
                condition: Box::new(Or {
                    conditions: vec![
                        Compare {
                            field: SwapTotal,
                            operator: MoreThanEqual,
                            value: None,
                            timeframe: H1.into(),
                        },
                        Compare {
                            field: SwapTotal,
                            operator: LessThanEqual,
                            value: None,
                            timeframe: H1.into(),
                        },
                    ],
                }),
            };
            assert!(!test_instance.applicable())
        }

        #[test]
        fn test_value_and_no_value() {
            // compare 2 is applicable
            let test_instance = Condition::Compose {
                composition: "SomeComposition".into(),
                condition: Box::new(Or {
                    conditions: vec![
                        Compare {
                            field: SwapTotal,
                            operator: MoreThanEqual,
                            value: None,
                            timeframe: H1.into(),
                        },
                        Compare {
                            field: SwapTotal,
                            operator: LessThanEqual,
                            value: Value::count(23).into(),
                            timeframe: H1.into(),
                        },
                    ],
                }),
            };
            assert!(test_instance.applicable())
        }

        #[test]
        fn test_multiple_value_and_no_value() {
            // compare 2 Or 3 are applicable
            let test_instance = Condition::Compose {
                composition: "SomeComposition".into(),
                condition: Box::new(Or {
                    conditions: vec![
                        Compare {
                            field: SwapTotal,
                            operator: MoreThanEqual,
                            value: None,
                            timeframe: H1.into(),
                        },
                        Compare {
                            field: SwapTotal,
                            operator: LessThanEqual,
                            value: Value::count(23).into(),
                            timeframe: H1.into(),
                        },
                        Compare {
                            field: SwapTotal,
                            operator: MoreThanEqual,
                            value: Value::count(42).into(),
                            timeframe: H1.into(),
                        },
                    ],
                }),
            };
            assert!(test_instance.applicable())
        }

        #[test]
        fn test_single_child_not_applicable() {
            let test_instance = Or {
                conditions: vec![Compare {
                    field: TwitterAccountHandle,
                    operator: NotEqual,
                    value: Value::string("AI_nyanbot").into(),
                    timeframe: None,
                }],
            };
            assert!(!test_instance.applicable())
        }

        #[test]
        fn test_children_applicable() {
            let test_instance = Or {
                conditions: vec![
                    Compare {
                        field: SwapBuy,
                        operator: IncreasedByMoreThan,
                        value: Value::percent(1.0).into(),
                        timeframe: Some(M15),
                    },
                    Compare {
                        field: TwitterAccountHandle,
                        operator: Equal,
                        value: Value::string("AI_nyanbot").into(),
                        timeframe: None,
                    },
                ],
            };
            assert!(test_instance.applicable())
        }

        #[test]
        fn test_children_partial_applicable() {
            let test_instance = Or {
                conditions: vec![
                    // applicable
                    Compare {
                        field: SwapBuy,
                        operator: MoreThan,
                        value: Value::percent(1.0).into(),
                        timeframe: Some(M15),
                    },
                    // Not applicable
                    Compare {
                        field: TwitterAccountHandle,
                        operator: NotEqual,
                        value: Value::string("AI_nyanbot").into(),
                        timeframe: None,
                    },
                ],
            };
            assert!(!test_instance.applicable())
        }

        #[test]
        fn test_children_not_applicable() {
            let test_instance = Or {
                conditions: vec![
                    Condition::And { conditions: vec![] },
                    Or { conditions: vec![] },
                    Condition::AndNot { conditions: vec![] },
                ],
            };
            assert!(!test_instance.applicable())
        }
    }

    mod and_not {
        use crate::model::Condition::{And, Compare, Or};
        use crate::model::Field::{SwapBuy, SwapTotal, TwitterAccountHandle};
        use crate::model::Operator::{
            Equal, IncreasedByMoreThan, LessThanEqual, MoreThan, MoreThanEqual, NotEqual,
        };
        use crate::model::{Condition, Value};
        use common::model::Timeframe::{H1, M15};
        use Condition::AndNot;

        #[test]
        fn test_empty() {
            let test_instance = AndNot { conditions: vec![] };
            assert!(!test_instance.applicable())
        }

        #[test]
        fn test_single_child_applicable() {
            let test_instance = AndNot {
                conditions: vec![Compare {
                    field: TwitterAccountHandle,
                    operator: Equal,
                    value: Value::string("AI_nyanbot").into(),
                    timeframe: None,
                }],
            };
            assert!(test_instance.applicable())
        }

        #[test]
        fn test_no_value() {
            // nothing is applicable
            let test_instance = AndNot {
                conditions: vec![Compare {
                    field: TwitterAccountHandle,
                    operator: Equal,
                    value: None,
                    timeframe: None,
                }],
            };
            assert!(!test_instance.applicable())
        }

        #[test]
        fn test_multiple_no_value() {
            // nothing is applicable
            let test_instance = Condition::Compose {
                composition: "SomeComposition".into(),
                condition: Box::new(AndNot {
                    conditions: vec![
                        Compare {
                            field: SwapTotal,
                            operator: MoreThanEqual,
                            value: None,
                            timeframe: H1.into(),
                        },
                        Compare {
                            field: SwapTotal,
                            operator: LessThanEqual,
                            value: None,
                            timeframe: H1.into(),
                        },
                    ],
                }),
            };
            assert!(!test_instance.applicable())
        }

        #[test]
        fn test_value_and_no_value() {
            // compare 2 is applicable
            let test_instance = Condition::Compose {
                composition: "SomeComposition".into(),
                condition: Box::new(AndNot {
                    conditions: vec![
                        Compare {
                            field: SwapTotal,
                            operator: MoreThanEqual,
                            value: None,
                            timeframe: H1.into(),
                        },
                        Compare {
                            field: SwapTotal,
                            operator: LessThanEqual,
                            value: Value::count(23).into(),
                            timeframe: H1.into(),
                        },
                    ],
                }),
            };
            assert!(test_instance.applicable())
        }

        #[test]
        fn test_multiple_value_and_no_value() {
            // compare 2 AndNot 3 are applicable
            let test_instance = Condition::Compose {
                composition: "SomeComposition".into(),
                condition: Box::new(AndNot {
                    conditions: vec![
                        Compare {
                            field: SwapTotal,
                            operator: MoreThanEqual,
                            value: None,
                            timeframe: H1.into(),
                        },
                        Compare {
                            field: SwapTotal,
                            operator: LessThanEqual,
                            value: Value::count(23).into(),
                            timeframe: H1.into(),
                        },
                        Compare {
                            field: SwapTotal,
                            operator: MoreThanEqual,
                            value: Value::count(42).into(),
                            timeframe: H1.into(),
                        },
                    ],
                }),
            };
            assert!(test_instance.applicable())
        }

        #[test]
        fn test_single_child_not_applicable() {
            let test_instance = AndNot {
                conditions: vec![Compare {
                    field: TwitterAccountHandle,
                    operator: NotEqual,
                    value: Value::string("AI_nyanbot").into(),
                    timeframe: None,
                }],
            };
            assert!(!test_instance.applicable())
        }

        #[test]
        fn test_children_applicable() {
            let test_instance = AndNot {
                conditions: vec![
                    Compare {
                        field: SwapBuy,
                        operator: IncreasedByMoreThan,
                        value: Value::percent(1.0).into(),
                        timeframe: Some(M15),
                    },
                    Compare {
                        field: TwitterAccountHandle,
                        operator: Equal,
                        value: Value::string("AI_nyanbot").into(),
                        timeframe: None,
                    },
                ],
            };
            assert!(test_instance.applicable())
        }

        #[test]
        fn test_children_partial_applicable() {
            let test_instance = AndNot {
                conditions: vec![
                    // applicable
                    Compare {
                        field: SwapBuy,
                        operator: MoreThan,
                        value: Value::percent(1.0).into(),
                        timeframe: Some(M15),
                    },
                    // Not applicable
                    Compare {
                        field: TwitterAccountHandle,
                        operator: NotEqual,
                        value: Value::string("AI_nyanbot").into(),
                        timeframe: None,
                    },
                ],
            };
            assert!(!test_instance.applicable())
        }

        #[test]
        fn test_children_not_applicable() {
            let test_instance = AndNot {
                conditions: vec![
                    And { conditions: vec![] },
                    Or { conditions: vec![] },
                    AndNot { conditions: vec![] },
                ],
            };
            assert!(!test_instance.applicable())
        }
    }
}

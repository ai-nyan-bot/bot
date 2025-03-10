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
            Condition::And { conditions } => {
                !conditions.is_empty() && !conditions.iter().any(|c| !c.applicable())
            }
            Condition::Or { conditions } => {
                !conditions.is_empty() && !conditions.iter().any(|c| !c.applicable())
            }
            Condition::AndNot { conditions } => {
                !conditions.is_empty() && !conditions.iter().any(|c| !c.applicable())
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
        use crate::model::Condition::Compare;
        use crate::model::Field::{SwapBuy, TwitterAccountHandle};
        use crate::model::Operator::{Equal, MoreThan, NotEqual};
        use crate::model::{Condition, Value};
        use common::model::Timeframe::M15;

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
            let test_instance = Condition::And {
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
        use crate::model::Field::{SwapBuy, TwitterAccountHandle};
        use crate::model::Operator::{Equal, IncreasedByMoreThan, MoreThan, NotEqual};
        use crate::model::{Condition, Value};
        use common::model::Timeframe::M15;
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
            let test_instance = Condition::Or {
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
        use crate::model::Field::{SwapBuy, TwitterAccountHandle};
        use crate::model::Operator::{Equal, IncreasedByMoreThan, MoreThan, NotEqual};
        use crate::model::{Condition, Value};
        use common::model::Timeframe::M15;
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
            let test_instance = Condition::AndNot {
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

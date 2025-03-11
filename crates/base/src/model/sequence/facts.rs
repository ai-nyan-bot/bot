// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Fact, FactError, Value};
use common::model::Timeframe;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Facts {
    values: HashMap<Fact, Value>,
    timeframe_values: HashMap<Fact, HashMap<Timeframe, Value>>,
}

impl Default for Facts {
    fn default() -> Self {
        Self::new()
    }
}

impl Facts {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            timeframe_values: HashMap::new(),
        }
    }

    pub fn with_value(mut self, fact: Fact, value: impl Into<Value>) -> Self {
        self.set(fact, value.into(), None).unwrap();
        self
    }

    pub fn with_timeframe_value(
        mut self,
        fact: Fact,
        value: impl Into<Value>,
        timeframe: Timeframe,
    ) -> Self {
        self.set(fact, value.into(), Some(timeframe)).unwrap();
        self
    }

    pub fn get(&self, fact: &Fact) -> Option<&Value> {
        self.values.get(fact)
    }

    pub fn get_with_timeframe(&self, fact: &Fact, timeframe: &Timeframe) -> Option<&Value> {
        self.timeframe_values
            .get(fact)
            .and_then(|map| map.get(timeframe))
    }

    pub fn exists(&self, fact: &Fact, timeframe: Option<&Timeframe>) -> bool {
        match timeframe {
            None => self.values.contains_key(fact),
            Some(timeframe) => self
                .timeframe_values
                .get(fact)
                .map_or(false, |map| map.contains_key(timeframe)),
        }
    }

    pub fn set_timeframe_value(
        &mut self,
        fact: Fact,
        value: impl Into<Value>,
        timeframe: Timeframe,
    ) {
        self.set(fact, value.into(), Some(timeframe)).unwrap()
    }

    pub fn set_value(&mut self, fact: Fact, value: impl Into<Value>) {
        self.set(fact, value.into(), None).unwrap()
    }

    pub fn set(
        &mut self,
        fact: Fact,
        value: Value,
        timeframe: Option<Timeframe>,
    ) -> Result<(), FactError> {
        if value.value_type() != fact.value_type() {
            return Err(FactError::ValueTypeMismatch {
                expected: fact.value_type(),
                got: value.value_type(),
            });
        }

        if fact.has_timeframe() {
            if let Some(timeframe) = timeframe {
                self.timeframe_values
                    .entry(fact)
                    .or_default()
                    .insert(timeframe, value);
            } else {
                return Err(FactError::TimeframeRequired(fact));
            }
        } else {
            if timeframe.is_some() {
                return Err(FactError::TimeframeNotAllowed(fact));
            }
            self.values.insert(fact, value);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{Fact, FactError, Facts, Value, ValueType};
    use common::model::Timeframe;

    #[test]
    fn test_set_and_get_fact_without_timeframe() {
        let mut test_instance = Facts::new();
        let fact = Fact::PriceQuote;
        let value = Value::quote(123);

        assert!(test_instance.set(fact, value.clone(), None).is_ok());
        assert_eq!(test_instance.get(&fact), Some(&value));
    }

    #[test]
    fn test_set_and_get_fact_with_timeframe() {
        let mut test_instance = Facts::new();
        let fact = Fact::SwapBuyChangePercent;
        let timeframe = Timeframe::D1;
        let value = Value::percent(-2.5);

        assert!(test_instance
            .set(fact, value.clone(), Some(timeframe))
            .is_ok());
        assert_eq!(
            test_instance.get_with_timeframe(&fact, &timeframe),
            Some(&value)
        );
    }

    #[test]
    fn test_set_fact_with_timeframe_when_not_allowed() {
        let mut test_instance = Facts::new();
        let fact = Fact::PriceQuote;
        let timeframe = Timeframe::D1;
        let value = Value::quote(50);

        let result = test_instance.set(fact, value.clone(), Some(timeframe));
        assert!(matches!(result, Err(FactError::TimeframeNotAllowed(_))));
    }

    #[test]
    fn test_set_fact_without_timeframe_when_required() {
        let mut test_instance = Facts::new();
        let fact = Fact::SwapBuyChangePercent;
        let value = Value::percent(-3.0);

        let result = test_instance.set(fact, value.clone(), None);
        assert!(matches!(result, Err(FactError::TimeframeRequired(_))));
    }

    #[test]
    fn test_set_fact_with_invalid_value_kind() {
        let mut test_instance = Facts::new();
        let fact = Fact::SwapBuyChangePercent;
        let invalid_value = Value::boolean(true);

        let result = test_instance.set(fact, invalid_value.clone(), Some(Timeframe::H1));
        assert!(matches!(
            result,
            Err(FactError::ValueTypeMismatch {
                expected: ValueType::Percent,
                got: ValueType::Boolean
            })
        ));
    }

    #[test]
    fn test_exists_fact_without_timeframe() {
        let mut test_instance = Facts::new();
        let fact = Fact::PriceQuote;
        let value = Value::quote(200);

        assert!(!test_instance.exists(&fact, None));
        test_instance.set(fact, value, None).unwrap();
        assert!(test_instance.exists(&fact, None));
    }

    #[test]
    fn test_exists_fact_with_timeframe() {
        let mut test_instance = Facts::new();
        let fact = Fact::SwapChangePercent;
        let timeframe = Timeframe::D1;
        let value = Value::percent(500.0);

        assert!(!test_instance.exists(&fact, Some(&timeframe)));
        test_instance.set(fact, value, Some(timeframe)).unwrap();
        assert!(test_instance.exists(&fact, Some(&timeframe)));
    }

    #[test]
    fn test_overwrite_existing_fact_without_timeframe() {
        let mut test_instance = Facts::new();
        let fact = Fact::PriceQuote;
        let value1 = Value::quote(100);
        let value2 = Value::quote(200);

        test_instance.set(fact, value1.clone(), None).unwrap();
        assert_eq!(test_instance.get(&fact), Some(&value1));

        test_instance.set(fact, value2.clone(), None).unwrap();
        assert_eq!(test_instance.get(&fact), Some(&value2));
    }

    #[test]
    fn test_overwrite_existing_fact_with_timeframe() {
        let mut test_instance = Facts::new();
        let fact = Fact::SwapChangePercent;
        let timeframe = Timeframe::D1;
        let value1 = Value::percent(300.0);
        let value2 = Value::percent(500.0);

        test_instance
            .set(fact, value1.clone(), Some(timeframe))
            .unwrap();
        assert_eq!(
            test_instance.get_with_timeframe(&fact, &timeframe),
            Some(&value1)
        );

        test_instance
            .set(fact, value2.clone(), Some(timeframe))
            .unwrap();
        assert_eq!(
            test_instance.get_with_timeframe(&fact, &timeframe),
            Some(&value2)
        );
    }

    #[test]
    fn test_get_nonexistent_fact() {
        let test_instance = Facts::new();
        assert!(test_instance.get(&Fact::SwapBuyChangePercent).is_none());
        assert!(test_instance
            .get_with_timeframe(&Fact::SwapBuyChangePercent, &Timeframe::S1)
            .is_none());
    }

    #[test]
    fn test_set_fact_with_different_timeframes() {
        let mut test_instance = Facts::new();
        let fact = Fact::SwapBuyChangePercent;

        let value1 = Value::percent(1.2);
        let value2 = Value::percent(2.4);
        let value3 = Value::percent(3.6);

        let timeframe1 = Timeframe::H1;
        let timeframe2 = Timeframe::D1;
        let timeframe3 = Timeframe::S1;

        assert!(test_instance
            .set(fact, value1.clone(), Some(timeframe1))
            .is_ok());
        assert!(test_instance
            .set(fact, value2.clone(), Some(timeframe2))
            .is_ok());
        assert!(test_instance
            .set(fact, value3.clone(), Some(timeframe3))
            .is_ok());

        assert_eq!(
            test_instance.get_with_timeframe(&fact, &timeframe1),
            Some(&value1)
        );
        assert_eq!(
            test_instance.get_with_timeframe(&fact, &timeframe2),
            Some(&value2)
        );
        assert_eq!(
            test_instance.get_with_timeframe(&fact, &timeframe3),
            Some(&value3)
        );
    }

    #[test]
    fn test_exists_for_different_timeframes() {
        let mut test_instance = Facts::new();
        let fact = Fact::SwapBuyChangePercent;

        let timeframe1 = Timeframe::H1;
        let timeframe2 = Timeframe::D1;

        let value1 = Value::percent(-2.0);
        let value2 = Value::percent(-5.5);

        assert!(test_instance
            .set(fact, value1.clone(), Some(timeframe1))
            .is_ok());
        assert!(test_instance
            .set(fact, value2.clone(), Some(timeframe2))
            .is_ok());

        assert!(test_instance.exists(&fact, Some(&timeframe1)));
        assert!(test_instance.exists(&fact, Some(&timeframe2)));
        assert!(!test_instance.exists(&fact, Some(&Timeframe::S1)));
    }

    #[test]
    fn test_update_multiple_timeframes_independently() {
        let mut test_instance = Facts::new();
        let fact = Fact::SwapBuyChangePercent;

        let timeframe1 = Timeframe::H1;
        let timeframe2 = Timeframe::D1;
        let timeframe3 = Timeframe::S1;

        let value1 = Value::percent(1.5);
        let value2 = Value::percent(3.0);
        let value3 = Value::percent(4.5);

        assert!(test_instance
            .set(fact, value1.clone(), Some(timeframe1))
            .is_ok());
        assert!(test_instance
            .set(fact, value2.clone(), Some(timeframe2))
            .is_ok());
        assert!(test_instance
            .set(fact, value3.clone(), Some(timeframe3))
            .is_ok());

        let new_value2 = Value::percent(6.0);
        assert!(test_instance
            .set(fact, new_value2.clone(), Some(timeframe2))
            .is_ok());

        assert_eq!(
            test_instance.get_with_timeframe(&fact, &timeframe1),
            Some(&value1)
        );
        assert_eq!(
            test_instance.get_with_timeframe(&fact, &timeframe2),
            Some(&new_value2)
        );
        assert_eq!(
            test_instance.get_with_timeframe(&fact, &timeframe3),
            Some(&value3)
        );
    }
}

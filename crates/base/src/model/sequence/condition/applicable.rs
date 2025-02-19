// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Condition, Fact};

impl Condition {
	/// Determines whether a condition can be applied.
	/// E.g. the user provides an empty AND condition, which would match everything
	pub fn applicable(&self) -> bool {
		match self {
			Condition::Compare { .. } => Fact::try_from(self).is_ok(),
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
			use crate::model::Operator::{
				DecreasedByLessThan, DecreasedByLessThanEqual, DecreasedByMoreThan, DecreasedByMoreThanEqual, Equal, IncreasedByMoreThan,
				IncreasedByMoreThanEqual, LessThan, LessThanEqual, MoreThan, MoreThanEqual, NotEqual,
			};
			use crate::model::Value::Boolean;

			#[test]
			fn equal_applicable() {
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
			fn not_equal_applicable() {
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
			fn increased_by_more_than_not_applicable() {
				let test_instance = Compare {
					field: TwitterExists,
					operator: IncreasedByMoreThan,
					value: Boolean(false),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false);

				let test_instance = Compare {
					field: TwitterExists,
					operator: IncreasedByMoreThan,
					value: Boolean(true),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false)
			}

			#[test]
			fn increased_by_more_than_equal_not_applicable() {
				let test_instance = Compare {
					field: TwitterExists,
					operator: IncreasedByMoreThanEqual,
					value: Boolean(false),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false);

				let test_instance = Compare {
					field: TwitterExists,
					operator: IncreasedByMoreThanEqual,
					value: Boolean(true),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false)
			}

			#[test]
			fn increased_by_less_than_not_applicable() {
				let test_instance = Compare {
					field: TwitterExists,
					operator: DecreasedByLessThan,
					value: Boolean(false),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false);

				let test_instance = Compare {
					field: TwitterExists,
					operator: DecreasedByLessThan,
					value: Boolean(true),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false)
			}

			#[test]
			fn increased_by_less_than_equal_not_applicable() {
				let test_instance = Compare {
					field: TwitterExists,
					operator: DecreasedByLessThanEqual,
					value: Boolean(false),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false);

				let test_instance = Compare {
					field: TwitterExists,
					operator: DecreasedByLessThanEqual,
					value: Boolean(true),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false)
			}

			#[test]
			fn decreased_by_more_than_not_applicable() {
				let test_instance = Compare {
					field: TwitterExists,
					operator: DecreasedByMoreThan,
					value: Boolean(false),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false);

				let test_instance = Compare {
					field: TwitterExists,
					operator: DecreasedByMoreThan,
					value: Boolean(true),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false)
			}

			#[test]
			fn decreased_by_more_than_equal_not_applicable() {
				let test_instance = Compare {
					field: TwitterExists,
					operator: DecreasedByMoreThanEqual,
					value: Boolean(false),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false);

				let test_instance = Compare {
					field: TwitterExists,
					operator: DecreasedByMoreThanEqual,
					value: Boolean(true),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false)
			}

			#[test]
			fn decreased_by_less_than_not_applicable() {
				let test_instance = Compare {
					field: TwitterExists,
					operator: DecreasedByLessThan,
					value: Boolean(false),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false);

				let test_instance = Compare {
					field: TwitterExists,
					operator: DecreasedByLessThan,
					value: Boolean(true),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false)
			}

			#[test]
			fn decreased_by_less_than_equal_not_applicable() {
				let test_instance = Compare {
					field: TwitterExists,
					operator: DecreasedByLessThanEqual,
					value: Boolean(false),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false);

				let test_instance = Compare {
					field: TwitterExists,
					operator: DecreasedByLessThanEqual,
					value: Boolean(true),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false)
			}

			#[test]
			fn more_than_not_applicable() {
				let test_instance = Compare {
					field: TwitterExists,
					operator: MoreThan,
					value: Boolean(false),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false);

				let test_instance = Compare {
					field: TwitterExists,
					operator: MoreThan,
					value: Boolean(true),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false)
			}

			#[test]
			fn more_than_equal_not_applicable() {
				let test_instance = Compare {
					field: TwitterExists,
					operator: MoreThanEqual,
					value: Boolean(false),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false);

				let test_instance = Compare {
					field: TwitterExists,
					operator: MoreThanEqual,
					value: Boolean(true),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false)
			}

			#[test]
			fn less_than_not_applicable() {
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
			fn less_than_equal_not_applicable() {
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
			use crate::model::Operator::{
				DecreasedByLessThan, DecreasedByLessThanEqual, DecreasedByMoreThan, DecreasedByMoreThanEqual, Equal, IncreasedByLessThan,
				IncreasedByLessThanEqual, IncreasedByMoreThan, IncreasedByMoreThanEqual, NotEqual,
			};
			use crate::model::{Condition, Field, Operator, Value};
			use rand::random;
			use Condition::Compare;
			use Field::TradesBuy;
			use Operator::{LessThan, LessThanEqual, MoreThan, MoreThanEqual};
			use Value::Count;

			#[test]
			fn increased_by_more_than_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: IncreasedByMoreThan,
					value: Count(random()),
					timeframe: Some(random()),
				};
				dbg!(&test_instance);
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn increased_by_more_than_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: IncreasedByMoreThanEqual,
					value: Count(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn increased_by_less_than_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: IncreasedByLessThan,
					value: Count(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn increased_by_less_than_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: IncreasedByLessThanEqual,
					value: Count(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn decreased_by_more_than_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: DecreasedByMoreThan,
					value: Count(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn decreased_by_more_than_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: DecreasedByMoreThanEqual,
					value: Count(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn decreased_by_less_than_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: DecreasedByLessThan,
					value: Count(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn decreased_by_less_than_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: DecreasedByLessThanEqual,
					value: Count(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn more_than_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: MoreThan,
					value: Count(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn more_than_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: MoreThanEqual,
					value: Count(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn less_than_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: LessThan,
					value: Count(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn less_than_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: LessThanEqual,
					value: Count(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: Equal,
					value: Count(random()),
					timeframe: Some(random()),
				};

				assert_eq!(test_instance.applicable(), true);
			}

			#[test]
			fn not_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: NotEqual,
					value: Count(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true);
			}
		}

		mod percent {
			use crate::model::Operator::{
				DecreasedByLessThan, DecreasedByLessThanEqual, DecreasedByMoreThan, DecreasedByMoreThanEqual, Equal, IncreasedByLessThan,
				IncreasedByLessThanEqual, IncreasedByMoreThan, IncreasedByMoreThanEqual, NotEqual,
			};
			use crate::model::{Condition, Field, Operator, Value};
			use rand::random;
			use Condition::Compare;
			use Field::TradesBuy;
			use Operator::{LessThan, LessThanEqual, MoreThan, MoreThanEqual};
			use Value::Percent;

			#[test]
			fn increased_by_more_than_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: IncreasedByMoreThan,
					value: Percent(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn increased_by_more_than_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: IncreasedByMoreThanEqual,
					value: Percent(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn increased_by_less_than_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: IncreasedByLessThan,
					value: Percent(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn increased_by_less_than_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: IncreasedByLessThanEqual,
					value: Percent(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn decreased_by_more_than_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: DecreasedByMoreThan,
					value: Percent(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn decreased_by_more_than_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: DecreasedByMoreThanEqual,
					value: Percent(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn decreased_by_less_than_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: DecreasedByLessThan,
					value: Percent(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn decreased_by_less_than_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: DecreasedByLessThanEqual,
					value: Percent(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn more_than_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: MoreThan,
					value: Percent(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn more_than_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: MoreThanEqual,
					value: Percent(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn less_than_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: LessThan,
					value: Percent(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn less_than_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: LessThanEqual,
					value: Percent(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: Equal,
					value: Percent(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true);
			}

			#[test]
			fn not_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: NotEqual,
					value: Percent(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true);
			}
		}

		mod quote {
			use crate::model::Operator::{
				DecreasedByLessThan, DecreasedByLessThanEqual, DecreasedByMoreThan, DecreasedByMoreThanEqual, Equal, IncreasedByLessThan,
				IncreasedByLessThanEqual, IncreasedByMoreThan, IncreasedByMoreThanEqual, NotEqual,
			};
			use crate::model::{Condition, Field, Operator, Value};
			use rand::random;
			use Condition::Compare;
			use Field::TradesBuy;
			use Operator::{LessThan, LessThanEqual, MoreThan, MoreThanEqual};
			use Value::Quote;

			#[test]
			fn increased_by_more_than_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: IncreasedByMoreThan,
					value: Quote(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn increased_by_more_than_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: IncreasedByMoreThanEqual,
					value: Quote(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn increased_by_less_than_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: IncreasedByLessThan,
					value: Quote(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn increased_by_less_than_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: IncreasedByLessThanEqual,
					value: Quote(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn decreased_by_more_than_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: DecreasedByMoreThan,
					value: Quote(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn decreased_by_more_than_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: DecreasedByMoreThanEqual,
					value: Quote(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn decreased_by_less_than_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: DecreasedByLessThan,
					value: Quote(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn decreased_by_less_than_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: DecreasedByLessThanEqual,
					value: Quote(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn more_than_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: MoreThan,
					value: Quote(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn more_than_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: MoreThanEqual,
					value: Quote(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn less_than_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: LessThan,
					value: Quote(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn less_than_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: LessThanEqual,
					value: Quote(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: Equal,
					value: Quote(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true);
			}

			#[test]
			fn not_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: NotEqual,
					value: Quote(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true);
			}
		}

		mod string {
			use crate::model::Condition::Compare;
			use crate::model::Field::TwitterHandle;
			use crate::model::Operator::{
				DecreasedByLessThan, DecreasedByLessThanEqual, DecreasedByMoreThan, DecreasedByMoreThanEqual, Equal, IncreasedByLessThan,
				IncreasedByLessThanEqual, IncreasedByMoreThan, IncreasedByMoreThanEqual, LessThan, LessThanEqual, MoreThan, MoreThanEqual, NotEqual,
			};
			use crate::model::Value::String;

			#[test]
			fn equal_applicable() {
				let test_instance = Compare {
					field: TwitterHandle,
					operator: Equal,
					value: String("AI_nyanbot".to_string()),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), true);
			}

			#[test]
			fn equal_not_applicable() {
				let test_instance = Compare {
					field: TwitterHandle,
					operator: Equal,
					value: String("   ".to_string()),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false);
			}

			#[test]
			fn not_equal_not_applicable() {
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
			fn increased_by_more_than_not_applicable() {
				let test_instance = Compare {
					field: TwitterHandle,
					operator: IncreasedByMoreThan,
					value: String("AI_nyanbot".to_string()),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false)
			}

			#[test]
			fn increased_by_more_than_equal_not_applicable() {
				let test_instance = Compare {
					field: TwitterHandle,
					operator: IncreasedByMoreThanEqual,
					value: String("AI_nyanbot".to_string()),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false);
			}

			#[test]
			fn increased_by_less_than_not_applicable() {
				let test_instance = Compare {
					field: TwitterHandle,
					operator: IncreasedByLessThan,
					value: String("AI_nyanbot".to_string()),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false)
			}

			#[test]
			fn increased_by_less_than_equal_not_applicable() {
				let test_instance = Compare {
					field: TwitterHandle,
					operator: IncreasedByLessThanEqual,
					value: String("AI_nyanbot".to_string()),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false);
			}

			#[test]
			fn decreased_by_more_than_not_applicable() {
				let test_instance = Compare {
					field: TwitterHandle,
					operator: DecreasedByMoreThan,
					value: String("AI_nyanbot".to_string()),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false)
			}

			#[test]
			fn decreased_by_more_than_equal_not_applicable() {
				let test_instance = Compare {
					field: TwitterHandle,
					operator: DecreasedByMoreThanEqual,
					value: String("AI_nyanbot".to_string()),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false);
			}

			#[test]
			fn decreased_by_less_than_not_applicable() {
				let test_instance = Compare {
					field: TwitterHandle,
					operator: DecreasedByLessThan,
					value: String("AI_nyanbot".to_string()),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false)
			}

			#[test]
			fn decreased_by_less_than_equal_not_applicable() {
				let test_instance = Compare {
					field: TwitterHandle,
					operator: DecreasedByLessThanEqual,
					value: String("AI_nyanbot".to_string()),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false);
			}

			#[test]
			fn more_than_not_applicable() {
				let test_instance = Compare {
					field: TwitterHandle,
					operator: MoreThan,
					value: String("AI_nyanbot".to_string()),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false)
			}

			#[test]
			fn more_than_equal_not_applicable() {
				let test_instance = Compare {
					field: TwitterHandle,
					operator: MoreThanEqual,
					value: String("AI_nyanbot".to_string()),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false);
			}

			#[test]
			fn less_than_not_applicable() {
				let test_instance = Compare {
					field: TwitterHandle,
					operator: LessThan,
					value: String("AI_nyanbot".to_string()),
					timeframe: None,
				};
				assert_eq!(test_instance.applicable(), false)
			}

			#[test]
			fn less_than_equal_not_applicable() {
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
			use crate::model::Operator::{
				DecreasedByLessThan, DecreasedByLessThanEqual, DecreasedByMoreThan, DecreasedByMoreThanEqual, Equal, IncreasedByLessThan,
				IncreasedByLessThanEqual, IncreasedByMoreThan, IncreasedByMoreThanEqual, NotEqual,
			};
			use crate::model::{Condition, Field, Operator, Value};
			use rand::random;
			use Condition::Compare;
			use Field::TradesBuy;
			use Operator::{LessThan, LessThanEqual, MoreThan, MoreThanEqual};
			use Value::Usd;

			#[test]
			fn increased_by_more_than_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: IncreasedByMoreThan,
					value: Usd(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn increased_by_more_than_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: IncreasedByMoreThanEqual,
					value: Usd(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn increased_by_less_than_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: IncreasedByLessThan,
					value: Usd(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn increased_by_less_than_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: IncreasedByLessThanEqual,
					value: Usd(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn decreased_by_more_than_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: DecreasedByMoreThan,
					value: Usd(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn decreased_by_more_than_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: DecreasedByMoreThanEqual,
					value: Usd(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn decreased_by_less_than_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: DecreasedByLessThan,
					value: Usd(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn decreased_by_less_than_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: DecreasedByLessThanEqual,
					value: Usd(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn more_than_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: MoreThan,
					value: Usd(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn more_than_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: MoreThanEqual,
					value: Usd(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn less_than_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: LessThan,
					value: Usd(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn less_than_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: LessThanEqual,
					value: Usd(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true)
			}

			#[test]
			fn equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: Equal,
					value: Usd(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true);
			}

			#[test]
			fn not_equal_applicable() {
				let test_instance = Compare {
					field: TradesBuy,
					operator: NotEqual,
					value: Usd(random()),
					timeframe: Some(random()),
				};
				assert_eq!(test_instance.applicable(), true);
			}
		}
	}

	mod and {
		use crate::model::Condition;
		use crate::model::Condition::Compare;
		use crate::model::Field::{TradesBuy, TwitterHandle};
		use crate::model::Operator::{Equal, MoreThan, NotEqual};
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
						operator: MoreThan,
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
						operator: MoreThan,
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
		use crate::model::Operator::{Equal, MoreThan, NotEqual};
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
						operator: MoreThan,
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
						operator: MoreThan,
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
		use crate::model::Operator::{Equal, MoreThan, NotEqual};
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
						operator: MoreThan,
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
						operator: MoreThan,
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

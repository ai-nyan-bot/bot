// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::FactError::UnableToDeriveFact;
use crate::model::Field::PriceAvg;
use crate::model::ValueType::{Count, Percent, Quote, Usd};
use crate::model::{Condition, FactError, Field, Operator, Value, ValueType};
use serde::{Deserialize, Serialize};
use Fact::{PriceAvgQuote, PriceAvgUsd, PriceQuote, PriceUsd, TelegramGroup, TelegramGroupHandle, TradesBuyChangeCount, TradesBuyChangePercent, TradesBuyCount, TradesChangeCount, TradesChangePercent, TradesCount, TradesSellChangeCount, TradesSellChangePercent, TradesSellCount, TwitterAccount, TwitterAccountHandle, VolumeChangeQuote};
use Field::{Price, Trades, TradesBuy, TradesSell, Volume};
use Operator::{DecreasedByLessThan, DecreasedByLessThanEqual, DecreasedByMoreThan, DecreasedByMoreThanEqual, Equal, IncreasedByLessThan, IncreasedByLessThanEqual, IncreasedByMoreThan, IncreasedByMoreThanEqual, LessThan, LessThanEqual, MoreThan, MoreThanEqual};
use ValueType::Boolean;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Fact {
	PriceQuote,
	PriceUsd,

	PriceAvgQuote,
	PriceAvgUsd,
	// PriceChangePercent,
	// PriceChangeQuote,
	// PriceChangeUsd,

	TradesCount,
	TradesBuyCount,
	TradesSellCount,

	TradesChangeCount,
	TradesBuyChangeCount,
	TradesSellChangeCount,

	TradesChangePercent,
	TradesBuyChangePercent,
	TradesSellChangePercent,

	// VolumeQuote,
	// VolumeUsd,
	// VolumeChangePercent,
	VolumeChangeQuote,
	// VolumeChangeUsd,

	TelegramGroup,
	TelegramGroupHandle,

	TwitterAccount,
	TwitterAccountHandle,
}

impl Fact {
	pub fn has_timeframe(&self) -> bool {
		match self {
			PriceQuote => false,
			PriceUsd => false,

			PriceAvgQuote => true,
			PriceAvgUsd => true,
			//
			// PriceChangeQuote => true,
			// PriceChangePercent => true,
			// PriceChangeUsd => true,

			TradesCount => true,
			TradesBuyCount => true,
			TradesSellCount => true,

			TradesChangeCount => true,
			TradesBuyChangeCount => true,
			TradesSellChangeCount => true,

			TradesChangePercent => true,
			TradesBuyChangePercent => true,
			TradesSellChangePercent => true,

			// VolumeQuote => false,
			// VolumeUsd => false,
			VolumeChangeQuote => true,
			// VolumeChangePercent => true,
			// VolumeChangeUsd => true,

			TelegramGroup => false,
			TelegramGroupHandle => false,

			TwitterAccount => false,
			TwitterAccountHandle => false,
		}
	}

	pub fn value_type(&self) -> ValueType {
		match self {
			PriceQuote => Quote,
			PriceUsd => Usd,

			PriceAvgQuote => Quote,
			PriceAvgUsd => Usd,
			//
			// PriceChangeQuote => Quote,
			// PriceChangeUsd => Usd,
			// PriceChangePercent => Percent,

			TradesCount => Count,
			TradesBuyCount => Count,
			TradesSellCount => Count,

			TradesChangeCount => Count,
			TradesBuyChangeCount => Count,
			TradesSellChangeCount => Count,

			TradesChangePercent => Percent,
			TradesBuyChangePercent => Percent,
			TradesSellChangePercent => Percent,

			// VolumeQuote => Quote,
			// VolumeUsd => Usd,
			VolumeChangeQuote => Quote,
			// VolumeChangePercent => Percent,
			// VolumeChangeUsd => Usd,

			TelegramGroup => Boolean,
			TelegramGroupHandle => ValueType::String,
			TwitterAccount => Boolean,
			TwitterAccountHandle => ValueType::String,
		}
	}
}

impl TryFrom<&Condition> for Fact {
	type Error = FactError;

	fn try_from(condition: &Condition) -> Result<Self, Self::Error> {
		match condition {
			Condition::Compare {
				field,
				operator,
				value,
				timeframe,
				..
			} => Fact::from_comparison(field, operator, value, timeframe.is_some()).ok_or(UnableToDeriveFact(condition.clone())),
			Condition::And { .. } | Condition::Or { .. } | Condition::AndNot { .. } => Err(UnableToDeriveFact(condition.clone())),
		}
	}
}

impl Fact {
	fn from_comparison(field: &Field, operator: &Operator, value: &Value, has_timeframe: bool) -> Option<Self> {
		let fact = match (field, operator, value.value_type(), has_timeframe) {
			// PriceQuote
			(Price, MoreThan, Quote, false) => PriceQuote,
			(Price, MoreThanEqual, Quote, false) => PriceQuote,
			(Price, LessThan, Quote, false) => PriceQuote,
			(Price, LessThanEqual, Quote, false) => PriceQuote,

			// PriceUsd
			(Price, MoreThan, Usd, false) => PriceUsd,
			(Price, MoreThanEqual, Usd, false) => PriceUsd,
			(Price, LessThan, Usd, false) => PriceUsd,
			(Price, LessThanEqual, Usd, false) => PriceUsd,

			// PriceAvgQuote
			(PriceAvg, MoreThan, Quote, true) => PriceAvgQuote,
			(PriceAvg, MoreThanEqual, Quote, true) => PriceAvgQuote,
			(PriceAvg, LessThan, Quote, true) => PriceAvgQuote,
			(PriceAvg, LessThanEqual, Quote, true) => PriceAvgQuote,

			// PriceAvgUsd
			(PriceAvg, MoreThan, Usd, true) => PriceAvgUsd,
			(PriceAvg, MoreThanEqual, Usd, true) => PriceAvgUsd,
			(PriceAvg, LessThan, Usd, true) => PriceAvgUsd,
			(PriceAvg, LessThanEqual, Usd, true) => PriceAvgUsd,

			// TradesCount
			(Trades, MoreThan, Count, true) => TradesCount,
			(Trades, MoreThanEqual, Count, true) => TradesCount,
			(Trades, LessThan, Count, true) => TradesCount,
			(Trades, LessThanEqual, Count, true) => TradesCount,

			// TradesBuyCount
			(TradesBuy, MoreThan, Count, true) => TradesBuyCount,
			(TradesBuy, MoreThanEqual, Count, true) => TradesBuyCount,
			(TradesBuy, LessThan, Count, true) => TradesBuyCount,
			(TradesBuy, LessThanEqual, Count, true) => TradesBuyCount,

			// TradesSellCount
			(TradesSell, MoreThan, Count, true) => TradesSellCount,
			(TradesSell, MoreThanEqual, Count, true) => TradesSellCount,
			(TradesSell, LessThan, Count, true) => TradesSellCount,
			(TradesSell, LessThanEqual, Count, true) => TradesSellCount,

			// TradesChangeCount
			(Trades, IncreasedByMoreThan, Count, true) => TradesChangeCount,
			(Trades, IncreasedByMoreThanEqual, Count, true) => TradesChangeCount,
			(Trades, IncreasedByLessThan, Count, true) => TradesChangeCount,
			(Trades, IncreasedByLessThanEqual, Count, true) => TradesChangeCount,
			(Trades, DecreasedByMoreThan, Count, true) => TradesChangeCount,
			(Trades, DecreasedByMoreThanEqual, Count, true) => TradesChangeCount,
			(Trades, DecreasedByLessThan, Count, true) => TradesChangeCount,
			(Trades, DecreasedByLessThanEqual, Count, true) => TradesChangeCount,

			// TradesBuyChangeCount
			(TradesBuy, IncreasedByMoreThan, Count, true) => TradesBuyChangeCount,
			(TradesBuy, IncreasedByMoreThanEqual, Count, true) => TradesBuyChangeCount,
			(TradesBuy, IncreasedByLessThan, Count, true) => TradesBuyChangeCount,
			(TradesBuy, IncreasedByLessThanEqual, Count, true) => TradesBuyChangeCount,
			(TradesBuy, DecreasedByMoreThan, Count, true) => TradesBuyChangeCount,
			(TradesBuy, DecreasedByMoreThanEqual, Count, true) => TradesBuyChangeCount,
			(TradesBuy, DecreasedByLessThan, Count, true) => TradesBuyChangeCount,
			(TradesBuy, DecreasedByLessThanEqual, Count, true) => TradesBuyChangeCount,

			// TradesSellChangeCount
			(TradesSell, IncreasedByMoreThan, Count, true) => TradesSellChangeCount,
			(TradesSell, IncreasedByMoreThanEqual, Count, true) => TradesSellChangeCount,
			(TradesSell, IncreasedByLessThan, Count, true) => TradesSellChangeCount,
			(TradesSell, IncreasedByLessThanEqual, Count, true) => TradesSellChangeCount,
			(TradesSell, DecreasedByMoreThan, Count, true) => TradesSellChangeCount,
			(TradesSell, DecreasedByMoreThanEqual, Count, true) => TradesSellChangeCount,
			(TradesSell, DecreasedByLessThan, Count, true) => TradesSellChangeCount,
			(TradesSell, DecreasedByLessThanEqual, Count, true) => TradesSellChangeCount,

			// TradesChangePercent
			(Trades, IncreasedByMoreThan, Percent, true) => TradesChangePercent,
			(Trades, IncreasedByMoreThanEqual, Percent, true) => TradesChangePercent,
			(Trades, IncreasedByLessThan, Percent, true) => TradesChangePercent,
			(Trades, IncreasedByLessThanEqual, Percent, true) => TradesChangePercent,
			(Trades, DecreasedByMoreThan, Percent, true) => TradesChangePercent,
			(Trades, DecreasedByMoreThanEqual, Percent, true) => TradesChangePercent,
			(Trades, DecreasedByLessThan, Percent, true) => TradesChangePercent,
			(Trades, DecreasedByLessThanEqual, Percent, true) => TradesChangePercent,

			// TradesBuyChangePercent
			(TradesBuy, IncreasedByMoreThan, Percent, true) => TradesBuyChangePercent,
			(TradesBuy, IncreasedByMoreThanEqual, Percent, true) => TradesBuyChangePercent,
			(TradesBuy, IncreasedByLessThan, Percent, true) => TradesBuyChangePercent,
			(TradesBuy, IncreasedByLessThanEqual, Percent, true) => TradesBuyChangePercent,
			(TradesBuy, DecreasedByMoreThan, Percent, true) => TradesBuyChangePercent,
			(TradesBuy, DecreasedByMoreThanEqual, Percent, true) => TradesBuyChangePercent,
			(TradesBuy, DecreasedByLessThan, Percent, true) => TradesBuyChangePercent,
			(TradesBuy, DecreasedByLessThanEqual, Percent, true) => TradesBuyChangePercent,

			// TradesSellChangePercent
			(TradesSell, IncreasedByMoreThan, Percent, true) => TradesSellChangePercent,
			(TradesSell, IncreasedByMoreThanEqual, Percent, true) => TradesSellChangePercent,
			(TradesSell, IncreasedByLessThan, Percent, true) => TradesSellChangePercent,
			(TradesSell, IncreasedByLessThanEqual, Percent, true) => TradesSellChangePercent,
			(TradesSell, DecreasedByMoreThan, Percent, true) => TradesSellChangePercent,
			(TradesSell, DecreasedByMoreThanEqual, Percent, true) => TradesSellChangePercent,
			(TradesSell, DecreasedByLessThan, Percent, true) => TradesSellChangePercent,
			(TradesSell, DecreasedByLessThanEqual, Percent, true) => TradesSellChangePercent,

			// VolumeChangeQuote
			(Volume, IncreasedByMoreThan, Quote, true) => VolumeChangeQuote,
			(Volume, IncreasedByMoreThanEqual, Quote, true) => VolumeChangeQuote,
			(Volume, IncreasedByLessThan, Quote, true) => VolumeChangeQuote,
			(Volume, IncreasedByLessThanEqual, Quote, true) => VolumeChangeQuote,
			(Volume, DecreasedByMoreThan, Quote, true) => VolumeChangeQuote,
			(Volume, DecreasedByMoreThanEqual, Quote, true) => VolumeChangeQuote,
			(Volume, DecreasedByLessThan, Quote, true) => VolumeChangeQuote,
			(Volume, DecreasedByLessThanEqual, Quote, true) => VolumeChangeQuote,

			// Telegram
			(Field::TelegramGroupHandle, Equal, ValueType::String, false) => TelegramGroupHandle,

			// Twitter
			(Field::TwitterAccountHandle, Equal, ValueType::String, false) => TwitterAccountHandle,

			_ => return None,
		};

		Some(fact)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_price_quote() {
		assert_eq!(Fact::from_comparison(&Price, &MoreThan, &Value::Quote(99.24), false), Some(PriceQuote));
		assert_eq!(Fact::from_comparison(&Price, &MoreThanEqual, &Value::Quote(99.24), false), Some(PriceQuote));
		assert_eq!(Fact::from_comparison(&Price, &LessThan, &Value::Quote(99.24), false), Some(PriceQuote));
		assert_eq!(Fact::from_comparison(&Price, &LessThanEqual, &Value::Quote(99.24), false), Some(PriceQuote));
	}

	#[test]
	fn test_price_usd() {
		assert_eq!(Fact::from_comparison(&Price, &MoreThan, &Value::Usd(99.24), false), Some(PriceUsd));
		assert_eq!(Fact::from_comparison(&Price, &MoreThanEqual, &Value::Usd(99.24), false), Some(PriceUsd));
		assert_eq!(Fact::from_comparison(&Price, &LessThan, &Value::Usd(99.24), false), Some(PriceUsd));
		assert_eq!(Fact::from_comparison(&Price, &LessThanEqual, &Value::Usd(99.24), false), Some(PriceUsd));
	}

	#[test]
	fn test_price_avg_quote() {
		assert_eq!(Fact::from_comparison(&PriceAvg, &MoreThan, &Value::Quote(99.24), true), Some(PriceAvgQuote));
		assert_eq!(Fact::from_comparison(&PriceAvg, &MoreThanEqual, &Value::Quote(99.24), true), Some(PriceAvgQuote));
		assert_eq!(Fact::from_comparison(&PriceAvg, &LessThan, &Value::Quote(99.24), true), Some(PriceAvgQuote));
		assert_eq!(Fact::from_comparison(&PriceAvg, &LessThanEqual, &Value::Quote(99.24), true), Some(PriceAvgQuote));
	}

	#[test]
	fn test_price_avg_usd() {
		assert_eq!(Fact::from_comparison(&PriceAvg, &MoreThan, &Value::Usd(99.24), true), Some(PriceAvgUsd));
		assert_eq!(Fact::from_comparison(&PriceAvg, &MoreThanEqual, &Value::Usd(99.24), true), Some(PriceAvgUsd));
		assert_eq!(Fact::from_comparison(&PriceAvg, &LessThan, &Value::Usd(99.24), true), Some(PriceAvgUsd));
		assert_eq!(Fact::from_comparison(&PriceAvg, &LessThanEqual, &Value::Usd(99.24), true), Some(PriceAvgUsd));
	}

	#[test]
	fn test_trades_count() {
		assert_eq!(Fact::from_comparison(&Trades, &MoreThan, &Value::Count(9924), true), Some(TradesCount));
		assert_eq!(Fact::from_comparison(&Trades, &MoreThanEqual, &Value::Count(9924), true), Some(TradesCount));
		assert_eq!(Fact::from_comparison(&Trades, &LessThan, &Value::Count(9924), true), Some(TradesCount));
		assert_eq!(Fact::from_comparison(&Trades, &LessThanEqual, &Value::Count(9924), true), Some(TradesCount));
	}

	#[test]
	fn test_trades_buy_count() {
		assert_eq!(Fact::from_comparison(&TradesBuy, &MoreThan, &Value::Count(9924), true), Some(TradesBuyCount));
		assert_eq!(Fact::from_comparison(&TradesBuy, &MoreThanEqual, &Value::Count(9924), true), Some(TradesBuyCount));
		assert_eq!(Fact::from_comparison(&TradesBuy, &LessThan, &Value::Count(9924), true), Some(TradesBuyCount));
		assert_eq!(Fact::from_comparison(&TradesBuy, &LessThanEqual, &Value::Count(9924), true), Some(TradesBuyCount));
	}

	#[test]
	fn test_trades_sell_count() {
		assert_eq!(Fact::from_comparison(&TradesSell, &MoreThan, &Value::Count(9924), true), Some(TradesSellCount));
		assert_eq!(Fact::from_comparison(&TradesSell, &MoreThanEqual, &Value::Count(9924), true), Some(TradesSellCount));
		assert_eq!(Fact::from_comparison(&TradesSell, &LessThan, &Value::Count(9924), true), Some(TradesSellCount));
		assert_eq!(Fact::from_comparison(&TradesSell, &LessThanEqual, &Value::Count(9924), true), Some(TradesSellCount));
	}

	#[test]
	fn test_trades_change_count() {
		assert_eq!(Fact::from_comparison(&Trades, &IncreasedByMoreThan, &Value::Count(9924), true), Some(TradesChangeCount));
		assert_eq!(Fact::from_comparison(&Trades, &IncreasedByMoreThanEqual, &Value::Count(9924), true), Some(TradesChangeCount));
		assert_eq!(Fact::from_comparison(&Trades, &IncreasedByLessThan, &Value::Count(9924), true), Some(TradesChangeCount));
		assert_eq!(Fact::from_comparison(&Trades, &IncreasedByLessThanEqual, &Value::Count(9924), true), Some(TradesChangeCount));
		assert_eq!(Fact::from_comparison(&Trades, &DecreasedByMoreThan, &Value::Count(9924), true), Some(TradesChangeCount));
		assert_eq!(Fact::from_comparison(&Trades, &DecreasedByMoreThanEqual, &Value::Count(9924), true), Some(TradesChangeCount));
		assert_eq!(Fact::from_comparison(&Trades, &DecreasedByLessThan, &Value::Count(9924), true), Some(TradesChangeCount));
		assert_eq!(Fact::from_comparison(&Trades, &DecreasedByLessThanEqual, &Value::Count(9924), true), Some(TradesChangeCount));
	}

	#[test]
	fn test_trades_buy_change_count() {
		assert_eq!(Fact::from_comparison(&TradesBuy, &IncreasedByMoreThan, &Value::Count(9924), true), Some(TradesBuyChangeCount));
		assert_eq!(Fact::from_comparison(&TradesBuy, &IncreasedByMoreThanEqual, &Value::Count(9924), true), Some(TradesBuyChangeCount));
		assert_eq!(Fact::from_comparison(&TradesBuy, &IncreasedByLessThan, &Value::Count(9924), true), Some(TradesBuyChangeCount));
		assert_eq!(Fact::from_comparison(&TradesBuy, &IncreasedByLessThanEqual, &Value::Count(9924), true), Some(TradesBuyChangeCount));
		assert_eq!(Fact::from_comparison(&TradesBuy, &DecreasedByMoreThan, &Value::Count(9924), true), Some(TradesBuyChangeCount));
		assert_eq!(Fact::from_comparison(&TradesBuy, &DecreasedByMoreThanEqual, &Value::Count(9924), true), Some(TradesBuyChangeCount));
		assert_eq!(Fact::from_comparison(&TradesBuy, &DecreasedByLessThan, &Value::Count(9924), true), Some(TradesBuyChangeCount));
		assert_eq!(Fact::from_comparison(&TradesBuy, &DecreasedByLessThanEqual, &Value::Count(9924), true), Some(TradesBuyChangeCount));
	}


	#[test]
	fn test_trades_sell_change_count() {
		assert_eq!(Fact::from_comparison(&TradesSell, &IncreasedByMoreThan, &Value::Count(9924), true), Some(TradesSellChangeCount));
		assert_eq!(Fact::from_comparison(&TradesSell, &IncreasedByMoreThanEqual, &Value::Count(9924), true), Some(TradesSellChangeCount));
		assert_eq!(Fact::from_comparison(&TradesSell, &IncreasedByLessThan, &Value::Count(9924), true), Some(TradesSellChangeCount));
		assert_eq!(Fact::from_comparison(&TradesSell, &IncreasedByLessThanEqual, &Value::Count(9924), true), Some(TradesSellChangeCount));
		assert_eq!(Fact::from_comparison(&TradesSell, &DecreasedByMoreThan, &Value::Count(9924), true), Some(TradesSellChangeCount));
		assert_eq!(Fact::from_comparison(&TradesSell, &DecreasedByMoreThanEqual, &Value::Count(9924), true), Some(TradesSellChangeCount));
		assert_eq!(Fact::from_comparison(&TradesSell, &DecreasedByLessThan, &Value::Count(9924), true), Some(TradesSellChangeCount));
		assert_eq!(Fact::from_comparison(&TradesSell, &DecreasedByLessThanEqual, &Value::Count(9924), true), Some(TradesSellChangeCount));
	}

	#[test]
	fn test_trades_change_percent() {
		assert_eq!(Fact::from_comparison(&Trades, &IncreasedByMoreThan, &Value::Percent(99.24), true), Some(TradesChangePercent));
		assert_eq!(Fact::from_comparison(&Trades, &IncreasedByMoreThanEqual, &Value::Percent(99.24), true), Some(TradesChangePercent));
		assert_eq!(Fact::from_comparison(&Trades, &IncreasedByLessThan, &Value::Percent(99.24), true), Some(TradesChangePercent));
		assert_eq!(Fact::from_comparison(&Trades, &IncreasedByLessThanEqual, &Value::Percent(99.24), true), Some(TradesChangePercent));
		assert_eq!(Fact::from_comparison(&Trades, &DecreasedByMoreThan, &Value::Percent(99.24), true), Some(TradesChangePercent));
		assert_eq!(Fact::from_comparison(&Trades, &DecreasedByMoreThanEqual, &Value::Percent(99.24), true), Some(TradesChangePercent));
		assert_eq!(Fact::from_comparison(&Trades, &DecreasedByLessThan, &Value::Percent(99.24), true), Some(TradesChangePercent));
		assert_eq!(Fact::from_comparison(&Trades, &DecreasedByLessThanEqual, &Value::Percent(99.24), true), Some(TradesChangePercent));
	}


	#[test]
	fn test_trades_buy_change_percent() {
		assert_eq!(Fact::from_comparison(&TradesBuy, &IncreasedByMoreThan, &Value::Percent(99.24), true), Some(TradesBuyChangePercent));
		assert_eq!(Fact::from_comparison(&TradesBuy, &IncreasedByMoreThanEqual, &Value::Percent(99.24), true), Some(TradesBuyChangePercent));
		assert_eq!(Fact::from_comparison(&TradesBuy, &IncreasedByLessThan, &Value::Percent(99.24), true), Some(TradesBuyChangePercent));
		assert_eq!(Fact::from_comparison(&TradesBuy, &IncreasedByLessThanEqual, &Value::Percent(99.24), true), Some(TradesBuyChangePercent));
		assert_eq!(Fact::from_comparison(&TradesBuy, &DecreasedByMoreThan, &Value::Percent(99.24), true), Some(TradesBuyChangePercent));
		assert_eq!(Fact::from_comparison(&TradesBuy, &DecreasedByMoreThanEqual, &Value::Percent(99.24), true), Some(TradesBuyChangePercent));
		assert_eq!(Fact::from_comparison(&TradesBuy, &DecreasedByLessThan, &Value::Percent(99.24), true), Some(TradesBuyChangePercent));
		assert_eq!(Fact::from_comparison(&TradesBuy, &DecreasedByLessThanEqual, &Value::Percent(99.24), true), Some(TradesBuyChangePercent));
	}

	#[test]
	fn test_trades_sell_change_percent() {
		assert_eq!(Fact::from_comparison(&TradesSell, &IncreasedByMoreThan, &Value::Percent(99.24), true), Some(TradesSellChangePercent));
		assert_eq!(Fact::from_comparison(&TradesSell, &IncreasedByMoreThanEqual, &Value::Percent(99.24), true), Some(TradesSellChangePercent));
		assert_eq!(Fact::from_comparison(&TradesSell, &IncreasedByLessThan, &Value::Percent(99.24), true), Some(TradesSellChangePercent));
		assert_eq!(Fact::from_comparison(&TradesSell, &IncreasedByLessThanEqual, &Value::Percent(99.24), true), Some(TradesSellChangePercent));
		assert_eq!(Fact::from_comparison(&TradesSell, &DecreasedByMoreThan, &Value::Percent(99.24), true), Some(TradesSellChangePercent));
		assert_eq!(Fact::from_comparison(&TradesSell, &DecreasedByMoreThanEqual, &Value::Percent(99.24), true), Some(TradesSellChangePercent));
		assert_eq!(Fact::from_comparison(&TradesSell, &DecreasedByLessThan, &Value::Percent(99.24), true), Some(TradesSellChangePercent));
		assert_eq!(Fact::from_comparison(&TradesSell, &DecreasedByLessThanEqual, &Value::Percent(99.24), true), Some(TradesSellChangePercent));
	}

	#[test]
	fn test_volume_change_quote() {
		assert_eq!(Fact::from_comparison(&Volume, &IncreasedByMoreThan, &Value::Quote(99.24), true), Some(VolumeChangeQuote));
		assert_eq!(Fact::from_comparison(&Volume, &IncreasedByMoreThanEqual, &Value::Quote(99.24), true), Some(VolumeChangeQuote));
		assert_eq!(Fact::from_comparison(&Volume, &IncreasedByLessThan, &Value::Quote(99.24), true), Some(VolumeChangeQuote));
		assert_eq!(Fact::from_comparison(&Volume, &IncreasedByLessThanEqual, &Value::Quote(99.24), true), Some(VolumeChangeQuote));
		assert_eq!(Fact::from_comparison(&Volume, &DecreasedByMoreThan, &Value::Quote(99.24), true), Some(VolumeChangeQuote));
		assert_eq!(Fact::from_comparison(&Volume, &DecreasedByMoreThanEqual, &Value::Quote(99.24), true), Some(VolumeChangeQuote));
		assert_eq!(Fact::from_comparison(&Volume, &DecreasedByLessThan, &Value::Quote(99.24), true), Some(VolumeChangeQuote));
		assert_eq!(Fact::from_comparison(&Volume, &DecreasedByLessThanEqual, &Value::Quote(99.24), true), Some(VolumeChangeQuote));
	}
}

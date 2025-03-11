// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Fact::CurveProgressPercent;
use crate::model::FactError::UnableToDeriveFact;
use crate::model::Field::PriceAvg;
use crate::model::ValueType::{Count, Duration, Percent, Quote, Usd};
use crate::model::{Condition, FactError, Field, Operator, Value, ValueType};
use serde::{Deserialize, Serialize};
use Fact::{
    CurveProgressAgeDuration, PriceAvgQuote, PriceAvgUsd, PriceQuote, PriceUsd, SwapBuyChangeCount,
    SwapBuyChangePercent, SwapBuyCount, SwapChangeCount, SwapChangePercent, SwapCount,
    SwapSellChangeCount, SwapSellChangePercent, SwapSellCount, TelegramGroup, TelegramGroupHandle,
    TwitterAccount, TwitterAccountHandle, VolumeChangeQuote,
};
use Field::{CurveProgress, CurveProgressAge, Price, SwapBuy, SwapSell, SwapTotal, Volume};
use Operator::{
    DecreasedByLessThan, DecreasedByLessThanEqual, DecreasedByMoreThan, DecreasedByMoreThanEqual,
    Equal, IncreasedByLessThan, IncreasedByLessThanEqual, IncreasedByMoreThan,
    IncreasedByMoreThanEqual, LessThan, LessThanEqual, MoreThan, MoreThanEqual,
};
use ValueType::Boolean;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Fact {
    CurveProgressPercent,
    CurveProgressAgeDuration,

    PriceQuote,
    PriceUsd,

    PriceAvgQuote,
    PriceAvgUsd,
    // PriceChangePercent,
    // PriceChangeQuote,
    // PriceChangeUsd,
    SwapCount,
    SwapBuyCount,
    SwapSellCount,

    SwapChangeCount,
    SwapBuyChangeCount,
    SwapSellChangeCount,

    SwapChangePercent,
    SwapBuyChangePercent,
    SwapSellChangePercent,

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
            CurveProgressPercent => false,
            CurveProgressAgeDuration => false,

            PriceQuote => false,
            PriceUsd => false,

            PriceAvgQuote => true,
            PriceAvgUsd => true,
            //
            // PriceChangeQuote => true,
            // PriceChangePercent => true,
            // PriceChangeUsd => true,
            SwapCount => true,
            SwapBuyCount => true,
            SwapSellCount => true,

            SwapChangeCount => true,
            SwapBuyChangeCount => true,
            SwapSellChangeCount => true,

            SwapChangePercent => true,
            SwapBuyChangePercent => true,
            SwapSellChangePercent => true,

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
            CurveProgressPercent => Percent,
            CurveProgressAgeDuration => Duration,

            PriceQuote => Quote,
            PriceUsd => Usd,

            PriceAvgQuote => Quote,
            PriceAvgUsd => Usd,
            //
            // PriceChangeQuote => Quote,
            // PriceChangeUsd => Usd,
            // PriceChangePercent => Percent,
            SwapCount => Count,
            SwapBuyCount => Count,
            SwapSellCount => Count,

            SwapChangeCount => Count,
            SwapBuyChangeCount => Count,
            SwapSellChangeCount => Count,

            SwapChangePercent => Percent,
            SwapBuyChangePercent => Percent,
            SwapSellChangePercent => Percent,

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
            } => {
                if let Some(value) = value {
                    Fact::from_comparison(field, operator, value, timeframe.is_some())
                        .ok_or(UnableToDeriveFact(condition.clone()))
                } else {
                    Err(UnableToDeriveFact(condition.clone()))
                }
            }
            Condition::Compose { .. }
            | Condition::And { .. }
            | Condition::Or { .. }
            | Condition::AndNot { .. } => Err(UnableToDeriveFact(condition.clone())),
        }
    }
}

impl Fact {
    fn from_comparison(
        field: &Field,
        operator: &Operator,
        value: &Value,
        has_timeframe: bool,
    ) -> Option<Self> {
        let fact = match (field, operator, value.value_type(), has_timeframe) {
            // CurveProgressPercent
            (CurveProgress, MoreThan, Percent, false) => CurveProgressPercent,
            (CurveProgress, MoreThanEqual, Percent, false) => CurveProgressPercent,
            (CurveProgress, LessThan, Percent, false) => CurveProgressPercent,
            (CurveProgress, LessThanEqual, Percent, false) => CurveProgressPercent,

            // CurveProgressAgeDuration
            (CurveProgressAge, MoreThan, Duration, false) => CurveProgressAgeDuration,
            (CurveProgressAge, MoreThanEqual, Duration, false) => CurveProgressAgeDuration,
            (CurveProgressAge, LessThan, Duration, false) => CurveProgressAgeDuration,
            (CurveProgressAge, LessThanEqual, Duration, false) => CurveProgressAgeDuration,

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

            // SwapCount
            (SwapTotal, MoreThan, Count, true) => SwapCount,
            (SwapTotal, MoreThanEqual, Count, true) => SwapCount,
            (SwapTotal, LessThan, Count, true) => SwapCount,
            (SwapTotal, LessThanEqual, Count, true) => SwapCount,

            // SwapBuyCount
            (SwapBuy, MoreThan, Count, true) => SwapBuyCount,
            (SwapBuy, MoreThanEqual, Count, true) => SwapBuyCount,
            (SwapBuy, LessThan, Count, true) => SwapBuyCount,
            (SwapBuy, LessThanEqual, Count, true) => SwapBuyCount,

            // SwapSellCount
            (SwapSell, MoreThan, Count, true) => SwapSellCount,
            (SwapSell, MoreThanEqual, Count, true) => SwapSellCount,
            (SwapSell, LessThan, Count, true) => SwapSellCount,
            (SwapSell, LessThanEqual, Count, true) => SwapSellCount,

            // SwapChangeCount
            (SwapTotal, IncreasedByMoreThan, Count, true) => SwapChangeCount,
            (SwapTotal, IncreasedByMoreThanEqual, Count, true) => SwapChangeCount,
            (SwapTotal, IncreasedByLessThan, Count, true) => SwapChangeCount,
            (SwapTotal, IncreasedByLessThanEqual, Count, true) => SwapChangeCount,
            (SwapTotal, DecreasedByMoreThan, Count, true) => SwapChangeCount,
            (SwapTotal, DecreasedByMoreThanEqual, Count, true) => SwapChangeCount,
            (SwapTotal, DecreasedByLessThan, Count, true) => SwapChangeCount,
            (SwapTotal, DecreasedByLessThanEqual, Count, true) => SwapChangeCount,

            // SwapBuyChangeCount
            (SwapBuy, IncreasedByMoreThan, Count, true) => SwapBuyChangeCount,
            (SwapBuy, IncreasedByMoreThanEqual, Count, true) => SwapBuyChangeCount,
            (SwapBuy, IncreasedByLessThan, Count, true) => SwapBuyChangeCount,
            (SwapBuy, IncreasedByLessThanEqual, Count, true) => SwapBuyChangeCount,
            (SwapBuy, DecreasedByMoreThan, Count, true) => SwapBuyChangeCount,
            (SwapBuy, DecreasedByMoreThanEqual, Count, true) => SwapBuyChangeCount,
            (SwapBuy, DecreasedByLessThan, Count, true) => SwapBuyChangeCount,
            (SwapBuy, DecreasedByLessThanEqual, Count, true) => SwapBuyChangeCount,

            // SwapSellChangeCount
            (SwapSell, IncreasedByMoreThan, Count, true) => SwapSellChangeCount,
            (SwapSell, IncreasedByMoreThanEqual, Count, true) => SwapSellChangeCount,
            (SwapSell, IncreasedByLessThan, Count, true) => SwapSellChangeCount,
            (SwapSell, IncreasedByLessThanEqual, Count, true) => SwapSellChangeCount,
            (SwapSell, DecreasedByMoreThan, Count, true) => SwapSellChangeCount,
            (SwapSell, DecreasedByMoreThanEqual, Count, true) => SwapSellChangeCount,
            (SwapSell, DecreasedByLessThan, Count, true) => SwapSellChangeCount,
            (SwapSell, DecreasedByLessThanEqual, Count, true) => SwapSellChangeCount,

            // SwapChangePercent
            (SwapTotal, IncreasedByMoreThan, Percent, true) => SwapChangePercent,
            (SwapTotal, IncreasedByMoreThanEqual, Percent, true) => SwapChangePercent,
            (SwapTotal, IncreasedByLessThan, Percent, true) => SwapChangePercent,
            (SwapTotal, IncreasedByLessThanEqual, Percent, true) => SwapChangePercent,
            (SwapTotal, DecreasedByMoreThan, Percent, true) => SwapChangePercent,
            (SwapTotal, DecreasedByMoreThanEqual, Percent, true) => SwapChangePercent,
            (SwapTotal, DecreasedByLessThan, Percent, true) => SwapChangePercent,
            (SwapTotal, DecreasedByLessThanEqual, Percent, true) => SwapChangePercent,

            // SwapBuyChangePercent
            (SwapBuy, IncreasedByMoreThan, Percent, true) => SwapBuyChangePercent,
            (SwapBuy, IncreasedByMoreThanEqual, Percent, true) => SwapBuyChangePercent,
            (SwapBuy, IncreasedByLessThan, Percent, true) => SwapBuyChangePercent,
            (SwapBuy, IncreasedByLessThanEqual, Percent, true) => SwapBuyChangePercent,
            (SwapBuy, DecreasedByMoreThan, Percent, true) => SwapBuyChangePercent,
            (SwapBuy, DecreasedByMoreThanEqual, Percent, true) => SwapBuyChangePercent,
            (SwapBuy, DecreasedByLessThan, Percent, true) => SwapBuyChangePercent,
            (SwapBuy, DecreasedByLessThanEqual, Percent, true) => SwapBuyChangePercent,

            // SwapSellChangePercent
            (SwapSell, IncreasedByMoreThan, Percent, true) => SwapSellChangePercent,
            (SwapSell, IncreasedByMoreThanEqual, Percent, true) => SwapSellChangePercent,
            (SwapSell, IncreasedByLessThan, Percent, true) => SwapSellChangePercent,
            (SwapSell, IncreasedByLessThanEqual, Percent, true) => SwapSellChangePercent,
            (SwapSell, DecreasedByMoreThan, Percent, true) => SwapSellChangePercent,
            (SwapSell, DecreasedByMoreThanEqual, Percent, true) => SwapSellChangePercent,
            (SwapSell, DecreasedByLessThan, Percent, true) => SwapSellChangePercent,
            (SwapSell, DecreasedByLessThanEqual, Percent, true) => SwapSellChangePercent,

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
    use common::model::TimeUnit::Minute;

    #[test]
    fn test_curve_progress() {
        assert_eq!(
            Fact::from_comparison(&CurveProgress, &MoreThan, &Value::percent(99.24), false),
            Some(CurveProgressPercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &CurveProgress,
                &MoreThanEqual,
                &Value::percent(99.24),
                false
            ),
            Some(CurveProgressPercent)
        );

        assert_eq!(
            Fact::from_comparison(&CurveProgress, &LessThan, &Value::percent(99.24), false),
            Some(CurveProgressPercent)
        );

        assert_eq!(
            Fact::from_comparison(
                &CurveProgress,
                &LessThanEqual,
                &Value::percent(99.24),
                false
            ),
            Some(CurveProgressPercent)
        );
    }

    #[test]
    fn test_curve_progress_age() {
        assert_eq!(
            Fact::from_comparison(
                &CurveProgressAge,
                &MoreThan,
                &Value::duration(1, Minute),
                false
            ),
            Some(CurveProgressAgeDuration)
        );
        assert_eq!(
            Fact::from_comparison(
                &CurveProgressAge,
                &MoreThanEqual,
                &Value::duration(1, Minute),
                false
            ),
            Some(CurveProgressAgeDuration)
        );

        assert_eq!(
            Fact::from_comparison(
                &CurveProgressAge,
                &LessThan,
                &Value::duration(1, Minute),
                false
            ),
            Some(CurveProgressAgeDuration)
        );

        assert_eq!(
            Fact::from_comparison(
                &CurveProgressAge,
                &LessThanEqual,
                &Value::duration(1, Minute),
                false
            ),
            Some(CurveProgressAgeDuration)
        );
    }

    #[test]
    fn test_price_quote() {
        assert_eq!(
            Fact::from_comparison(&Price, &MoreThan, &Value::quote(99), false),
            Some(PriceQuote)
        );
        assert_eq!(
            Fact::from_comparison(&Price, &MoreThanEqual, &Value::quote(99), false),
            Some(PriceQuote)
        );
        assert_eq!(
            Fact::from_comparison(&Price, &LessThan, &Value::quote(99), false),
            Some(PriceQuote)
        );
        assert_eq!(
            Fact::from_comparison(&Price, &LessThanEqual, &Value::quote(99), false),
            Some(PriceQuote)
        );
    }

    #[test]
    fn test_price_usd() {
        assert_eq!(
            Fact::from_comparison(&Price, &MoreThan, &Value::usd(99), false),
            Some(PriceUsd)
        );
        assert_eq!(
            Fact::from_comparison(&Price, &MoreThanEqual, &Value::usd(99), false),
            Some(PriceUsd)
        );
        assert_eq!(
            Fact::from_comparison(&Price, &LessThan, &Value::usd(99), false),
            Some(PriceUsd)
        );
        assert_eq!(
            Fact::from_comparison(&Price, &LessThanEqual, &Value::usd(99), false),
            Some(PriceUsd)
        );
    }

    #[test]
    fn test_price_avg_quote() {
        assert_eq!(
            Fact::from_comparison(&PriceAvg, &MoreThan, &Value::quote(99), true),
            Some(PriceAvgQuote)
        );
        assert_eq!(
            Fact::from_comparison(&PriceAvg, &MoreThanEqual, &Value::quote(99), true),
            Some(PriceAvgQuote)
        );
        assert_eq!(
            Fact::from_comparison(&PriceAvg, &LessThan, &Value::quote(99), true),
            Some(PriceAvgQuote)
        );
        assert_eq!(
            Fact::from_comparison(&PriceAvg, &LessThanEqual, &Value::quote(99), true),
            Some(PriceAvgQuote)
        );
    }

    #[test]
    fn test_price_avg_usd() {
        assert_eq!(
            Fact::from_comparison(&PriceAvg, &MoreThan, &Value::usd(99), true),
            Some(PriceAvgUsd)
        );
        assert_eq!(
            Fact::from_comparison(&PriceAvg, &MoreThanEqual, &Value::usd(99), true),
            Some(PriceAvgUsd)
        );
        assert_eq!(
            Fact::from_comparison(&PriceAvg, &LessThan, &Value::usd(99), true),
            Some(PriceAvgUsd)
        );
        assert_eq!(
            Fact::from_comparison(&PriceAvg, &LessThanEqual, &Value::usd(99), true),
            Some(PriceAvgUsd)
        );
    }

    #[test]
    fn test_swaps_count() {
        assert_eq!(
            Fact::from_comparison(&SwapTotal, &MoreThan, &Value::count(9924), true),
            Some(SwapCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapTotal, &MoreThanEqual, &Value::count(9924), true),
            Some(SwapCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapTotal, &LessThan, &Value::count(9924), true),
            Some(SwapCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapTotal, &LessThanEqual, &Value::count(9924), true),
            Some(SwapCount)
        );
    }

    #[test]
    fn test_swaps_buy_count() {
        assert_eq!(
            Fact::from_comparison(&SwapBuy, &MoreThan, &Value::count(9924), true),
            Some(SwapBuyCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapBuy, &MoreThanEqual, &Value::count(9924), true),
            Some(SwapBuyCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapBuy, &LessThan, &Value::count(9924), true),
            Some(SwapBuyCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapBuy, &LessThanEqual, &Value::count(9924), true),
            Some(SwapBuyCount)
        );
    }

    #[test]
    fn test_swaps_sell_count() {
        assert_eq!(
            Fact::from_comparison(&SwapSell, &MoreThan, &Value::count(9924), true),
            Some(SwapSellCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapSell, &MoreThanEqual, &Value::count(9924), true),
            Some(SwapSellCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapSell, &LessThan, &Value::count(9924), true),
            Some(SwapSellCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapSell, &LessThanEqual, &Value::count(9924), true),
            Some(SwapSellCount)
        );
    }

    #[test]
    fn test_swaps_change_count() {
        assert_eq!(
            Fact::from_comparison(&SwapTotal, &IncreasedByMoreThan, &Value::count(9924), true),
            Some(SwapChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapTotal,
                &IncreasedByMoreThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapTotal, &IncreasedByLessThan, &Value::count(9924), true),
            Some(SwapChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapTotal,
                &IncreasedByLessThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapTotal, &DecreasedByMoreThan, &Value::count(9924), true),
            Some(SwapChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapTotal,
                &DecreasedByMoreThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapTotal, &DecreasedByLessThan, &Value::count(9924), true),
            Some(SwapChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapTotal,
                &DecreasedByLessThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapChangeCount)
        );
    }

    #[test]
    fn test_swaps_buy_change_count() {
        assert_eq!(
            Fact::from_comparison(&SwapBuy, &IncreasedByMoreThan, &Value::count(9924), true),
            Some(SwapBuyChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapBuy,
                &IncreasedByMoreThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapBuyChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapBuy, &IncreasedByLessThan, &Value::count(9924), true),
            Some(SwapBuyChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapBuy,
                &IncreasedByLessThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapBuyChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapBuy, &DecreasedByMoreThan, &Value::count(9924), true),
            Some(SwapBuyChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapBuy,
                &DecreasedByMoreThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapBuyChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapBuy, &DecreasedByLessThan, &Value::count(9924), true),
            Some(SwapBuyChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapBuy,
                &DecreasedByLessThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapBuyChangeCount)
        );
    }

    #[test]
    fn test_swaps_sell_change_count() {
        assert_eq!(
            Fact::from_comparison(&SwapSell, &IncreasedByMoreThan, &Value::count(9924), true),
            Some(SwapSellChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapSell,
                &IncreasedByMoreThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapSellChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapSell, &IncreasedByLessThan, &Value::count(9924), true),
            Some(SwapSellChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapSell,
                &IncreasedByLessThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapSellChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapSell, &DecreasedByMoreThan, &Value::count(9924), true),
            Some(SwapSellChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapSell,
                &DecreasedByMoreThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapSellChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapSell, &DecreasedByLessThan, &Value::count(9924), true),
            Some(SwapSellChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapSell,
                &DecreasedByLessThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapSellChangeCount)
        );
    }

    #[test]
    fn test_swaps_percent() {
        assert_eq!(
            Fact::from_comparison(
                &SwapTotal,
                &IncreasedByMoreThan,
                &Value::percent(99.24),
                true
            ),
            Some(SwapChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapTotal,
                &IncreasedByMoreThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapTotal,
                &IncreasedByLessThan,
                &Value::percent(99.24),
                true
            ),
            Some(SwapChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapTotal,
                &IncreasedByLessThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapTotal,
                &DecreasedByMoreThan,
                &Value::percent(99.24),
                true
            ),
            Some(SwapChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapTotal,
                &DecreasedByMoreThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapTotal,
                &DecreasedByLessThan,
                &Value::percent(99.24),
                true
            ),
            Some(SwapChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapTotal,
                &DecreasedByLessThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapChangePercent)
        );
    }

    #[test]
    fn test_swaps_buy_percent() {
        assert_eq!(
            Fact::from_comparison(&SwapBuy, &IncreasedByMoreThan, &Value::percent(99.24), true),
            Some(SwapBuyChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapBuy,
                &IncreasedByMoreThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapBuyChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(&SwapBuy, &IncreasedByLessThan, &Value::percent(99.24), true),
            Some(SwapBuyChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapBuy,
                &IncreasedByLessThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapBuyChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(&SwapBuy, &DecreasedByMoreThan, &Value::percent(99.24), true),
            Some(SwapBuyChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapBuy,
                &DecreasedByMoreThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapBuyChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(&SwapBuy, &DecreasedByLessThan, &Value::percent(99.24), true),
            Some(SwapBuyChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapBuy,
                &DecreasedByLessThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapBuyChangePercent)
        );
    }

    #[test]
    fn test_swaps_sell_percent() {
        assert_eq!(
            Fact::from_comparison(
                &SwapSell,
                &IncreasedByMoreThan,
                &Value::percent(99.24),
                true
            ),
            Some(SwapSellChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapSell,
                &IncreasedByMoreThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapSellChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapSell,
                &IncreasedByLessThan,
                &Value::percent(99.24),
                true
            ),
            Some(SwapSellChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapSell,
                &IncreasedByLessThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapSellChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapSell,
                &DecreasedByMoreThan,
                &Value::percent(99.24),
                true
            ),
            Some(SwapSellChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapSell,
                &DecreasedByMoreThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapSellChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapSell,
                &DecreasedByLessThan,
                &Value::percent(99.24),
                true
            ),
            Some(SwapSellChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapSell,
                &DecreasedByLessThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapSellChangePercent)
        );
    }

    #[test]
    fn test_volume_change_quote() {
        assert_eq!(
            Fact::from_comparison(&Volume, &IncreasedByMoreThan, &Value::quote(99), true),
            Some(VolumeChangeQuote)
        );
        assert_eq!(
            Fact::from_comparison(&Volume, &IncreasedByMoreThanEqual, &Value::quote(99), true),
            Some(VolumeChangeQuote)
        );
        assert_eq!(
            Fact::from_comparison(&Volume, &IncreasedByLessThan, &Value::quote(99), true),
            Some(VolumeChangeQuote)
        );
        assert_eq!(
            Fact::from_comparison(&Volume, &IncreasedByLessThanEqual, &Value::quote(99), true),
            Some(VolumeChangeQuote)
        );
        assert_eq!(
            Fact::from_comparison(&Volume, &DecreasedByMoreThan, &Value::quote(99), true),
            Some(VolumeChangeQuote)
        );
        assert_eq!(
            Fact::from_comparison(&Volume, &DecreasedByMoreThanEqual, &Value::quote(99), true),
            Some(VolumeChangeQuote)
        );
        assert_eq!(
            Fact::from_comparison(&Volume, &DecreasedByLessThan, &Value::quote(99), true),
            Some(VolumeChangeQuote)
        );
        assert_eq!(
            Fact::from_comparison(&Volume, &DecreasedByLessThanEqual, &Value::quote(99), true),
            Some(VolumeChangeQuote)
        );
    }
}

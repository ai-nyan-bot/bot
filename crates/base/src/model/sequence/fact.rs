// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Fact::CurveProgressPercent;
use crate::model::FactError::UnableToDeriveFact;
use crate::model::Field::PriceAvg;
use crate::model::ValueType::{Count, Duration, Percent, Quote, Usd};
use crate::model::{Condition, FactError, Field, Operator, Value, ValueType};
use serde::{Deserialize, Serialize};
use Fact::{
    CurveProgressAgeDuration, PriceAvgQuote, PriceAvgUsd, PriceQuote, PriceUsd, TelegramGroup,
    TelegramGroupHandle, SwapsBuyChangeCount, SwapsBuyChangePercent, SwapsBuyCount,
    SwapsChangeCount, SwapsChangePercent, SwapsCount, SwapsSellChangeCount,
    SwapsSellChangePercent, SwapsSellCount, TwitterAccount, TwitterAccountHandle,
    VolumeChangeQuote,
};
use Field::{CurveProgress, CurveProgressAge, Price, Swaps, SwapsBuy, SwapsSell, Volume};
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
    SwapsCount,
    SwapsBuyCount,
    SwapsSellCount,

    SwapsChangeCount,
    SwapsBuyChangeCount,
    SwapsSellChangeCount,

    SwapsChangePercent,
    SwapsBuyChangePercent,
    SwapsSellChangePercent,

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
            SwapsCount => true,
            SwapsBuyCount => true,
            SwapsSellCount => true,

            SwapsChangeCount => true,
            SwapsBuyChangeCount => true,
            SwapsSellChangeCount => true,

            SwapsChangePercent => true,
            SwapsBuyChangePercent => true,
            SwapsSellChangePercent => true,

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
            SwapsCount => Count,
            SwapsBuyCount => Count,
            SwapsSellCount => Count,

            SwapsChangeCount => Count,
            SwapsBuyChangeCount => Count,
            SwapsSellChangeCount => Count,

            SwapsChangePercent => Percent,
            SwapsBuyChangePercent => Percent,
            SwapsSellChangePercent => Percent,

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
            } => Fact::from_comparison(field, operator, value, timeframe.is_some())
                .ok_or(UnableToDeriveFact(condition.clone())),
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

            // SwapsCount
            (Swaps, MoreThan, Count, true) => SwapsCount,
            (Swaps, MoreThanEqual, Count, true) => SwapsCount,
            (Swaps, LessThan, Count, true) => SwapsCount,
            (Swaps, LessThanEqual, Count, true) => SwapsCount,

            // SwapsBuyCount
            (SwapsBuy, MoreThan, Count, true) => SwapsBuyCount,
            (SwapsBuy, MoreThanEqual, Count, true) => SwapsBuyCount,
            (SwapsBuy, LessThan, Count, true) => SwapsBuyCount,
            (SwapsBuy, LessThanEqual, Count, true) => SwapsBuyCount,

            // SwapsSellCount
            (SwapsSell, MoreThan, Count, true) => SwapsSellCount,
            (SwapsSell, MoreThanEqual, Count, true) => SwapsSellCount,
            (SwapsSell, LessThan, Count, true) => SwapsSellCount,
            (SwapsSell, LessThanEqual, Count, true) => SwapsSellCount,

            // SwapsChangeCount
            (Swaps, IncreasedByMoreThan, Count, true) => SwapsChangeCount,
            (Swaps, IncreasedByMoreThanEqual, Count, true) => SwapsChangeCount,
            (Swaps, IncreasedByLessThan, Count, true) => SwapsChangeCount,
            (Swaps, IncreasedByLessThanEqual, Count, true) => SwapsChangeCount,
            (Swaps, DecreasedByMoreThan, Count, true) => SwapsChangeCount,
            (Swaps, DecreasedByMoreThanEqual, Count, true) => SwapsChangeCount,
            (Swaps, DecreasedByLessThan, Count, true) => SwapsChangeCount,
            (Swaps, DecreasedByLessThanEqual, Count, true) => SwapsChangeCount,

            // SwapsBuyChangeCount
            (SwapsBuy, IncreasedByMoreThan, Count, true) => SwapsBuyChangeCount,
            (SwapsBuy, IncreasedByMoreThanEqual, Count, true) => SwapsBuyChangeCount,
            (SwapsBuy, IncreasedByLessThan, Count, true) => SwapsBuyChangeCount,
            (SwapsBuy, IncreasedByLessThanEqual, Count, true) => SwapsBuyChangeCount,
            (SwapsBuy, DecreasedByMoreThan, Count, true) => SwapsBuyChangeCount,
            (SwapsBuy, DecreasedByMoreThanEqual, Count, true) => SwapsBuyChangeCount,
            (SwapsBuy, DecreasedByLessThan, Count, true) => SwapsBuyChangeCount,
            (SwapsBuy, DecreasedByLessThanEqual, Count, true) => SwapsBuyChangeCount,

            // SwapsSellChangeCount
            (SwapsSell, IncreasedByMoreThan, Count, true) => SwapsSellChangeCount,
            (SwapsSell, IncreasedByMoreThanEqual, Count, true) => SwapsSellChangeCount,
            (SwapsSell, IncreasedByLessThan, Count, true) => SwapsSellChangeCount,
            (SwapsSell, IncreasedByLessThanEqual, Count, true) => SwapsSellChangeCount,
            (SwapsSell, DecreasedByMoreThan, Count, true) => SwapsSellChangeCount,
            (SwapsSell, DecreasedByMoreThanEqual, Count, true) => SwapsSellChangeCount,
            (SwapsSell, DecreasedByLessThan, Count, true) => SwapsSellChangeCount,
            (SwapsSell, DecreasedByLessThanEqual, Count, true) => SwapsSellChangeCount,

            // SwapsChangePercent
            (Swaps, IncreasedByMoreThan, Percent, true) => SwapsChangePercent,
            (Swaps, IncreasedByMoreThanEqual, Percent, true) => SwapsChangePercent,
            (Swaps, IncreasedByLessThan, Percent, true) => SwapsChangePercent,
            (Swaps, IncreasedByLessThanEqual, Percent, true) => SwapsChangePercent,
            (Swaps, DecreasedByMoreThan, Percent, true) => SwapsChangePercent,
            (Swaps, DecreasedByMoreThanEqual, Percent, true) => SwapsChangePercent,
            (Swaps, DecreasedByLessThan, Percent, true) => SwapsChangePercent,
            (Swaps, DecreasedByLessThanEqual, Percent, true) => SwapsChangePercent,

            // SwapsBuyChangePercent
            (SwapsBuy, IncreasedByMoreThan, Percent, true) => SwapsBuyChangePercent,
            (SwapsBuy, IncreasedByMoreThanEqual, Percent, true) => SwapsBuyChangePercent,
            (SwapsBuy, IncreasedByLessThan, Percent, true) => SwapsBuyChangePercent,
            (SwapsBuy, IncreasedByLessThanEqual, Percent, true) => SwapsBuyChangePercent,
            (SwapsBuy, DecreasedByMoreThan, Percent, true) => SwapsBuyChangePercent,
            (SwapsBuy, DecreasedByMoreThanEqual, Percent, true) => SwapsBuyChangePercent,
            (SwapsBuy, DecreasedByLessThan, Percent, true) => SwapsBuyChangePercent,
            (SwapsBuy, DecreasedByLessThanEqual, Percent, true) => SwapsBuyChangePercent,

            // SwapsSellChangePercent
            (SwapsSell, IncreasedByMoreThan, Percent, true) => SwapsSellChangePercent,
            (SwapsSell, IncreasedByMoreThanEqual, Percent, true) => SwapsSellChangePercent,
            (SwapsSell, IncreasedByLessThan, Percent, true) => SwapsSellChangePercent,
            (SwapsSell, IncreasedByLessThanEqual, Percent, true) => SwapsSellChangePercent,
            (SwapsSell, DecreasedByMoreThan, Percent, true) => SwapsSellChangePercent,
            (SwapsSell, DecreasedByMoreThanEqual, Percent, true) => SwapsSellChangePercent,
            (SwapsSell, DecreasedByLessThan, Percent, true) => SwapsSellChangePercent,
            (SwapsSell, DecreasedByLessThanEqual, Percent, true) => SwapsSellChangePercent,

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
            Fact::from_comparison(&Swaps, &MoreThan, &Value::count(9924), true),
            Some(SwapsCount)
        );
        assert_eq!(
            Fact::from_comparison(&Swaps, &MoreThanEqual, &Value::count(9924), true),
            Some(SwapsCount)
        );
        assert_eq!(
            Fact::from_comparison(&Swaps, &LessThan, &Value::count(9924), true),
            Some(SwapsCount)
        );
        assert_eq!(
            Fact::from_comparison(&Swaps, &LessThanEqual, &Value::count(9924), true),
            Some(SwapsCount)
        );
    }

    #[test]
    fn test_swaps_buy_count() {
        assert_eq!(
            Fact::from_comparison(&SwapsBuy, &MoreThan, &Value::count(9924), true),
            Some(SwapsBuyCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapsBuy, &MoreThanEqual, &Value::count(9924), true),
            Some(SwapsBuyCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapsBuy, &LessThan, &Value::count(9924), true),
            Some(SwapsBuyCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapsBuy, &LessThanEqual, &Value::count(9924), true),
            Some(SwapsBuyCount)
        );
    }

    #[test]
    fn test_swaps_sell_count() {
        assert_eq!(
            Fact::from_comparison(&SwapsSell, &MoreThan, &Value::count(9924), true),
            Some(SwapsSellCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapsSell, &MoreThanEqual, &Value::count(9924), true),
            Some(SwapsSellCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapsSell, &LessThan, &Value::count(9924), true),
            Some(SwapsSellCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapsSell, &LessThanEqual, &Value::count(9924), true),
            Some(SwapsSellCount)
        );
    }

    #[test]
    fn test_swaps_change_count() {
        assert_eq!(
            Fact::from_comparison(&Swaps, &IncreasedByMoreThan, &Value::count(9924), true),
            Some(SwapsChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(
                &Swaps,
                &IncreasedByMoreThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapsChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(&Swaps, &IncreasedByLessThan, &Value::count(9924), true),
            Some(SwapsChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(
                &Swaps,
                &IncreasedByLessThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapsChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(&Swaps, &DecreasedByMoreThan, &Value::count(9924), true),
            Some(SwapsChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(
                &Swaps,
                &DecreasedByMoreThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapsChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(&Swaps, &DecreasedByLessThan, &Value::count(9924), true),
            Some(SwapsChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(
                &Swaps,
                &DecreasedByLessThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapsChangeCount)
        );
    }

    #[test]
    fn test_swaps_buy_change_count() {
        assert_eq!(
            Fact::from_comparison(&SwapsBuy, &IncreasedByMoreThan, &Value::count(9924), true),
            Some(SwapsBuyChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapsBuy,
                &IncreasedByMoreThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapsBuyChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapsBuy, &IncreasedByLessThan, &Value::count(9924), true),
            Some(SwapsBuyChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapsBuy,
                &IncreasedByLessThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapsBuyChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapsBuy, &DecreasedByMoreThan, &Value::count(9924), true),
            Some(SwapsBuyChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapsBuy,
                &DecreasedByMoreThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapsBuyChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapsBuy, &DecreasedByLessThan, &Value::count(9924), true),
            Some(SwapsBuyChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapsBuy,
                &DecreasedByLessThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapsBuyChangeCount)
        );
    }

    #[test]
    fn test_swaps_sell_change_count() {
        assert_eq!(
            Fact::from_comparison(&SwapsSell, &IncreasedByMoreThan, &Value::count(9924), true),
            Some(SwapsSellChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapsSell,
                &IncreasedByMoreThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapsSellChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapsSell, &IncreasedByLessThan, &Value::count(9924), true),
            Some(SwapsSellChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapsSell,
                &IncreasedByLessThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapsSellChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapsSell, &DecreasedByMoreThan, &Value::count(9924), true),
            Some(SwapsSellChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapsSell,
                &DecreasedByMoreThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapsSellChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(&SwapsSell, &DecreasedByLessThan, &Value::count(9924), true),
            Some(SwapsSellChangeCount)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapsSell,
                &DecreasedByLessThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapsSellChangeCount)
        );
    }

    #[test]
    fn test_swaps_percent() {
        assert_eq!(
            Fact::from_comparison(&Swaps, &IncreasedByMoreThan, &Value::percent(99.24), true),
            Some(SwapsChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &Swaps,
                &IncreasedByMoreThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapsChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(&Swaps, &IncreasedByLessThan, &Value::percent(99.24), true),
            Some(SwapsChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &Swaps,
                &IncreasedByLessThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapsChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(&Swaps, &DecreasedByMoreThan, &Value::percent(99.24), true),
            Some(SwapsChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &Swaps,
                &DecreasedByMoreThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapsChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(&Swaps, &DecreasedByLessThan, &Value::percent(99.24), true),
            Some(SwapsChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &Swaps,
                &DecreasedByLessThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapsChangePercent)
        );
    }

    #[test]
    fn test_swaps_buy_percent() {
        assert_eq!(
            Fact::from_comparison(
                &SwapsBuy,
                &IncreasedByMoreThan,
                &Value::percent(99.24),
                true
            ),
            Some(SwapsBuyChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapsBuy,
                &IncreasedByMoreThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapsBuyChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapsBuy,
                &IncreasedByLessThan,
                &Value::percent(99.24),
                true
            ),
            Some(SwapsBuyChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapsBuy,
                &IncreasedByLessThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapsBuyChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapsBuy,
                &DecreasedByMoreThan,
                &Value::percent(99.24),
                true
            ),
            Some(SwapsBuyChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapsBuy,
                &DecreasedByMoreThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapsBuyChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapsBuy,
                &DecreasedByLessThan,
                &Value::percent(99.24),
                true
            ),
            Some(SwapsBuyChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapsBuy,
                &DecreasedByLessThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapsBuyChangePercent)
        );
    }

    #[test]
    fn test_swaps_sell_percent() {
        assert_eq!(
            Fact::from_comparison(
                &SwapsSell,
                &IncreasedByMoreThan,
                &Value::percent(99.24),
                true
            ),
            Some(SwapsSellChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapsSell,
                &IncreasedByMoreThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapsSellChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapsSell,
                &IncreasedByLessThan,
                &Value::percent(99.24),
                true
            ),
            Some(SwapsSellChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapsSell,
                &IncreasedByLessThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapsSellChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapsSell,
                &DecreasedByMoreThan,
                &Value::percent(99.24),
                true
            ),
            Some(SwapsSellChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapsSell,
                &DecreasedByMoreThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapsSellChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapsSell,
                &DecreasedByLessThan,
                &Value::percent(99.24),
                true
            ),
            Some(SwapsSellChangePercent)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapsSell,
                &DecreasedByLessThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapsSellChangePercent)
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

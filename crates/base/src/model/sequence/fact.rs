// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Fact::{CurveProgressPercent, MarketCapSolAggregate, MarketCapUsdAggregate};
use crate::model::FactError::UnableToDeriveFact;
use crate::model::Field::{AgeQuote, PriceAvg};
use crate::model::ValueType::{Count, Duration, Percent, Quote, Sol, Usd};
use crate::model::{Condition, FactError, Field, Operator, Value, ValueType};
use serde::{Deserialize, Serialize};
use Fact::{
    AgeBaseDuration, AgeQuoteDuration, CurveProgressAgeDuration, MarketCapQuote,
    MarketCapQuoteAggregate, MarketCapSol, MarketCapUsd, PriceAvgQuoteAggregate,
    PriceAvgUsdAggregate, PriceQuote, PriceUsd, SwapAllChangeAggregate, SwapAllCountAggregate,
    SwapAllPercentAggregate, SwapBuyChangeAggregate, SwapBuyCountAggregate,
    SwapBuyPercentAggregate, SwapSellChangeAggregate, SwapSellCountAggregate,
    SwapSellPercentAggregate, TelegramGroup, TelegramGroupHandle, TwitterAccount,
    TwitterAccountHandle, VenueJupiter, VenuePumpfun, VolumeChangeQuoteAggregate,
};
use Field::{AgeBase, CurveProgress, CurveProgressAge, MarketCap, Price, SwapAll, SwapBuy, SwapSell, Volume};
use Operator::{
    DecreasedByLessThan, DecreasedByLessThanEqual, DecreasedByMoreThan, DecreasedByMoreThanEqual,
    Equal, IncreasedByLessThan, IncreasedByLessThanEqual, IncreasedByMoreThan,
    IncreasedByMoreThanEqual, LessThan, LessThanEqual, MoreThan, MoreThanEqual, NotEqual,
};
use ValueType::Boolean;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Fact {
    /// Age of the base token
    AgeBaseDuration,
    /// Age of the quote token
    AgeQuoteDuration,

    CurveProgressPercent,
    /// Duration since last update of curve progress
    CurveProgressAgeDuration,

    MarketCapQuote,
    MarketCapSol,
    MarketCapUsd,

    MarketCapQuoteAggregate,
    MarketCapSolAggregate,
    MarketCapUsdAggregate,

    PriceQuote,
    PriceUsd,

    PriceAvgQuoteAggregate,
    PriceAvgUsdAggregate,

    SwapAllCountAggregate,
    SwapBuyCountAggregate,
    SwapSellCountAggregate,

    SwapAllChangeAggregate,
    SwapBuyChangeAggregate,
    SwapSellChangeAggregate,

    SwapAllPercentAggregate,
    SwapBuyPercentAggregate,
    SwapSellPercentAggregate,

    VolumeChangeQuoteAggregate,

    TelegramGroup,
    TelegramGroupHandle,

    TwitterAccount,
    TwitterAccountHandle,

    VenuePumpfun,
    VenueJupiter,
}

impl Fact {
    pub fn has_timeframe(&self) -> bool {
        match self {
            AgeBaseDuration => false,
            AgeQuoteDuration => false,

            CurveProgressPercent => false,
            CurveProgressAgeDuration => false,

            MarketCapQuote => false,
            MarketCapSol => false,
            MarketCapUsd => false,

            MarketCapQuoteAggregate => true,
            MarketCapSolAggregate => true,
            MarketCapUsdAggregate => true,

            PriceQuote => false,
            PriceUsd => false,

            PriceAvgQuoteAggregate => true,
            PriceAvgUsdAggregate => true,

            //
            // PriceChangeQuote => true,
            // PriceChangePercent => true,
            // PriceChangeUsd => true,
            SwapAllCountAggregate => true,
            SwapBuyCountAggregate => true,
            SwapSellCountAggregate => true,

            SwapAllChangeAggregate => true,
            SwapBuyChangeAggregate => true,
            SwapSellChangeAggregate => true,

            SwapAllPercentAggregate => true,
            SwapBuyPercentAggregate => true,
            SwapSellPercentAggregate => true,

            // VolumeQuote => false,
            // VolumeUsd => false,
            VolumeChangeQuoteAggregate => true,
            // VolumeChangePercent => true,
            // VolumeChangeUsd => true,
            TelegramGroup => false,
            TelegramGroupHandle => false,

            TwitterAccount => false,
            TwitterAccountHandle => false,

            VenuePumpfun => false,
            VenueJupiter => false,
        }
    }

    pub fn value_type(&self) -> ValueType {
        match self {
            AgeBaseDuration => Duration,
            AgeQuoteDuration => Duration,

            CurveProgressPercent => Percent,
            CurveProgressAgeDuration => Duration,

            MarketCapQuote => Quote,
            MarketCapSol => Sol,
            MarketCapUsd => Usd,

            MarketCapQuoteAggregate => Quote,
            MarketCapSolAggregate => Sol,
            MarketCapUsdAggregate => Usd,

            PriceQuote => Quote,
            PriceUsd => Usd,

            PriceAvgQuoteAggregate => Quote,
            PriceAvgUsdAggregate => Usd,
            //
            // PriceChangeQuote => Quote,
            // PriceChangeUsd => Usd,
            // PriceChangePercent => Percent,
            SwapAllCountAggregate => Count,
            SwapBuyCountAggregate => Count,
            SwapSellCountAggregate => Count,

            SwapAllChangeAggregate => Count,
            SwapBuyChangeAggregate => Count,
            SwapSellChangeAggregate => Count,

            SwapAllPercentAggregate => Percent,
            SwapBuyPercentAggregate => Percent,
            SwapSellPercentAggregate => Percent,

            // VolumeQuote => Quote,
            // VolumeUsd => Usd,
            VolumeChangeQuoteAggregate => Quote,
            // VolumeChangePercent => Percent,
            // VolumeChangeUsd => Usd,
            TelegramGroup => Boolean,
            TelegramGroupHandle => ValueType::String,
            TwitterAccount => Boolean,
            TwitterAccountHandle => ValueType::String,

            VenuePumpfun => Boolean,
            VenueJupiter => Boolean,
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
            // AgeBaseDuration
            (AgeBase, MoreThan, Duration, false) => AgeBaseDuration,
            (AgeBase, MoreThanEqual, Duration, false) => AgeBaseDuration,
            (AgeBase, LessThan, Duration, false) => AgeBaseDuration,
            (AgeBase, LessThanEqual, Duration, false) => AgeBaseDuration,

            // AgeQuoteDuration
            (AgeQuote, MoreThan, Duration, false) => AgeQuoteDuration,
            (AgeQuote, MoreThanEqual, Duration, false) => AgeQuoteDuration,
            (AgeQuote, LessThan, Duration, false) => AgeQuoteDuration,
            (AgeQuote, LessThanEqual, Duration, false) => AgeQuoteDuration,

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

            // MarketCapQuote
            (MarketCap, MoreThan, Quote, false) => MarketCapQuote,
            (MarketCap, MoreThanEqual, Quote, false) => MarketCapQuote,
            (MarketCap, LessThan, Quote, false) => MarketCapQuote,
            (MarketCap, LessThanEqual, Quote, false) => MarketCapQuote,

            // MarketCapSol
            (MarketCap, MoreThan, Sol, false) => MarketCapSol,
            (MarketCap, MoreThanEqual, Sol, false) => MarketCapSol,
            (MarketCap, LessThan, Sol, false) => MarketCapSol,
            (MarketCap, LessThanEqual, Sol, false) => MarketCapSol,

            // MarketCapUsd
            (MarketCap, MoreThan, Usd, false) => MarketCapUsd,
            (MarketCap, MoreThanEqual, Usd, false) => MarketCapUsd,
            (MarketCap, LessThan, Usd, false) => MarketCapUsd,
            (MarketCap, LessThanEqual, Usd, false) => MarketCapUsd,

            // MarketCapQuoteAggregate
            (MarketCap, MoreThan, Quote, true) => MarketCapQuoteAggregate,
            (MarketCap, MoreThanEqual, Quote, true) => MarketCapQuoteAggregate,
            (MarketCap, LessThan, Quote, true) => MarketCapQuoteAggregate,
            (MarketCap, LessThanEqual, Quote, true) => MarketCapQuoteAggregate,

            // MarketCapSolAggregate
            (MarketCap, MoreThan, Sol, true) => MarketCapSolAggregate,
            (MarketCap, MoreThanEqual, Sol, true) => MarketCapSolAggregate,
            (MarketCap, LessThan, Sol, true) => MarketCapSolAggregate,
            (MarketCap, LessThanEqual, Sol, true) => MarketCapSolAggregate,

            // MarketCapUsdAggregate
            (MarketCap, MoreThan, Usd, true) => MarketCapUsdAggregate,
            (MarketCap, MoreThanEqual, Usd, true) => MarketCapUsdAggregate,
            (MarketCap, LessThan, Usd, true) => MarketCapUsdAggregate,
            (MarketCap, LessThanEqual, Usd, true) => MarketCapUsdAggregate,

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

            // PriceAvgQuoteAggregate
            (PriceAvg, MoreThan, Quote, true) => PriceAvgQuoteAggregate,
            (PriceAvg, MoreThanEqual, Quote, true) => PriceAvgQuoteAggregate,
            (PriceAvg, LessThan, Quote, true) => PriceAvgQuoteAggregate,
            (PriceAvg, LessThanEqual, Quote, true) => PriceAvgQuoteAggregate,

            // PriceAvgUsdAggregate
            (PriceAvg, MoreThan, Usd, true) => PriceAvgUsdAggregate,
            (PriceAvg, MoreThanEqual, Usd, true) => PriceAvgUsdAggregate,
            (PriceAvg, LessThan, Usd, true) => PriceAvgUsdAggregate,
            (PriceAvg, LessThanEqual, Usd, true) => PriceAvgUsdAggregate,

            // SwapAllCountAggregate
            (SwapAll, MoreThan, Count, true) => SwapAllCountAggregate,
            (SwapAll, MoreThanEqual, Count, true) => SwapAllCountAggregate,
            (SwapAll, LessThan, Count, true) => SwapAllCountAggregate,
            (SwapAll, LessThanEqual, Count, true) => SwapAllCountAggregate,

            // SwapBuyCountAggregate
            (SwapBuy, MoreThan, Count, true) => SwapBuyCountAggregate,
            (SwapBuy, MoreThanEqual, Count, true) => SwapBuyCountAggregate,
            (SwapBuy, LessThan, Count, true) => SwapBuyCountAggregate,
            (SwapBuy, LessThanEqual, Count, true) => SwapBuyCountAggregate,

            // SwapSellCountAggregate
            (SwapSell, MoreThan, Count, true) => SwapSellCountAggregate,
            (SwapSell, MoreThanEqual, Count, true) => SwapSellCountAggregate,
            (SwapSell, LessThan, Count, true) => SwapSellCountAggregate,
            (SwapSell, LessThanEqual, Count, true) => SwapSellCountAggregate,

            // SwapAllChangeAggregate
            (SwapAll, IncreasedByMoreThan, Count, true) => SwapAllChangeAggregate,
            (SwapAll, IncreasedByMoreThanEqual, Count, true) => SwapAllChangeAggregate,
            (SwapAll, IncreasedByLessThan, Count, true) => SwapAllChangeAggregate,
            (SwapAll, IncreasedByLessThanEqual, Count, true) => SwapAllChangeAggregate,
            (SwapAll, DecreasedByMoreThan, Count, true) => SwapAllChangeAggregate,
            (SwapAll, DecreasedByMoreThanEqual, Count, true) => SwapAllChangeAggregate,
            (SwapAll, DecreasedByLessThan, Count, true) => SwapAllChangeAggregate,
            (SwapAll, DecreasedByLessThanEqual, Count, true) => SwapAllChangeAggregate,

            // SwapBuyChangeAggregate
            (SwapBuy, IncreasedByMoreThan, Count, true) => SwapBuyChangeAggregate,
            (SwapBuy, IncreasedByMoreThanEqual, Count, true) => SwapBuyChangeAggregate,
            (SwapBuy, IncreasedByLessThan, Count, true) => SwapBuyChangeAggregate,
            (SwapBuy, IncreasedByLessThanEqual, Count, true) => SwapBuyChangeAggregate,
            (SwapBuy, DecreasedByMoreThan, Count, true) => SwapBuyChangeAggregate,
            (SwapBuy, DecreasedByMoreThanEqual, Count, true) => SwapBuyChangeAggregate,
            (SwapBuy, DecreasedByLessThan, Count, true) => SwapBuyChangeAggregate,
            (SwapBuy, DecreasedByLessThanEqual, Count, true) => SwapBuyChangeAggregate,

            // SwapSellChangeAggregate
            (SwapSell, IncreasedByMoreThan, Count, true) => SwapSellChangeAggregate,
            (SwapSell, IncreasedByMoreThanEqual, Count, true) => SwapSellChangeAggregate,
            (SwapSell, IncreasedByLessThan, Count, true) => SwapSellChangeAggregate,
            (SwapSell, IncreasedByLessThanEqual, Count, true) => SwapSellChangeAggregate,
            (SwapSell, DecreasedByMoreThan, Count, true) => SwapSellChangeAggregate,
            (SwapSell, DecreasedByMoreThanEqual, Count, true) => SwapSellChangeAggregate,
            (SwapSell, DecreasedByLessThan, Count, true) => SwapSellChangeAggregate,
            (SwapSell, DecreasedByLessThanEqual, Count, true) => SwapSellChangeAggregate,

            // SwapAllPercentAggregate
            (SwapAll, IncreasedByMoreThan, Percent, true) => SwapAllPercentAggregate,
            (SwapAll, IncreasedByMoreThanEqual, Percent, true) => SwapAllPercentAggregate,
            (SwapAll, IncreasedByLessThan, Percent, true) => SwapAllPercentAggregate,
            (SwapAll, IncreasedByLessThanEqual, Percent, true) => SwapAllPercentAggregate,
            (SwapAll, DecreasedByMoreThan, Percent, true) => SwapAllPercentAggregate,
            (SwapAll, DecreasedByMoreThanEqual, Percent, true) => SwapAllPercentAggregate,
            (SwapAll, DecreasedByLessThan, Percent, true) => SwapAllPercentAggregate,
            (SwapAll, DecreasedByLessThanEqual, Percent, true) => SwapAllPercentAggregate,

            // SwapBuyPercentAggregate
            (SwapBuy, IncreasedByMoreThan, Percent, true) => SwapBuyPercentAggregate,
            (SwapBuy, IncreasedByMoreThanEqual, Percent, true) => SwapBuyPercentAggregate,
            (SwapBuy, IncreasedByLessThan, Percent, true) => SwapBuyPercentAggregate,
            (SwapBuy, IncreasedByLessThanEqual, Percent, true) => SwapBuyPercentAggregate,
            (SwapBuy, DecreasedByMoreThan, Percent, true) => SwapBuyPercentAggregate,
            (SwapBuy, DecreasedByMoreThanEqual, Percent, true) => SwapBuyPercentAggregate,
            (SwapBuy, DecreasedByLessThan, Percent, true) => SwapBuyPercentAggregate,
            (SwapBuy, DecreasedByLessThanEqual, Percent, true) => SwapBuyPercentAggregate,

            // SwapBuyPercentAggregate
            (SwapSell, IncreasedByMoreThan, Percent, true) => SwapSellPercentAggregate,
            (SwapSell, IncreasedByMoreThanEqual, Percent, true) => SwapSellPercentAggregate,
            (SwapSell, IncreasedByLessThan, Percent, true) => SwapSellPercentAggregate,
            (SwapSell, IncreasedByLessThanEqual, Percent, true) => SwapSellPercentAggregate,
            (SwapSell, DecreasedByMoreThan, Percent, true) => SwapSellPercentAggregate,
            (SwapSell, DecreasedByMoreThanEqual, Percent, true) => SwapSellPercentAggregate,
            (SwapSell, DecreasedByLessThan, Percent, true) => SwapSellPercentAggregate,
            (SwapSell, DecreasedByLessThanEqual, Percent, true) => SwapSellPercentAggregate,

            // VolumeChangeQuoteAggregate
            (Volume, IncreasedByMoreThan, Quote, true) => VolumeChangeQuoteAggregate,
            (Volume, IncreasedByMoreThanEqual, Quote, true) => VolumeChangeQuoteAggregate,
            (Volume, IncreasedByLessThan, Quote, true) => VolumeChangeQuoteAggregate,
            (Volume, IncreasedByLessThanEqual, Quote, true) => VolumeChangeQuoteAggregate,
            (Volume, DecreasedByMoreThan, Quote, true) => VolumeChangeQuoteAggregate,
            (Volume, DecreasedByMoreThanEqual, Quote, true) => VolumeChangeQuoteAggregate,
            (Volume, DecreasedByLessThan, Quote, true) => VolumeChangeQuoteAggregate,
            (Volume, DecreasedByLessThanEqual, Quote, true) => VolumeChangeQuoteAggregate,

            // Telegram
            (Field::TelegramGroupHandle, Equal, ValueType::String, false) => TelegramGroupHandle,

            // Twitter
            (Field::TwitterAccountHandle, Equal, ValueType::String, false) => TwitterAccountHandle,

            // Venue
            (Field::VenuePumpfun, Equal, Boolean, false) => VenuePumpfun,
            (Field::VenuePumpfun, NotEqual, Boolean, false) => VenuePumpfun,
            (Field::VenueJupiter, Equal, Boolean, false) => VenueJupiter,
            (Field::VenueJupiter, NotEqual, Boolean, false) => VenueJupiter,

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
    fn test_age_base() {
        assert_eq!(
            Fact::from_comparison(&AgeBase, &MoreThan, &Value::duration(1, Minute), false),
            Some(AgeBaseDuration)
        );
        assert_eq!(
            Fact::from_comparison(&AgeBase, &MoreThanEqual, &Value::duration(1, Minute), false),
            Some(AgeBaseDuration)
        );

        assert_eq!(
            Fact::from_comparison(&AgeBase, &LessThan, &Value::duration(1, Minute), false),
            Some(AgeBaseDuration)
        );

        assert_eq!(
            Fact::from_comparison(&AgeBase, &LessThanEqual, &Value::duration(1, Minute), false),
            Some(AgeBaseDuration)
        );
    }

    #[test]
    fn test_age_quote() {
        assert_eq!(
            Fact::from_comparison(&AgeQuote, &MoreThan, &Value::duration(1, Minute), false),
            Some(AgeQuoteDuration)
        );
        assert_eq!(
            Fact::from_comparison(
                &AgeQuote,
                &MoreThanEqual,
                &Value::duration(1, Minute),
                false
            ),
            Some(AgeQuoteDuration)
        );

        assert_eq!(
            Fact::from_comparison(&AgeQuote, &LessThan, &Value::duration(1, Minute), false),
            Some(AgeQuoteDuration)
        );

        assert_eq!(
            Fact::from_comparison(
                &AgeQuote,
                &LessThanEqual,
                &Value::duration(1, Minute),
                false
            ),
            Some(AgeQuoteDuration)
        );
    }

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
    fn test_market_cap_quote() {
        assert_eq!(
            Fact::from_comparison(&MarketCap, &MoreThan, &Value::quote(99), false),
            Some(MarketCapQuote)
        );
        assert_eq!(
            Fact::from_comparison(
                &MarketCap,
                &MoreThanEqual,
                &Value::quote(99),
                false
            ),
            Some(MarketCapQuote)
        );
        assert_eq!(
            Fact::from_comparison(&MarketCap, &LessThan, &Value::quote(99), false),
            Some(MarketCapQuote)
        );
        assert_eq!(
            Fact::from_comparison(
                &MarketCap,
                &LessThanEqual,
                &Value::quote(99),
                false
            ),
            Some(MarketCapQuote)
        );
    }

    #[test]
    fn test_market_cap_sol() {
        assert_eq!(
            Fact::from_comparison(&MarketCap, &MoreThan, &Value::sol(99), false),
            Some(MarketCapSol)
        );
        assert_eq!(
            Fact::from_comparison(&MarketCap, &MoreThanEqual, &Value::sol(99), false),
            Some(MarketCapSol)
        );
        assert_eq!(
            Fact::from_comparison(&MarketCap, &LessThan, &Value::sol(99), false),
            Some(MarketCapSol)
        );
        assert_eq!(
            Fact::from_comparison(&MarketCap, &LessThanEqual, &Value::sol(99), false),
            Some(MarketCapSol)
        );
    }

    #[test]
    fn test_market_cap_usd() {
        assert_eq!(
            Fact::from_comparison(&MarketCap, &MoreThan, &Value::usd(99), false),
            Some(MarketCapUsd)
        );
        assert_eq!(
            Fact::from_comparison(&MarketCap, &MoreThanEqual, &Value::usd(99), false),
            Some(MarketCapUsd)
        );
        assert_eq!(
            Fact::from_comparison(&MarketCap, &LessThan, &Value::usd(99), false),
            Some(MarketCapUsd)
        );
        assert_eq!(
            Fact::from_comparison(&MarketCap, &LessThanEqual, &Value::usd(99), false),
            Some(MarketCapUsd)
        );
    }

    #[test]
    fn test_market_cap_quote_aggregate() {
        assert_eq!(
            Fact::from_comparison(&MarketCap, &MoreThan, &Value::quote(99), true),
            Some(MarketCapQuoteAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &MarketCap,
                &MoreThanEqual,
                &Value::quote(99),
                true
            ),
            Some(MarketCapQuoteAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&MarketCap, &LessThan, &Value::quote(99), true),
            Some(MarketCapQuoteAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &MarketCap,
                &LessThanEqual,
                &Value::quote(99),
                true
            ),
            Some(MarketCapQuoteAggregate)
        );
    }

    #[test]
    fn test_market_cap_sol_aggregate() {
        assert_eq!(
            Fact::from_comparison(&MarketCap, &MoreThan, &Value::sol(99), true),
            Some(MarketCapSolAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&MarketCap, &MoreThanEqual, &Value::sol(99), true),
            Some(MarketCapSolAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&MarketCap, &LessThan, &Value::sol(99), true),
            Some(MarketCapSolAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&MarketCap, &LessThanEqual, &Value::sol(99), true),
            Some(MarketCapSolAggregate)
        );
    }

    #[test]
    fn test_market_cap_usd_aggregate() {
        assert_eq!(
            Fact::from_comparison(&MarketCap, &MoreThan, &Value::usd(99), true),
            Some(MarketCapUsdAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&MarketCap, &MoreThanEqual, &Value::usd(99), true),
            Some(MarketCapUsdAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&MarketCap, &LessThan, &Value::usd(99), true),
            Some(MarketCapUsdAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&MarketCap, &LessThanEqual, &Value::usd(99), true),
            Some(MarketCapUsdAggregate)
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
    fn test_price_avg_quote_aggregate() {
        assert_eq!(
            Fact::from_comparison(&PriceAvg, &MoreThan, &Value::quote(99), true),
            Some(PriceAvgQuoteAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&PriceAvg, &MoreThanEqual, &Value::quote(99), true),
            Some(PriceAvgQuoteAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&PriceAvg, &LessThan, &Value::quote(99), true),
            Some(PriceAvgQuoteAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&PriceAvg, &LessThanEqual, &Value::quote(99), true),
            Some(PriceAvgQuoteAggregate)
        );
    }

    #[test]
    fn test_price_avg_usd_aggregate() {
        assert_eq!(
            Fact::from_comparison(&PriceAvg, &MoreThan, &Value::usd(99), true),
            Some(PriceAvgUsdAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&PriceAvg, &MoreThanEqual, &Value::usd(99), true),
            Some(PriceAvgUsdAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&PriceAvg, &LessThan, &Value::usd(99), true),
            Some(PriceAvgUsdAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&PriceAvg, &LessThanEqual, &Value::usd(99), true),
            Some(PriceAvgUsdAggregate)
        );
    }

    #[test]
    fn test_swap_all_count_aggregate() {
        assert_eq!(
            Fact::from_comparison(&SwapAll, &MoreThan, &Value::count(9924), true),
            Some(SwapAllCountAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&SwapAll, &MoreThanEqual, &Value::count(9924), true),
            Some(SwapAllCountAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&SwapAll, &LessThan, &Value::count(9924), true),
            Some(SwapAllCountAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&SwapAll, &LessThanEqual, &Value::count(9924), true),
            Some(SwapAllCountAggregate)
        );
    }

    #[test]
    fn test_swap_buy_count_aggregate() {
        assert_eq!(
            Fact::from_comparison(&SwapBuy, &MoreThan, &Value::count(9924), true),
            Some(SwapBuyCountAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&SwapBuy, &MoreThanEqual, &Value::count(9924), true),
            Some(SwapBuyCountAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&SwapBuy, &LessThan, &Value::count(9924), true),
            Some(SwapBuyCountAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&SwapBuy, &LessThanEqual, &Value::count(9924), true),
            Some(SwapBuyCountAggregate)
        );
    }

    #[test]
    fn test_swap_sell_count_aggregate() {
        assert_eq!(
            Fact::from_comparison(&SwapSell, &MoreThan, &Value::count(9924), true),
            Some(SwapSellCountAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&SwapSell, &MoreThanEqual, &Value::count(9924), true),
            Some(SwapSellCountAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&SwapSell, &LessThan, &Value::count(9924), true),
            Some(SwapSellCountAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&SwapSell, &LessThanEqual, &Value::count(9924), true),
            Some(SwapSellCountAggregate)
        );
    }

    #[test]
    fn test_swap_change_aggregate() {
        assert_eq!(
            Fact::from_comparison(&SwapAll, &IncreasedByMoreThan, &Value::count(9924), true),
            Some(SwapAllChangeAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapAll,
                &IncreasedByMoreThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapAllChangeAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&SwapAll, &IncreasedByLessThan, &Value::count(9924), true),
            Some(SwapAllChangeAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapAll,
                &IncreasedByLessThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapAllChangeAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&SwapAll, &DecreasedByMoreThan, &Value::count(9924), true),
            Some(SwapAllChangeAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapAll,
                &DecreasedByMoreThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapAllChangeAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&SwapAll, &DecreasedByLessThan, &Value::count(9924), true),
            Some(SwapAllChangeAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapAll,
                &DecreasedByLessThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapAllChangeAggregate)
        );
    }

    #[test]
    fn test_swap_buy_change_aggregate() {
        assert_eq!(
            Fact::from_comparison(&SwapBuy, &IncreasedByMoreThan, &Value::count(9924), true),
            Some(SwapBuyChangeAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapBuy,
                &IncreasedByMoreThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapBuyChangeAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&SwapBuy, &IncreasedByLessThan, &Value::count(9924), true),
            Some(SwapBuyChangeAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapBuy,
                &IncreasedByLessThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapBuyChangeAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&SwapBuy, &DecreasedByMoreThan, &Value::count(9924), true),
            Some(SwapBuyChangeAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapBuy,
                &DecreasedByMoreThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapBuyChangeAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&SwapBuy, &DecreasedByLessThan, &Value::count(9924), true),
            Some(SwapBuyChangeAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapBuy,
                &DecreasedByLessThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapBuyChangeAggregate)
        );
    }

    #[test]
    fn test_swap_sell_change_aggregate() {
        assert_eq!(
            Fact::from_comparison(&SwapSell, &IncreasedByMoreThan, &Value::count(9924), true),
            Some(SwapSellChangeAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapSell,
                &IncreasedByMoreThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapSellChangeAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&SwapSell, &IncreasedByLessThan, &Value::count(9924), true),
            Some(SwapSellChangeAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapSell,
                &IncreasedByLessThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapSellChangeAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&SwapSell, &DecreasedByMoreThan, &Value::count(9924), true),
            Some(SwapSellChangeAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapSell,
                &DecreasedByMoreThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapSellChangeAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&SwapSell, &DecreasedByLessThan, &Value::count(9924), true),
            Some(SwapSellChangeAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapSell,
                &DecreasedByLessThanEqual,
                &Value::count(9924),
                true
            ),
            Some(SwapSellChangeAggregate)
        );
    }

    #[test]
    fn test_swap_percent_aggregate() {
        assert_eq!(
            Fact::from_comparison(&SwapAll, &IncreasedByMoreThan, &Value::percent(99.24), true),
            Some(SwapAllPercentAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapAll,
                &IncreasedByMoreThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapAllPercentAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&SwapAll, &IncreasedByLessThan, &Value::percent(99.24), true),
            Some(SwapAllPercentAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapAll,
                &IncreasedByLessThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapAllPercentAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&SwapAll, &DecreasedByMoreThan, &Value::percent(99.24), true),
            Some(SwapAllPercentAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapAll,
                &DecreasedByMoreThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapAllPercentAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&SwapAll, &DecreasedByLessThan, &Value::percent(99.24), true),
            Some(SwapAllPercentAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapAll,
                &DecreasedByLessThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapAllPercentAggregate)
        );
    }

    #[test]
    fn test_swap_buy_percent_aggregate() {
        assert_eq!(
            Fact::from_comparison(&SwapBuy, &IncreasedByMoreThan, &Value::percent(99.24), true),
            Some(SwapBuyPercentAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapBuy,
                &IncreasedByMoreThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapBuyPercentAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&SwapBuy, &IncreasedByLessThan, &Value::percent(99.24), true),
            Some(SwapBuyPercentAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapBuy,
                &IncreasedByLessThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapBuyPercentAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&SwapBuy, &DecreasedByMoreThan, &Value::percent(99.24), true),
            Some(SwapBuyPercentAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapBuy,
                &DecreasedByMoreThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapBuyPercentAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&SwapBuy, &DecreasedByLessThan, &Value::percent(99.24), true),
            Some(SwapBuyPercentAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapBuy,
                &DecreasedByLessThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapBuyPercentAggregate)
        );
    }

    #[test]
    fn test_swap_sell_percent_aggregate() {
        assert_eq!(
            Fact::from_comparison(
                &SwapSell,
                &IncreasedByMoreThan,
                &Value::percent(99.24),
                true
            ),
            Some(SwapSellPercentAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapSell,
                &IncreasedByMoreThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapSellPercentAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapSell,
                &IncreasedByLessThan,
                &Value::percent(99.24),
                true
            ),
            Some(SwapSellPercentAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapSell,
                &IncreasedByLessThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapSellPercentAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapSell,
                &DecreasedByMoreThan,
                &Value::percent(99.24),
                true
            ),
            Some(SwapSellPercentAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapSell,
                &DecreasedByMoreThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapSellPercentAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapSell,
                &DecreasedByLessThan,
                &Value::percent(99.24),
                true
            ),
            Some(SwapSellPercentAggregate)
        );
        assert_eq!(
            Fact::from_comparison(
                &SwapSell,
                &DecreasedByLessThanEqual,
                &Value::percent(99.24),
                true
            ),
            Some(SwapSellPercentAggregate)
        );
    }

    #[test]
    fn test_volume_change_quote_aggregate() {
        assert_eq!(
            Fact::from_comparison(&Volume, &IncreasedByMoreThan, &Value::quote(99), true),
            Some(VolumeChangeQuoteAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&Volume, &IncreasedByMoreThanEqual, &Value::quote(99), true),
            Some(VolumeChangeQuoteAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&Volume, &IncreasedByLessThan, &Value::quote(99), true),
            Some(VolumeChangeQuoteAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&Volume, &IncreasedByLessThanEqual, &Value::quote(99), true),
            Some(VolumeChangeQuoteAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&Volume, &DecreasedByMoreThan, &Value::quote(99), true),
            Some(VolumeChangeQuoteAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&Volume, &DecreasedByMoreThanEqual, &Value::quote(99), true),
            Some(VolumeChangeQuoteAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&Volume, &DecreasedByLessThan, &Value::quote(99), true),
            Some(VolumeChangeQuoteAggregate)
        );
        assert_eq!(
            Fact::from_comparison(&Volume, &DecreasedByLessThanEqual, &Value::quote(99), true),
            Some(VolumeChangeQuoteAggregate)
        );
    }

    #[test]
    fn test_venue_pumpfun() {
        assert_eq!(
            Fact::from_comparison(&Field::VenuePumpfun, &Equal, &Value::boolean(true), false),
            Some(VenuePumpfun)
        );

        assert_eq!(
            Fact::from_comparison(
                &Field::VenuePumpfun,
                &NotEqual,
                &Value::boolean(true),
                false
            ),
            Some(VenuePumpfun)
        );
    }

    #[test]
    fn test_venue_jupiter() {
        assert_eq!(
            Fact::from_comparison(&Field::VenueJupiter, &Equal, &Value::boolean(true), false),
            Some(VenueJupiter)
        );

        assert_eq!(
            Fact::from_comparison(
                &Field::VenueJupiter,
                &NotEqual,
                &Value::boolean(true),
                false
            ),
            Some(VenueJupiter)
        );
    }
}

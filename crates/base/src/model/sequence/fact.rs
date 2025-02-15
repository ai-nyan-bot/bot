// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Fact::{PriceChangePercent, PriceChangeQuote, PriceChangeUsd, PriceQuote, PriceUsd, VolumeChangePercent, VolumeChangeUsd, VolumeUsd};
use crate::model::FactError::UnableToDeriveFact;
use crate::model::ValueType::{Count, Percent, Quote};
use crate::model::{Condition, FactError, Field, Value, ValueType};
use serde::{Deserialize, Serialize};
use Fact::{
    TelegramGroup, TelegramGroupName, TradesBuyCount, TradesCount, TradesSellCount, TwitterAccount, TwitterAccountName, VolumeChangeQuote, VolumeQuote,
};
use Field::{Price, Trades, TradesBuy, TradesSell, Volume};
use ValueType::{Boolean, Usd};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Fact {
    PriceQuote,
    PriceUsd,
    PriceChangePercent,
    PriceChangeQuote,
    PriceChangeUsd,

    TradesCount,
    TradesBuyCount,
    TradesSellCount,

    VolumeQuote,
    VolumeUsd,
    VolumeChangePercent,
    VolumeChangeQuote,
    VolumeChangeUsd,

    TelegramGroup,
    TelegramGroupName,

    TwitterAccount,
    TwitterAccountName,
}

impl Fact {
    pub fn has_timeframe(&self) -> bool {
        match self {
            PriceQuote => false,
            PriceUsd => false,

            PriceChangeQuote => true,
            PriceChangePercent => true,
            PriceChangeUsd => true,

            TradesCount => true,
            TradesBuyCount => true,
            TradesSellCount => true,

            VolumeQuote => false,
            VolumeUsd => false,
            VolumeChangeQuote => true,
            VolumeChangePercent => true,
            VolumeChangeUsd => true,

            TelegramGroup => false,
            TelegramGroupName => false,

            TwitterAccount => false,
            TwitterAccountName => false,
        }
    }

    pub fn value_type(&self) -> ValueType {
        match self {
            PriceQuote => Quote,
            PriceUsd => Usd,

            PriceChangeQuote => Quote,
            PriceChangeUsd => Usd,
            PriceChangePercent => Percent,

            TradesCount => Count,
            TradesBuyCount => Count,
            TradesSellCount => Count,

            VolumeQuote => Quote,
            VolumeUsd => Usd,
            VolumeChangeQuote => Quote,
            VolumeChangePercent => Percent,
            VolumeChangeUsd => Usd,

            TelegramGroup => Boolean,
            TelegramGroupName => ValueType::String,
            TwitterAccount => Boolean,
            TwitterAccountName => ValueType::String,
        }
    }
}

impl TryFrom<&Condition> for Fact {
    type Error = FactError;

    fn try_from(condition: &Condition) -> Result<Self, Self::Error> {
        match condition {
            Condition::Compare { field, value, timeframe, .. } => {
                Fact::from_comparison(field, value, timeframe.is_some()).ok_or(UnableToDeriveFact(condition.clone()))
            }
            Condition::And { .. } | Condition::Or { .. } | Condition::AndNot { .. } => Err(UnableToDeriveFact(condition.clone())),
        }
    }
}

impl Fact {
    fn from_comparison(field: &Field, value: &Value, has_timeframe: bool) -> Option<Self> {
        let fact = match (field, value.value_type(), has_timeframe) {
            (Price, Quote, false) => PriceQuote,
            (Price, Usd, false) => PriceUsd,
            (Price, Quote, true) => PriceChangeQuote,
            (Price, Usd, true) => PriceChangeUsd,
            (Price, Percent, true) => PriceChangePercent,

            (Trades, Count, true) => TradesCount,
            (TradesBuy, Count, true) => TradesBuyCount,
            (TradesSell, Count, true) => TradesSellCount,

            (Volume, Quote, false) => VolumeQuote,
            (Volume, Usd, false) => VolumeUsd,
            (Volume, Quote, true) => VolumeChangeQuote,
            (Volume, Usd, true) => VolumeChangeUsd,
            (Volume, Percent, true) => VolumeChangePercent,
            _ => return None,
        };

        Some(fact)
    }
}

#[cfg(test)]
mod test {
    use crate::model::Fact::{PriceChangePercent, PriceChangeQuote, PriceChangeUsd, PriceQuote, PriceUsd, TradesSellCount};
    use crate::model::Field::{Price, Trades, TradesBuy, TradesSell};
    use crate::model::Operator::GreaterThan;
    use crate::model::Value::{Count, Percent, Usd};
    use crate::model::{Condition, Fact, Value};
    use common::model::Timeframe;
    use common::model::Timeframe::M15;
    use Condition::Compare;
    use Fact::{TradesBuyCount, TradesCount};
    use Timeframe::H1;
    use Value::Quote;

    #[test]
    fn price_quote() {
        assert_eq!(
            Fact::try_from(&Compare {
                field: Price,
                operator: GreaterThan,
                value: Quote(2.0),
                timeframe: None
            })
            .unwrap(),
            PriceQuote
        );
    }

    #[test]
    fn price_usd() {
        assert_eq!(
            Fact::try_from(&Compare {
                field: Price,
                operator: GreaterThan,
                value: Usd(2.0),
                timeframe: None
            })
            .unwrap(),
            PriceUsd
        );
    }

    #[test]
    fn price_change_percent() {
        assert_eq!(
            Fact::try_from(&Compare {
                field: Price,
                operator: GreaterThan,
                value: Percent(2.0),
                timeframe: Some(H1)
            })
            .unwrap(),
            PriceChangePercent
        );
    }

    #[test]
    fn price_change_quote() {
        assert_eq!(
            Fact::try_from(&Compare {
                field: Price,
                operator: GreaterThan,
                value: Quote(2.0),
                timeframe: Some(H1)
            })
            .unwrap(),
            PriceChangeQuote
        );
    }

    #[test]
    fn price_change_usd() {
        assert_eq!(
            Fact::try_from(&Compare {
                field: Price,
                operator: GreaterThan,
                value: Usd(2.0),
                timeframe: Some(H1)
            })
            .unwrap(),
            PriceChangeUsd
        );
    }

    #[test]
    fn trades_count() {
        assert_eq!(
            Fact::try_from(&Compare {
                field: Trades,
                operator: GreaterThan,
                value: Count(2),
                timeframe: Some(M15)
            })
            .unwrap(),
            TradesCount
        );
    }

    #[test]
    fn trades_buy_count() {
        assert_eq!(
            Fact::try_from(&Compare {
                field: TradesBuy,
                operator: GreaterThan,
                value: Count(2),
                timeframe: Some(M15)
            })
            .unwrap(),
            TradesBuyCount
        );
    }

    #[test]
    fn trades_sell_count() {
        assert_eq!(
            Fact::try_from(&Compare {
                field: TradesSell,
                operator: GreaterThan,
                value: Count(2),
                timeframe: Some(M15)
            })
            .unwrap(),
            TradesSellCount
        );
    }
}

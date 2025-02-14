// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Fact::{
    TokenPriceChangePercent, TokenPriceChangeQuote, TokenPriceChangeUsd, TokenPriceQuote, TokenPriceUsd, TokenVolumeChangePercent, TokenVolumeChangeUsd,
    TokenVolumeUsd,
};
use crate::model::FactError::UnableToDeriveFact;
use crate::model::ValueType::{Percent, Quote};
use crate::model::{Condition, FactError, Field, Value, ValueType};
use serde::{Deserialize, Serialize};
use Fact::{TelegramGroup, TelegramGroupName, TokenVolumeChangeQuote, TokenVolumeQuote, TwitterAccount, TwitterAccountName};
use Field::{Price, Volume};
use ValueType::{Boolean, Usd};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Fact {
    TokenPriceQuote,
    TokenPriceUsd,
    TokenPriceChangePercent,
    TokenPriceChangeQuote,
    TokenPriceChangeUsd,

    TokenVolumeQuote,
    TokenVolumeUsd,
    TokenVolumeChangePercent,
    TokenVolumeChangeQuote,
    TokenVolumeChangeUsd,

    TelegramGroup,
    TelegramGroupName,

    TwitterAccount,
    TwitterAccountName,
}

impl Fact {
    pub fn has_timeframe(&self) -> bool {
        match self {
            TokenPriceQuote => false,
            TokenPriceUsd => false,

            TokenPriceChangeQuote => true,
            TokenPriceChangePercent => true,
            TokenPriceChangeUsd => true,

            TokenVolumeQuote => false,
            TokenVolumeUsd => false,
            TokenVolumeChangeQuote => true,
            TokenVolumeChangePercent => true,
            TokenVolumeChangeUsd => true,

            TelegramGroup => false,
            TelegramGroupName => false,

            TwitterAccount => false,
            TwitterAccountName => false,
        }
    }

    pub fn value_type(&self) -> ValueType {
        match self {
            TokenPriceQuote => Quote,
            TokenPriceUsd => Usd,

            TokenPriceChangeQuote => Quote,
            TokenPriceChangeUsd => Usd,
            TokenPriceChangePercent => Percent,

            TokenVolumeQuote => Quote,
            TokenVolumeUsd => Usd,
            TokenVolumeChangeQuote => Quote,
            TokenVolumeChangePercent => Percent,
            TokenVolumeChangeUsd => Usd,

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
            Condition::And(_) | Condition::Or(_) | Condition::AndNot(_) => Err(UnableToDeriveFact(condition.clone())),
        }
    }
}

impl Fact {
    fn from_comparison(field: &Field, value: &Value, has_timeframe: bool) -> Option<Self> {
        let fact = match (field, value.value_type(), has_timeframe) {
            (Price, Quote, false) => TokenPriceQuote,
            (Price, Usd, false) => TokenPriceUsd,
            (Price, Quote, true) => TokenPriceChangeQuote,
            (Price, Usd, true) => TokenPriceChangeUsd,
            (Price, Percent, true) => TokenPriceChangePercent,

            (Volume, Quote, false) => TokenVolumeQuote,
            (Volume, Usd, false) => TokenVolumeUsd,
            (Volume, Quote, true) => TokenVolumeChangeQuote,
            (Volume, Usd, true) => TokenVolumeChangeUsd,
            (Volume, Percent, true) => TokenVolumeChangePercent,
            _ => return None,
        };

        Some(fact)
    }
}

#[cfg(test)]
mod test {
    use crate::model::Fact::{TokenPriceChangePercent, TokenPriceChangeQuote, TokenPriceChangeUsd, TokenPriceQuote, TokenPriceUsd};
    use crate::model::Field::Price;
    use crate::model::Operator::GreaterThan;
    use crate::model::Value::{Percent, Usd};
    use crate::model::{Condition, Fact, Value};
    use common::model::Timeframe;
    use Condition::Compare;
    use Timeframe::H1;
    use Value::Quote;

    #[test]
    fn token_price_quote() {
        assert_eq!(
            Fact::try_from(&Compare {
                field: Price,
                operator: GreaterThan,
                value: Quote(2.0),
                timeframe: None
            })
            .unwrap(),
            TokenPriceQuote
        );
    }

    #[test]
    fn token_price_usd() {
        assert_eq!(
            Fact::try_from(&Compare {
                field: Price,
                operator: GreaterThan,
                value: Usd(2.0),
                timeframe: None
            })
            .unwrap(),
            TokenPriceUsd
        );
    }

    #[test]
    fn token_price_change_percent() {
        assert_eq!(
            Fact::try_from(&Compare {
                field: Price,
                operator: GreaterThan,
                value: Percent(2.0),
                timeframe: Some(H1)
            })
            .unwrap(),
            TokenPriceChangePercent
        );
    }

    #[test]
    fn token_price_change_quote() {
        assert_eq!(
            Fact::try_from(&Compare {
                field: Price,
                operator: GreaterThan,
                value: Quote(2.0),
                timeframe: Some(H1)
            })
            .unwrap(),
            TokenPriceChangeQuote
        );
    }

    #[test]
    fn token_price_change_usd() {
        assert_eq!(
            Fact::try_from(&Compare {
                field: Price,
                operator: GreaterThan,
                value: Usd(2.0),
                timeframe: Some(H1)
            })
            .unwrap(),
            TokenPriceChangeUsd
        );
    }
}

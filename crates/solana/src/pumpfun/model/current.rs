// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::solana::Slot;
use base::model::{Amount, TokenPairId};
use common::model::{
    AgeRelativeToLatestInSeconds, MarketCapQuote, MarketCapUsd, Percent, PriceQuote, PriceUsd,
};

#[derive(Debug, Clone)]
pub struct Current {
    pub id: TokenPairId,
    pub slot: Slot,
    pub virtual_base_reserves: Amount,
    pub virtual_quote_reserves: Amount,
    pub progress: Percent,
    pub price: PriceQuote,
    pub price_usd: Option<PriceUsd>,
    pub market_cap: Option<MarketCapQuote>,
    pub market_cap_usd: Option<MarketCapUsd>,
    pub complete: bool,
    pub age: AgeRelativeToLatestInSeconds,
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::solana::{Signature, Slot};
use base::model::{AddressId, Amount, DecimalAmount, SwapId, TokenPairId};
use common::model::{Percent, PriceQuote, Timestamp};

#[derive(Debug)]
pub struct Swap {
    pub id: SwapId,
    pub slot: Slot,
    pub address: AddressId,
    pub token_pair: TokenPairId,
    pub amount_base: DecimalAmount,
    pub amount_quote: DecimalAmount,
    pub price: PriceQuote,
    pub is_buy: bool,
    pub timestamp: Timestamp,
    pub virtual_base_reserves: Amount,
    pub virtual_quote_reserves: Amount,
    pub progress: Percent,
    pub signature: Signature,
}

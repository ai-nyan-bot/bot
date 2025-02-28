// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Slot;
use base::model::{AddressId, Amount, DecimalAmount, PriceQuote, TokenPairId};
use common::model::Timestamp;

#[derive(Debug)]
pub struct Trade {
    pub slot: Slot,
    pub address: AddressId,
    pub token_pair: TokenPairId,
    pub base_amount: DecimalAmount,
    pub quote_amount: DecimalAmount,
    pub price: PriceQuote,
    pub is_buy: bool,
    pub timestamp: Timestamp,
    pub virtual_base_reserves: Amount,
    pub virtual_quote_reserves: Amount,
}

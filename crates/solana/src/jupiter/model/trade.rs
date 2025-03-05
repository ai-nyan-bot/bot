// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Slot;
use base::model::{AddressId, DecimalAmount, PriceQuote, TokenPairId};
use common::model::Timestamp;

pub struct Trade {
    pub slot: Slot,
    pub address: AddressId,
    pub token_pair: TokenPairId,
    pub amount_base: DecimalAmount,
    pub amount_quote: DecimalAmount,
    pub price: PriceQuote,
    pub is_buy: bool,
    pub timestamp: Timestamp,
}

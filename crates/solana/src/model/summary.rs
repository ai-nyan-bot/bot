// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{TokenPairId, Trades};

#[derive(Clone, Debug)]
pub struct Summary {
    pub token_pair: TokenPairId,
    pub trades: SummaryTrades,
}

#[derive(Clone, Debug)]
pub struct SummaryTrades {
    pub buy: TradesAndChange,
    pub sell: TradesAndChange,
    pub total: TradesAndChange,
}

#[derive(Clone, Debug)]
pub struct TradesAndChange {
    pub trades: Trades,
    // pub change: Change,
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{TokenPairId, Trades, TradesChange, TradesChangePercent};

#[derive(Clone, Debug)]
pub struct Summary {
    pub token_pair: TokenPairId,
    pub trades: SummaryTrades,
}

#[derive(Clone, Debug)]
pub struct SummaryTrades {
    pub all: TradesWithChange,
    pub buy: TradesWithChange,
    pub sell: TradesWithChange,
}

#[derive(Clone, Debug)]
pub struct TradesWithChange {
    pub trades: Trades,
    pub change: Option<TradesChange>,
    pub change_percent: Option<TradesChangePercent>,
}

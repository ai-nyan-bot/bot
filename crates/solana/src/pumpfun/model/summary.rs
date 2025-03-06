// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::TokenPairId;
use common::model::{Change, Count, Percent};

#[derive(Clone, Debug)]
pub struct Summary {
    pub token_pair: TokenPairId,
    pub curve_progress: SummaryCurveProgress,
    pub trade: SummaryTrade,
}

#[derive(Clone, Debug)]
pub struct SummaryCurveProgress {
    pub open: ProgressWithChange,
    pub high: ProgressWithChange,
    pub low: ProgressWithChange,
    pub close: ProgressWithChange,
    pub avg: ProgressWithChange,
}

#[derive(Clone, Debug)]
pub struct SummaryTrade {
    pub all: TradesWithChange,
    pub buy: TradesWithChange,
    pub sell: TradesWithChange,
}

#[derive(Clone, Debug)]
pub struct TradesWithChange {
    pub count: Count,
    pub change: Option<Change>,
    pub percent: Option<Percent>,
}

#[derive(Clone, Debug)]
pub struct ProgressWithChange {
    pub progress: Option<Percent>,
    pub change: Option<Percent>,
}

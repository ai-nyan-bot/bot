// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::TokenPair;
use common::model::{Change, Count, Percent};
use crate::pumpfun::model::Curve;

#[derive(Clone, Debug)]
pub struct PumpfunSummary {
    pub pair: TokenPair,
    pub curve: Curve,
    pub m1: Option<TimeframeSummary>,
    pub m5: Option<TimeframeSummary>,
    pub m15: Option<TimeframeSummary>,
    pub h1: Option<TimeframeSummary>,
    pub h6: Option<TimeframeSummary>,
    pub d1: Option<TimeframeSummary>,
}

#[derive(Clone, Debug)]
pub struct TimeframeSummary {
    pub curve: SummaryCurveProgress,
    pub swap: SummarySwap,
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
pub struct SummarySwap {
    pub all: SwapsWithChange,
    pub buy: SwapsWithChange,
    pub sell: SwapsWithChange,
}

#[derive(Clone, Debug)]
pub struct SwapsWithChange {
    pub count: Count,
    pub change: Option<Change>,
    pub percent: Option<Percent>,
}

#[derive(Clone, Debug)]
pub struct ProgressWithChange {
    pub progress: Option<Percent>,
    pub change: Option<Percent>,
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::pumpfun::model::Curve;
use base::model::TokenPair;
use common::model::volume::{Volume, VolumeUsd};
use common::model::{Count, Percent};

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
    pub volume: SummaryVolume,
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
    pub all: SwapWithChange,
    pub buy: SwapWithChange,
    pub sell: SwapWithChange,
}

#[derive(Clone, Debug)]
pub struct SummaryVolume {
    pub all: VolumeWithChange,
    pub buy: VolumeWithChange,
    pub sell: VolumeWithChange,
}

#[derive(Clone, Debug)]
pub struct ProgressWithChange {
    pub progress: Option<Percent>,
    pub change: Option<Percent>,
}

#[derive(Clone, Debug)]
pub struct SwapWithChange {
    pub count: Option<Count>,
    pub change: Option<Count>,
    pub percent: Option<Percent>,
}

#[derive(Clone, Debug)]
pub struct VolumeWithChange {
    pub quote: Option<Volume>,
    pub usd: Option<VolumeUsd>,
    pub quote_change: Option<Volume>,
    pub usd_change: Option<VolumeUsd>,
    pub percent: Option<Percent>,
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::pumpfun::model::Curve;
use base::model::TokenPair;
use common::model::{
    Count, MarketCapQuote, MarketCapUsd, Percent, PriceQuote, PriceUsd, VolumeQuote, VolumeUsd,
};

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
    pub cap: SummaryMarketCap,
    pub price: SummaryPrice,
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
pub struct SummaryMarketCap {
    pub open: MarketCapWithChange,
    pub high: MarketCapWithChange,
    pub low: MarketCapWithChange,
    pub close: MarketCapWithChange,
    pub avg: MarketCapWithChange,
}

#[derive(Clone, Debug)]
pub struct SummaryPrice {
    pub open: PriceWithChange,
    pub high: PriceWithChange,
    pub low: PriceWithChange,
    pub close: PriceWithChange,
    pub avg: PriceWithChange,
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
pub struct MarketCapWithChange {
    pub quote: Option<MarketCapQuote>,
    pub usd: Option<MarketCapUsd>,
    pub quote_change: Option<MarketCapQuote>,
    pub usd_change: Option<MarketCapUsd>,
    pub percent: Option<Percent>,
}

#[derive(Clone, Debug)]
pub struct PriceWithChange {
    pub quote: Option<PriceQuote>,
    pub usd: Option<PriceUsd>,
    pub quote_change: Option<PriceQuote>,
    pub usd_change: Option<PriceUsd>,
    pub percent: Option<Percent>,
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
    pub quote: Option<VolumeQuote>,
    pub usd: Option<VolumeUsd>,
    pub quote_change: Option<VolumeQuote>,
    pub usd_change: Option<VolumeUsd>,
    pub percent: Option<Percent>,
}

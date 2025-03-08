// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{
    MarketCapWithChange, ProgressWithChange, SummaryCurveProgress, SummaryMarketCap, SummarySwap,
    SummaryVolume, SwapWithChange, VolumeWithChange,
};
use bigdecimal::{BigDecimal, ToPrimitive};
use common::model::{Count, MarketCap, MarketCapUsd, Percent, VolumeQuote, VolumeUsd};
use sqlx::postgres::PgRow;
use sqlx::Row;

pub(crate) fn row_to_curve_progress(row: &PgRow) -> SummaryCurveProgress {
    SummaryCurveProgress {
        open: ProgressWithChange {
            progress: row.try_get::<Percent, _>("curve_progress_open").ok(),
            change: row.try_get::<Percent, _>("curve_progress_change").ok(),
        },
        high: ProgressWithChange {
            progress: row.try_get::<Percent, _>("curve_progress_high").ok(),
            change: row.try_get::<Percent, _>("curve_progress_change").ok(),
        },
        low: ProgressWithChange {
            progress: row.try_get::<Percent, _>("curve_progress_low").ok(),
            change: row.try_get::<Percent, _>("curve_progress_change").ok(),
        },
        close: ProgressWithChange {
            progress: row.try_get::<Percent, _>("curve_progress_close").ok(),
            change: row.try_get::<Percent, _>("curve_progress_change").ok(),
        },
        avg: ProgressWithChange {
            progress: row.try_get::<Percent, _>("curve_progress_avg").ok(),
            change: row.try_get::<Percent, _>("curve_progress_change").ok(),
        },
    }
}

pub(crate) fn row_to_market_cap(row: &PgRow) -> SummaryMarketCap {
    SummaryMarketCap {
        open: MarketCapWithChange {
            quote: row.try_get::<MarketCap, _>("market_cap_open").ok(),
            usd: row.try_get::<MarketCapUsd, _>("market_cap_open_usd").ok(),
            quote_change: row.try_get::<MarketCap, _>("market_cap_open_change").ok(),
            usd_change: row
                .try_get::<MarketCapUsd, _>("market_cap_open_usd_change")
                .ok(),
            percent: row.try_get::<Percent, _>("market_cap_open_percent").ok(),
        },
        high: MarketCapWithChange {
            quote: row.try_get::<MarketCap, _>("market_cap_high").ok(),
            usd: row.try_get::<MarketCapUsd, _>("market_cap_high_usd").ok(),
            quote_change: row.try_get::<MarketCap, _>("market_cap_high_change").ok(),
            usd_change: row
                .try_get::<MarketCapUsd, _>("market_cap_high_usd_change")
                .ok(),
            percent: row.try_get::<Percent, _>("market_cap_high_percent").ok(),
        },
        low: MarketCapWithChange {
            quote: row.try_get::<MarketCap, _>("market_cap_low").ok(),
            usd: row.try_get::<MarketCapUsd, _>("market_cap_low_usd").ok(),
            quote_change: row.try_get::<MarketCap, _>("market_cap_low_change").ok(),
            usd_change: row
                .try_get::<MarketCapUsd, _>("market_cap_low_usd_change")
                .ok(),
            percent: row.try_get::<Percent, _>("market_cap_low_percent").ok(),
        },
        close: MarketCapWithChange {
            quote: row.try_get::<MarketCap, _>("market_cap_close").ok(),
            usd: row.try_get::<MarketCapUsd, _>("market_cap_close_usd").ok(),
            quote_change: row.try_get::<MarketCap, _>("market_cap_close_change").ok(),
            usd_change: row
                .try_get::<MarketCapUsd, _>("market_cap_close_usd_change")
                .ok(),
            percent: row.try_get::<Percent, _>("market_cap_close_percent").ok(),
        },
        avg: MarketCapWithChange {
            quote: row.try_get::<MarketCap, _>("market_cap_avg").ok(),
            usd: row.try_get::<MarketCapUsd, _>("market_cap_avg_usd").ok(),
            quote_change: row.try_get::<MarketCap, _>("market_cap_avg_change").ok(),
            usd_change: row
                .try_get::<MarketCapUsd, _>("market_cap_avg_usd_change")
                .ok(),
            percent: row.try_get::<Percent, _>("market_cap_avg_percent").ok(),
        },
    }
}

pub(crate) fn row_to_swaps(row: &PgRow) -> SummarySwap {
    SummarySwap {
        buy: SwapWithChange {
            count: row.try_get::<Count, _>("swap_buy").ok(),
            change: row
                .try_get::<BigDecimal, _>("swap_buy_change")
                .ok()
                .map(|v| Count(v.to_i64().unwrap())),
            percent: row.try_get::<Percent, _>("swap_buy_percent").ok(),
        },
        sell: SwapWithChange {
            count: row.try_get::<Count, _>("swap_sell").ok(),
            change: row
                .try_get::<BigDecimal, _>("swap_sell_change")
                .ok()
                .map(|v| Count(v.to_i64().unwrap())),
            percent: row.try_get::<Percent, _>("swap_sell_percent").ok(),
        },
        all: SwapWithChange {
            count: row.try_get::<Count, _>("swap").ok(),
            change: row
                .try_get::<BigDecimal, _>("swap_change")
                .ok()
                .map(|v| Count(v.to_i64().unwrap())),
            percent: row.try_get::<Percent, _>("swap_percent").ok(),
        },
    }
}

pub(crate) fn row_to_volume(row: &PgRow) -> SummaryVolume {
    SummaryVolume {
        all: VolumeWithChange {
            quote: row.try_get::<VolumeQuote, _>("volume").ok(),
            usd: row.try_get::<VolumeUsd, _>("volume_usd").ok(),
            quote_change: row.try_get::<VolumeQuote, _>("volume_change").ok(),
            usd_change: row.try_get::<VolumeUsd, _>("volume_usd_change").ok(),
            percent: row.try_get::<Percent, _>("volume_percent").ok(),
        },
        buy: VolumeWithChange {
            quote: row.try_get::<VolumeQuote, _>("volume_buy").ok(),
            usd: row.try_get::<VolumeUsd, _>("volume_buy_usd").ok(),
            quote_change: row.try_get::<VolumeQuote, _>("volume_buy_change").ok(),
            usd_change: row.try_get::<VolumeUsd, _>("volume_buy_usd_change").ok(),
            percent: row.try_get::<Percent, _>("volume_buy_percent").ok(),
        },
        sell: VolumeWithChange {
            quote: row.try_get::<VolumeQuote, _>("volume_sell").ok(),
            usd: row.try_get::<VolumeUsd, _>("volume_sell_usd").ok(),
            quote_change: row.try_get::<VolumeQuote, _>("volume_sell_change").ok(),
            usd_change: row.try_get::<VolumeUsd, _>("volume_sell_usd_change").ok(),
            percent: row.try_get::<Percent, _>("volume_sell_percent").ok(),
        },
    }
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{
    ProgressWithChange, SummaryCurveProgress, SummarySwap, SummaryVolume, SwapWithChange,
    TimeframeSummary, VolumeWithChange,
};
use crate::pumpfun::repo::SummaryRepo;
use base::model::TokenPairId;
use bigdecimal::{BigDecimal, ToPrimitive};
use common::model::volume::{Volume, VolumeUsd};
use common::model::{Count, Percent, Timeframe};
use common::repo::{RepoResult, Tx};
use sqlx::postgres::PgRow;
use sqlx::Row;

impl SummaryRepo {
    pub async fn get<'a>(
        &self,
        tx: &mut Tx<'a>,
        token_pair: impl Into<TokenPairId> + Send,
        timeframe: Timeframe,
    ) -> RepoResult<TimeframeSummary> {
        let table = format!("pumpfun.summary_{}", timeframe.table());

        Ok(sqlx::query(
            format!(
                r#"
select
    curve_progress_open,
    curve_progress_open_change,
    curve_progress_high,
    curve_progress_high_change,
    curve_progress_low,
    curve_progress_low_change,
    curve_progress_close,
    curve_progress_close_change,
    curve_progress_avg,
    curve_progress_avg_change,
    
    swap,
    swap_change,
    swap_percent,
    swap_buy,
    swap_buy_change,
    swap_buy_percent,
    swap_sell,
    swap_sell_change,
    swap_sell_percent,

    volume,
    volume_usd,
    volume_change,
    volume_usd_change,
    volume_percent,
    volume_buy,
    volume_buy_usd,
    volume_buy_change,
    volume_buy_usd_change,
    volume_buy_percent,
    volume_sell,
    volume_sell_usd,
    volume_sell_change,
    volume_sell_usd_change,
    volume_sell_percent
from {table}
where token_pair_id = $1
"#
            )
            .as_str(),
        )
        .bind(token_pair.into())
        .fetch_one(&mut **tx)
        .await
        .map(|row| TimeframeSummary {
            curve: row_to_curve_progress(&row),
            swap: row_to_swaps(&row),
            volume: row_to_volume(&row),
        })?)
    }
}

fn row_to_curve_progress(row: &PgRow) -> SummaryCurveProgress {
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

fn row_to_swaps(row: &PgRow) -> SummarySwap {
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

fn row_to_volume(row: &PgRow) -> SummaryVolume {
    SummaryVolume {
        all: VolumeWithChange {
            quote: row.try_get::<Volume, _>("volume").ok(),
            usd: row.try_get::<VolumeUsd, _>("volume_usd").ok(),
            quote_change: row.try_get::<Volume, _>("volume_change").ok(),
            usd_change: row.try_get::<VolumeUsd, _>("volume_usd_change").ok(),
            percent: row.try_get::<Percent, _>("volume_percent").ok(),
        },
        buy: VolumeWithChange {
            quote: row.try_get::<Volume, _>("volume_buy").ok(),
            usd: row.try_get::<VolumeUsd, _>("volume_buy_usd").ok(),
            quote_change: row.try_get::<Volume, _>("volume_buy_change").ok(),
            usd_change: row.try_get::<VolumeUsd, _>("volume_buy_usd_change").ok(),
            percent: row.try_get::<Percent, _>("volume_buy_percent").ok(),
        },
        sell: VolumeWithChange {
            quote: row.try_get::<Volume, _>("volume_sell").ok(),
            usd: row.try_get::<VolumeUsd, _>("volume_sell_usd").ok(),
            quote_change: row.try_get::<Volume, _>("volume_sell_change").ok(),
            usd_change: row.try_get::<VolumeUsd, _>("volume_sell_usd_change").ok(),
            percent: row.try_get::<Percent, _>("volume_sell_percent").ok(),
        },
    }
}

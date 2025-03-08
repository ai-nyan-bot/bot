// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::TimeframeSummary;
use crate::pumpfun::repo::summary::row::{
    row_to_curve_progress, row_to_market_cap, row_to_price, row_to_swaps, row_to_volume,
};
use crate::pumpfun::repo::SummaryRepo;
use base::model::TokenPairId;
use common::model::Timeframe;
use common::repo::{RepoResult, Tx};

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
    
    market_cap_open,
    market_cap_open_usd,
    market_cap_open_change,
    market_cap_open_usd_change,
    market_cap_open_percent,
    market_cap_high,
    market_cap_high_usd,
    market_cap_high_change,
    market_cap_high_usd_change,
    market_cap_high_percent,
    market_cap_low,
    market_cap_low_usd,
    market_cap_low_change,
    market_cap_low_usd_change,
    market_cap_low_percent,
    market_cap_close,
    market_cap_close_usd,
    market_cap_close_change,
    market_cap_close_usd_change,
    market_cap_close_percent,
    market_cap_avg,
    market_cap_avg_usd,
    market_cap_avg_change,
    market_cap_avg_usd_change,
    market_cap_avg_percent,
    
    price_open,
    price_open_usd,
    price_open_change,
    price_open_usd_change,
    price_open_percent,
    price_high,
    price_high_usd,
    price_high_change,
    price_high_usd_change,
    price_high_percent,
    price_low,
    price_low_usd,
    price_low_change,
    price_low_usd_change,
    price_low_percent,
    price_close,
    price_close_usd,
    price_close_change,
    price_close_usd_change,
    price_close_percent,
    price_avg,
    price_avg_usd,
    price_avg_change,
    price_avg_usd_change,
    price_avg_percent,
    
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
            cap: row_to_market_cap(&row),
            price: row_to_price(&row),
            swap: row_to_swaps(&row),
            volume: row_to_volume(&row),
        })?)
    }
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{ProgressWithChange, Summary, SummaryCurveProgress, SummarySwap, SwapsWithChange};
use crate::pumpfun::repo::SummaryRepo;
use base::model::TokenPairId;
use common::model::{Change, Count, Percent, Timeframe};
use common::repo::{RepoResult, Tx};
use sqlx::postgres::PgRow;
use sqlx::Row;


impl SummaryRepo {
	pub async fn get<'a>(
		&self,
		tx: &mut Tx<'a>,
		token_pair: impl Into<TokenPairId> + Send,
		timeframe: Timeframe,
	) -> RepoResult<Summary> {
		let table = format!("pumpfun.summary_{}", timeframe.table());

		Ok(sqlx::query(
			format!(
				r#"
select
    token_pair_id,
    
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
    swap_sell_percent
from {table}
where token_pair_id = $1
"#
			)
				.as_str(),
		)
			.bind(token_pair.into())
			.fetch_one(&mut **tx)
			.await
			.map(|row| Summary {
				token_pair: row.get::<TokenPairId, _>("token_pair_id"),
				curve_progress: row_to_curve_progress(&row),
				swap: row_to_swaps(&row),
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
		buy: SwapsWithChange {
			count: row.get::<Count, _>("swap_buy"),
			change: row.try_get::<Change, _>("swap_buy_change").ok(),
			percent: row.try_get::<Percent, _>("swap_buy_percent").ok(),
		},
		sell: SwapsWithChange {
			count: row.get::<Count, _>("swap_sell"),
			change: row.try_get::<Change, _>("swap_sell_change").ok(),
			percent: row.try_get::<Percent, _>("swap_sell_percent").ok(),
		},
		all: SwapsWithChange {
			count: row.get::<Count, _>("swap"),
			change: row.try_get::<Change, _>("swap_change").ok(),
			percent: row.try_get::<Percent, _>("swap_percent").ok(),
		},
	}
}

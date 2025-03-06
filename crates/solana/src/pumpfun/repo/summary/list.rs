// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{
    ProgressWithChange, Summary, SummaryCurveProgress, SummaryTrade, TradesWithChange,
};
use crate::pumpfun::repo::{SummaryQuery, SummaryRepo};
use base::model::TokenPairId;
use common::model::{Change, Count, Percent};
use common::repo::{RepoResult, Tx};
use sqlx::postgres::PgRow;
use sqlx::{Postgres, QueryBuilder, Row};

impl SummaryRepo {
    pub async fn list<'a>(&self, tx: &mut Tx<'a>, query: SummaryQuery) -> RepoResult<Vec<Summary>> {
        let table = format!("pumpfun.summary_{}", query.timeframe.table());

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(format!(
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

    trade,
    trade_change,
    trade_percent,
    trade_buy,
    trade_buy_change,
    trade_buy_percent,
    trade_sell,
    trade_sell_change,
    trade_sell_percent
from {table}
"#,
        ));

        query_builder.push("\nlimit ");
        query_builder.push_bind(query.limit);

        Ok(query_builder
            .build()
            .fetch_all(&mut **tx)
            .await?
            .into_iter()
            .map(|row| Summary {
                token_pair: row.get::<TokenPairId, _>("token_pair_id"),
                curve_progress: row_to_curve_progress(&row),
                trade: row_to_trades(&row),
            })
            .collect::<Vec<_>>())
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

fn row_to_trades(row: &PgRow) -> SummaryTrade {
    SummaryTrade {
        buy: TradesWithChange {
            count: row.get::<Count, _>("trade_buy"),
            change: row.try_get::<Change, _>("trade_buy_change").ok(),
            percent: row.try_get::<Percent, _>("trade_buy_percent").ok(),
        },
        sell: TradesWithChange {
            count: row.get::<Count, _>("trade_sell"),
            change: row.try_get::<Change, _>("trade_sell_change").ok(),
            percent: row.try_get::<Percent, _>("trade_sell_percent").ok(),
        },
        all: TradesWithChange {
            count: row.get::<Count, _>("trade"),
            change: row.try_get::<Change, _>("trade_change").ok(),
            percent: row.try_get::<Percent, _>("trade_percent").ok(),
        },
    }
}

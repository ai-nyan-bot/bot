// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Summary, SummaryTrades, TradesWithChange};
use crate::pumpfun::repo::SummaryRepo;
use base::model::{Change, Percent, TokenPairId, Trades};
use common::model::Timeframe;
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
            trades: row_to_trades(&row),
        })?)
    }
}

fn row_to_trades(row: &PgRow) -> SummaryTrades {
    SummaryTrades {
        buy: TradesWithChange {
            trades: row.get::<Trades, _>("trade_buy"),
            change: row.try_get::<Change, _>("trade_buy_change").ok(),
            percent: row.try_get::<Percent, _>("trade_buy_percent").ok(),
        },
        sell: TradesWithChange {
            trades: row.get::<Trades, _>("trade_sell"),
            change: row.try_get::<Change, _>("trade_sell_change").ok(),
            percent: row.try_get::<Percent, _>("trade_sell_percent").ok(),
        },
        all: TradesWithChange {
            trades: row.get::<Trades, _>("trade"),
            change: row.try_get::<Change, _>("trade_change").ok(),
            percent: row.try_get::<Percent, _>("trade_percent").ok(),
        },
    }
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Summary, SummaryTrades, TradesWithChange};
use crate::pumpfun::repo::{SummaryQuery, SummaryRepo};
use base::model::{Change, Percent, TokenPairId, Trades};
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
    trades,
    trades_change,
    trades_percent,
    trades_buy,
    trades_buy_change,
    trades_buy_percent,
    trades_sell,
    trades_sell_change,
    trades_sell_percent
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
                trades: row_to_trades(&row),
            })
            .collect::<Vec<_>>())
    }
}

fn row_to_trades(row: &PgRow) -> SummaryTrades {
    SummaryTrades {
        buy: TradesWithChange {
            trades: row.get::<Trades, _>("trades_buy"),
            change: row.try_get::<Change, _>("trades_buy_change").ok(),
            percent: row.try_get::<Percent, _>("trades_buy_percent").ok(),
        },
        sell: TradesWithChange {
            trades: row.get::<Trades, _>("trades_sell"),
            change: row.try_get::<Change, _>("trades_sell_change").ok(),
            percent: row.try_get::<Percent, _>("trades_sell_percent").ok(),
        },
        all: TradesWithChange {
            trades: row.get::<Trades, _>("trades"),
            change: row.try_get::<Change, _>("trades_change").ok(),
            percent: row.try_get::<Percent, _>("trades_percent").ok(),
        },
    }
}

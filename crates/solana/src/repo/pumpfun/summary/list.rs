// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Summary, SummaryTrades, TradesAndChange};
use crate::repo::pumpfun::{SummaryQuery, SummaryRepo};
use base::model::{TokenPairId, Trades};
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
    buy_trades,
    sell_trades
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
        buy: TradesAndChange {
            trades: row.get::<Trades, _>("buy_trades"),
            // change: Change(row.get::<f64, _>("buy_trades_change")),
        },
        sell: TradesAndChange {
            trades: row.get::<Trades, _>("sell_trades"),
            // change: Change(row.get::<f64, _>("sell_trades_change")),
        },
        total: TradesAndChange {
            trades: row.get::<Trades, _>("trades"),
            // change: Change(row.get::<f64, _>("trades_change")),
        },
    }
}

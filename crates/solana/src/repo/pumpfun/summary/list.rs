// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Summary, SummaryTrades, TradesWithChange};
use crate::repo::pumpfun::{SummaryQuery, SummaryRepo};
use base::model::{TokenPairId, Trades, TradesChange, TradesChangePercent};
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
    trades_change_percent,
    trades_buy,
    trades_buy_change,
    trades_buy_change_percent,
    trades_sell,
    trades_sell_change,
    trades_sell_change_percent
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
			change: row.try_get::<TradesChange, _>("trades_buy_change").ok(),
			change_percent: row.try_get::<TradesChangePercent, _>("trades_buy_change_percent").ok(),
		},
		sell: TradesWithChange {
			trades: row.get::<Trades, _>("trades_sell"),
			change: row.try_get::<TradesChange, _>("trades_sell_change").ok(),
			change_percent: row.try_get::<TradesChangePercent, _>("trades_sell_change_percent").ok(),
		},
		all: TradesWithChange {
			trades: row.get::<Trades, _>("trades"),
			change: row.try_get::<TradesChange, _>("trades_change").ok(),
			change_percent: row.try_get::<TradesChangePercent, _>("trades_change_percent").ok(),
		},
	}
}

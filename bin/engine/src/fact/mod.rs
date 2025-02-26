// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{Fact, Facts, TokenPairId, Value};
use common::model::{Limit, Timeframe};
use solana::pumpfun::repo::{ReadTradeRepo, SummaryQuery, SummaryRepo};
use sqlx::PgPool;

#[derive(Clone)]
pub struct FactService {
	pool: PgPool,
	trade_repo: ReadTradeRepo,
	summary_repo: SummaryRepo,
}

impl FactService {
	pub fn new(pool: PgPool, trade_repo: ReadTradeRepo, summary_repo: SummaryRepo) -> Self {
		Self {
			pool,
			trade_repo,
			summary_repo,
		}
	}

	pub async fn pumpfun_facts(&self) -> Box<[(TokenPairId, Facts)]> {
		let mut tx = self.pool.begin().await.unwrap();

		// let most_recent_trades = self.trade_repo.list_most_recent(&mut tx).await.unwrap();

		let summary_1m = self
			.summary_repo
			.list(
				&mut tx,
				SummaryQuery {
					limit: Limit::unlimited(),
					timeframe: Timeframe::M1,
				},
			)
			.await
			.unwrap();

		let mut result: Vec<(TokenPairId, Facts)> = Vec::with_capacity(summary_1m.len());
		// for trade in most_recent_trades {
		//     let facts = Facts::new().with_value(Fact::TokenPriceAvgQuote, trade.price).unwrap();
		//     result.push((trade.token_pair.clone(), facts));
		// }

		for summary in summary_1m {
			let mut facts = Facts::new()
				.with_timeframe_value(Fact::TradesCount, summary.trades.all.trades, Timeframe::M1)
				.unwrap()
				.with_timeframe_value(Fact::TradesBuyCount, summary.trades.buy.trades, Timeframe::M1)
				.unwrap()
				.with_timeframe_value(Fact::TradesSellCount, summary.trades.sell.trades, Timeframe::M1)
				.unwrap();

			if let Some(change) = summary.trades.all.change {
				facts.set_timeframe_value(Fact::TradesChangeCount, Value::Count(change.0 as i64), Timeframe::M1).unwrap();
			}

			if let Some(percent) = summary.trades.all.change_percent {
				facts.set_timeframe_value(Fact::TradesChangePercent, Value::Percent(percent.0 as f64), Timeframe::M1).unwrap();
			}


			if let Some(change) = summary.trades.buy.change {
				facts.set_timeframe_value(Fact::TradesBuyCount, Value::Count(change.0 as i64), Timeframe::M1).unwrap();
			}

			if let Some(percent) = summary.trades.buy.change_percent {
				facts.set_timeframe_value(Fact::TradesBuyChangePercent, Value::Percent(percent.0 as f64), Timeframe::M1).unwrap();
			}


			if let Some(change) = summary.trades.sell.change {
				facts.set_timeframe_value(Fact::TradesSellCount, Value::Count(change.0 as i64), Timeframe::M1).unwrap();
			}

			if let Some(percent) = summary.trades.sell.change_percent {
				facts.set_timeframe_value(Fact::TradesSellChangePercent, Value::Percent(percent.0 as f64), Timeframe::M1).unwrap();
			}

			result.push((summary.token_pair, facts));
		}

		tx.commit().await.unwrap();

		result.into_boxed_slice()
	}
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{Fact, Facts, TokenPairId};
use common::model::{Limit, Timeframe};
use solana::repo::pumpfun::{ReadTradeRepo, SummaryQuery, SummaryRepo};
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
        //     let facts = Facts::new().with_value(Fact::TokenPriceQuote, trade.price).unwrap();
        //     result.push((trade.token_pair.clone(), facts));
        // }

        for summary in summary_1m {
            let facts = Facts::new()
                .with_timeframe_value(Fact::TradesCount, summary.trades.total.trades, Timeframe::M1)
                .unwrap()
                .with_timeframe_value(Fact::TradesBuyCount, summary.trades.buy.trades, Timeframe::M1)
                .unwrap()
                .with_timeframe_value(Fact::TradesSellCount, summary.trades.sell.trades, Timeframe::M1)
                .unwrap();

            result.push((summary.token_pair.clone(), facts));
        }

        let _ = tx.commit().await.unwrap();

        result.into_boxed_slice()
    }
}

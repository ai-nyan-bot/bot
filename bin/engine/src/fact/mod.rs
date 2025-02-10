// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{Fact, Facts};
use common::model::TokenPairId;
use solana::repo::pumpfun::ReadTradeRepo;
use sqlx::PgPool;

#[derive(Clone)]
pub struct FactService {
    pool: PgPool,
    trade_repo: ReadTradeRepo,
}

impl FactService {
    pub fn new(pool: PgPool, trade_repo: ReadTradeRepo) -> Self {
        Self { pool, trade_repo }
    }

    pub async fn pumpfun_facts(&self) -> Box<[(TokenPairId, Facts)]> {
        let mut tx = self.pool.begin().await.unwrap();
        let most_recent_trades = self.trade_repo.list_most_recent(&mut tx).await.unwrap();

        let mut result: Vec<(TokenPairId, Facts)> = Vec::with_capacity(most_recent_trades.len());
        for trade in most_recent_trades {
            let facts = Facts::new().with_value(Fact::TokenPriceQuote, trade.price).unwrap();

            result.push((trade.token_pair.clone(), facts));
        }

        let _ = tx.commit().await.unwrap();

        result.into_boxed_slice()
    }
}

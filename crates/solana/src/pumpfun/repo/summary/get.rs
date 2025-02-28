// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Summary, SummaryTrades, TradesWithChange};
use crate::pumpfun::repo::SummaryRepo;
use base::model::{TokenPairId, Trades, TradesChange, TradesChangePercent};
use common::model::Timeframe;
use common::repo::{RepoResult, Tx};
use sqlx::postgres::PgRow;
use sqlx::Row;

impl SummaryRepo {
    pub async fn get<'a>(
        &self,
        tx: &mut Tx<'a>,
        token_pair: impl Into<TokenPairId> + Send,
        timeframe: Timeframe
    ) -> RepoResult<Summary> {
        let table = format!("pumpfun.summary_{}", timeframe.table());

        Ok(sqlx::query(
            format!(
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
            trades: row.get::<Trades, _>("trades_buy"),
            change: row.try_get::<TradesChange, _>("trades_buy_change").ok(),
            change_percent: row
                .try_get::<TradesChangePercent, _>("trades_buy_change_percent")
                .ok(),
        },
        sell: TradesWithChange {
            trades: row.get::<Trades, _>("trades_sell"),
            change: row.try_get::<TradesChange, _>("trades_sell_change").ok(),
            change_percent: row
                .try_get::<TradesChangePercent, _>("trades_sell_change_percent")
                .ok(),
        },
        all: TradesWithChange {
            trades: row.get::<Trades, _>("trades"),
            change: row.try_get::<TradesChange, _>("trades_change").ok(),
            change_percent: row
                .try_get::<TradesChangePercent, _>("trades_change_percent")
                .ok(),
        },
    }
}

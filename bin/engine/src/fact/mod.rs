// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{Fact, Facts, TokenPairId, Value};
use bigdecimal::ToPrimitive;
use common::model::Timeframe::M1;
use common::model::{Limit, TimeUnit};
use solana::pumpfun::repo::{CurveRepo, SummaryQuery, SummaryRepo};
use sqlx::PgPool;
use std::collections::HashMap;

#[derive(Clone)]
pub struct FactService {
    pool: PgPool,
    summary_repo: SummaryRepo,
    curve_repo: CurveRepo,
}

impl FactService {
    pub fn new(pool: PgPool, summary_repo: SummaryRepo, curve_repo: CurveRepo) -> Self {
        Self {
            pool,
            summary_repo,
            curve_repo,
        }
    }

    pub async fn pumpfun_facts(&self) -> HashMap<TokenPairId, Facts> {
        let mut tx = self.pool.begin().await.unwrap();

        let mut result: HashMap<TokenPairId, Facts> = self
            .curve_repo
            .list_all(&mut tx)
            .await
            .unwrap()
            .into_iter()
            .map(|c| {
                (
                    c.id.clone(),
                    Facts::new()
                        .with_value(Fact::CurveProgressPercent, Value::percent(c.progress.0))
                        .unwrap()
                        .with_value(
                            Fact::CurveProgressAgeDuration,
                            Value::duration(c.updated_at.age_sec(), TimeUnit::Second),
                        )
                        .unwrap(),
                )
            })
            .collect();

        let summary_1m = self
            .summary_repo
            .list(
                &mut tx,
                SummaryQuery {
                    limit: Limit::unlimited(),
                    timeframe: M1,
                },
            )
            .await
            .unwrap();

        for summary in summary_1m {
            let facts = result.entry(summary.token_pair).or_insert(Facts::default());

            facts
                .set_timeframe_value(Fact::TradesCount, summary.trade.all.count, M1)
                .unwrap();
            facts
                .set_timeframe_value(Fact::TradesBuyCount, summary.trade.buy.count, M1)
                .unwrap();
            facts
                .set_timeframe_value(Fact::TradesSellCount, summary.trade.sell.count, M1)
                .unwrap();

            if let Some(change) = summary.trade.all.change {
                facts
                    .set_timeframe_value(
                        Fact::TradesChangeCount,
                        Value::count(change.0.to_i64().unwrap()),
                        M1,
                    )
                    .unwrap();
            }

            if let Some(percent) = summary.trade.all.percent {
                facts
                    .set_timeframe_value(Fact::TradesChangePercent, Value::percent(percent.0), M1)
                    .unwrap();
            }

            if let Some(change) = summary.trade.buy.change {
                facts
                    .set_timeframe_value(
                        Fact::TradesBuyCount,
                        Value::count(change.0.to_i64().unwrap()),
                        M1,
                    )
                    .unwrap();
            }

            if let Some(percent) = summary.trade.buy.percent {
                facts
                    .set_timeframe_value(
                        Fact::TradesBuyChangePercent,
                        Value::percent(percent.0),
                        M1,
                    )
                    .unwrap();
            }

            if let Some(change) = summary.trade.sell.change {
                facts
                    .set_timeframe_value(
                        Fact::TradesSellCount,
                        Value::count(change.0.to_i64().unwrap()),
                        M1,
                    )
                    .unwrap();
            }

            if let Some(percent) = summary.trade.sell.percent {
                facts
                    .set_timeframe_value(
                        Fact::TradesSellChangePercent,
                        Value::percent(percent.0),
                        M1,
                    )
                    .unwrap();
            }
        }

        tx.commit().await.unwrap();

        result
    }
}

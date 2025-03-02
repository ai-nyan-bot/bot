// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{Fact, Facts, TokenPairId, Value};
use common::model::Timeframe::M1;
use common::model::{Limit, Timeframe};
use solana::pumpfun::repo::{CurveRepo, SummaryQuery, SummaryRepo};
use sqlx::PgPool;
use std::collections::HashMap;

#[derive(Clone)]
pub struct FactService {
    pool: PgPool,
    // trade_repo: ReadTradeRepo,
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
                        .with_value(
                            Fact::CurveProgressPercent,
                            Value::percent(c.progress.0 as f64),
                        )
                        .unwrap(),
                )
            })
            .collect();

        // let most_recent_trades = self.trade_repo.list_most_recent(&mut tx).await.unwrap();

        // for (token_pair, facts) in &mut result {
        //     match curves.get(token_pair) {
        //         None => {
        //
        //         }
        //         Some(curve) => facts
        //             .set_value(
        //                 Fact::CurveProgressPercent,
        //                 Value::percent(curve.progress.0 as f64),
        //             )
        //             .unwrap(),
        //     }
        // }

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

        // let mut result: HashMap<TokenPairId, Facts> = HashMap::with_capacity(summary_1m.len());
        // // for trade in most_recent_trades {
        // //     let facts = Facts::new().with_value(Fact::TokenPriceAvgQuote, trade.price).unwrap();
        // //     result.push((trade.token_pair.clone(), facts));
        // // }
        //
        for summary in summary_1m {
            match result.get_mut(&summary.token_pair) {
                None => {}
                Some(facts) => {
                    facts
                        .set_timeframe_value(Fact::TradesCount, summary.trades.all.trades, M1)
                        .unwrap();
                    facts
                        .set_timeframe_value(Fact::TradesBuyCount, summary.trades.buy.trades, M1)
                        .unwrap();
                    facts
                        .set_timeframe_value(Fact::TradesSellCount, summary.trades.sell.trades, M1)
                        .unwrap();

                    if let Some(change) = summary.trades.all.change {
                        facts
                            .set_timeframe_value(
                                Fact::TradesChangeCount,
                                Value::count(change.0 as i64),
                                M1,
                            )
                            .unwrap();
                    }

                    if let Some(percent) = summary.trades.all.change_percent {
                        facts
                            .set_timeframe_value(
                                Fact::TradesChangePercent,
                                Value::percent(percent.0 as f64),
                                M1,
                            )
                            .unwrap();
                    }

                    if let Some(change) = summary.trades.buy.change {
                        facts
                            .set_timeframe_value(
                                Fact::TradesBuyCount,
                                Value::count(change.0 as i64),
                                M1,
                            )
                            .unwrap();
                    }

                    if let Some(percent) = summary.trades.buy.change_percent {
                        facts
                            .set_timeframe_value(
                                Fact::TradesBuyChangePercent,
                                Value::percent(percent.0 as f64),
                                M1,
                            )
                            .unwrap();
                    }

                    if let Some(change) = summary.trades.sell.change {
                        facts
                            .set_timeframe_value(
                                Fact::TradesSellCount,
                                Value::count(change.0 as i64),
                                M1,
                            )
                            .unwrap();
                    }

                    if let Some(percent) = summary.trades.sell.change_percent {
                        facts
                            .set_timeframe_value(
                                Fact::TradesSellChangePercent,
                                Value::percent(percent.0 as f64),
                                M1,
                            )
                            .unwrap();
                    }
                }
            }
        }

        tx.commit().await.unwrap();

        result
    }
}

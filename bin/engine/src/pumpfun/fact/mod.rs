// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod summary;

use crate::pumpfun::fact::summary::add_summary_to_facts;
use base::model::Fact::CurveProgressAgeDuration;
use base::model::{Fact, Facts, TokenPairId, Value};
use common::model::Timeframe::M1;
use common::model::{Limit, TimeUnit};
use solana::pumpfun::repo::{CurveQuery, CurveRepo, SummaryQuery, SummaryRepo};
use sqlx::PgPool;
use std::collections::HashMap;
use Fact::CurveProgressPercent;

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
            .list(
                &mut tx,
                CurveQuery {
                    limit: Limit::unlimited(),
                },
            )
            .await
            .unwrap()
            .into_iter()
            .map(|c| {
                (
                    c.id.clone(),
                    Facts::new()
                        .with_value(CurveProgressPercent, Value::percent(c.progress.0))
                        .with_value(
                            CurveProgressAgeDuration,
                            Value::duration(c.age.0, TimeUnit::Second),
                        ),
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
            add_summary_to_facts(facts, summary, M1);
        }

        tx.commit().await.unwrap();

        result
    }
}

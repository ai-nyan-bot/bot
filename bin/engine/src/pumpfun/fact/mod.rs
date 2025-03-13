// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod summary;

use crate::pumpfun::fact::summary::add_summary_to_facts;
use base::model::Fact::CurveProgressAgeDuration;
use base::model::{Fact, Facts, TokenPairId, Value};
use base::repo::TokenPairRepo;
use common::model::{Limit, TimeUnit, Timeframe};
use solana::pumpfun::repo::{CurveQuery, CurveRepo, SummaryQuery, SummaryRepo};
use sqlx::PgPool;
use std::collections::HashMap;
use tokio::time::Instant;
use Fact::CurveProgressPercent;

#[derive(Clone)]
pub struct FactService {
    pool: PgPool,
    token_pair_repo: TokenPairRepo,
    summary_repo: SummaryRepo,
    curve_repo: CurveRepo,
}

impl FactService {
    pub fn new(
        pool: PgPool,
        token_pair_repo: TokenPairRepo,
        summary_repo: SummaryRepo,
        curve_repo: CurveRepo,
    ) -> Self {
        Self {
            pool,
            token_pair_repo,
            summary_repo,
            curve_repo,
        }
    }

    pub async fn pumpfun_facts(&self) -> HashMap<TokenPairId, Facts> {
        let mut tx = self.pool.begin().await.unwrap();

        let start = Instant::now();
        let mut result: HashMap<TokenPairId, Facts> = self
            .token_pair_repo
            .list_all(&mut tx)
            .await
            .unwrap()
            .into_iter()
            .map(|tp| {
                let mut facts = Facts::new();

                if let Some(age) = tp.base.age() {
                    facts.set_value(
                        Fact::AgeBaseDuration,
                        Value::duration(age.0, TimeUnit::Second),
                    )
                }

                if let Some(age) = tp.quote.age() {
                    facts.set_value(
                        Fact::AgeQuoteDuration,
                        Value::duration(age.0, TimeUnit::Second),
                    )
                }

                (tp.id, facts)
            })
            .collect();

        println!(
            "token pairs took: {}",
            Instant::now().duration_since(start).as_millis()
        );

        for curve in self
            .curve_repo
            .list(
                &mut tx,
                CurveQuery {
                    limit: Limit::unlimited(),
                },
            )
            .await
            .unwrap()
        {
            let facts = result.get_mut(&curve.id).unwrap();

            facts.set_value(CurveProgressPercent, Value::percent(curve.progress.0));
            facts.set_value(
                CurveProgressAgeDuration,
                Value::duration(curve.age.0, TimeUnit::Second),
            );
        }

        for timeframe in [
            Timeframe::M1,
            Timeframe::M5,
            Timeframe::M15,
            Timeframe::H1,
            Timeframe::H6,
            Timeframe::D1,
        ] {
            let start = Instant::now();

            let summary = self
                .summary_repo
                .list(
                    &mut tx,
                    SummaryQuery {
                        limit: Limit::unlimited(),
                        timeframe,
                    },
                )
                .await
                .unwrap();

            for (token_pair_id, summary) in summary {
                let facts = result.entry(token_pair_id).or_insert(Facts::default());
                add_summary_to_facts(facts, summary, timeframe);
            }

            println!(
                "Summary {:?} took: {}",
                timeframe,
                Instant::now().duration_since(start).as_millis()
            );
        }
        tx.commit().await.unwrap();

        result
    }
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Slot;
use crate::pumpfun::model::{Curve, Swap};
use crate::pumpfun::repo::curve::CurveRepo;
use base::model::{Amount, TokenPairId};
use common::model::{AgeRelativeToLatestInSeconds, Percent};
use common::repo::{RepoResult, Tx};
use sqlx::Row;

impl CurveRepo {
    pub async fn upsert<'a>(&self, tx: &mut Tx<'a>, swap: Swap) -> RepoResult<Curve> {
        Ok(sqlx::query(
            r#"
            insert into pumpfun.curve (
                id, slot, virtual_base_reserves, virtual_quote_reserves, progress, complete, updated_at
            )
            values ($1, $2, $3, $4, $5, $6, now())
            on conflict (id) do update set
                slot = excluded.slot,
                virtual_base_reserves = excluded.virtual_base_reserves,
                virtual_quote_reserves = excluded.virtual_quote_reserves,
                progress = excluded.progress,
                complete = excluded.complete,
                updated_at = now()
            returning id, slot, virtual_base_reserves, virtual_quote_reserves, progress, complete, updated_at
        "#
        )
            .bind(swap.token_pair)
            .bind(swap.slot)
            .bind(swap.virtual_base_reserves)
            .bind(swap.virtual_quote_reserves)
            .bind(swap.progress.clone())
            .bind(swap.progress >= 100.0)
            .fetch_one(&mut **tx)
            .await
            .map(|r| Curve {
                id: r.get::<TokenPairId, _>("id"),
                slot: r.get::<Slot, _>("slot"),
                virtual_base_reserves: r.get::<Amount, _>("virtual_base_reserves"),
                virtual_quote_reserves: r.get::<Amount, _>("virtual_quote_reserves"),
                progress: r.get::<Percent, _>("progress"),
                complete: r.get::<bool, _>("complete"),
                age: AgeRelativeToLatestInSeconds(0),
            })?)
    }
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Slot;
use crate::pumpfun::model::Curve;
use crate::pumpfun::repo::curve::CurveQuery;
use crate::pumpfun::repo::CurveRepo;
use base::model::{Amount, TokenPairId};
use common::model::{AgeRelativeToLatestInSeconds, Percent};
use common::repo::{RepoResult, Tx};
use sqlx::Row;

impl CurveRepo {
    pub async fn list<'a>(&self, tx: &mut Tx<'a>, query: CurveQuery) -> RepoResult<Vec<Curve>> {
        Ok(sqlx::query(
            r#"
        with latest as (
            select updated_at 
            from pumpfun.curve 
            order by updated_at desc 
            limit 1
        )
        select 
            c.*,
            extract(epoch from (latest.updated_at - c.updated_at))::int8 as age_seconds
        from 
            pumpfun.curve c,
            latest
        limit $1;
        "#,
        )
        .bind(query.limit)
        .fetch_all(&mut **tx)
        .await?
        .into_iter()
        .map(|r| Curve {
            id: r.get::<TokenPairId, _>("id"),
            slot: r.get::<Slot, _>("slot"),
            virtual_base_reserves: r.get::<Amount, _>("virtual_base_reserves"),
            virtual_quote_reserves: r.get::<Amount, _>("virtual_quote_reserves"),
            progress: r.get::<Percent, _>("progress"),
            complete: r.get::<bool, _>("complete"),
            age: r.get::<AgeRelativeToLatestInSeconds, _>("age_seconds"),
        })
        .collect::<Vec<_>>())
    }
}

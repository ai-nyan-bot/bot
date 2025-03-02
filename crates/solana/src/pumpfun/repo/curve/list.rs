// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Slot;
use crate::pumpfun::model::{Curve, CurveLog};
use crate::pumpfun::repo::CurveRepo;
use base::model::{Amount, Percent, TokenPairId};
use common::repo::{RepoResult, Tx};
use sqlx::Row;
use common::model::UpdatedAt;

impl CurveRepo {

    pub async fn list_all<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<Vec<Curve>> {
        let query = r#"
            select id, slot, virtual_base_reserves, virtual_quote_reserves, progress, complete, updated_at from pumpfun.curve order by id, slot desc
        "#;

        Ok(sqlx::query(query)
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
                updated_at: r.get::<UpdatedAt, _>("updated_at"),
            })
            .collect::<Vec<_>>())
    }

    pub async fn list_log_all<'a>(&self, tx: &mut Tx<'a>) -> RepoResult<Vec<CurveLog>> {
        let query = r#"
            select id, slot, virtual_base_reserves, virtual_quote_reserves, progress, complete from pumpfun.curve_log order by id, slot desc
        "#;

        Ok(sqlx::query(query)
            .fetch_all(&mut **tx)
            .await?
            .into_iter()
            .map(|r| CurveLog {
                id: r.get::<TokenPairId, _>("id"),
                slot: r.get::<Slot, _>("slot"),
                virtual_base_reserves: r.get::<Amount, _>("virtual_base_reserves"),
                virtual_quote_reserves: r.get::<Amount, _>("virtual_quote_reserves"),
                progress: r.get::<Percent, _>("progress"),
                complete: r.get::<bool, _>("complete"),
            })
            .collect::<Vec<_>>())
    }
}

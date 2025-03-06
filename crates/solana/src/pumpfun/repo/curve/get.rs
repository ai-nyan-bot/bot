// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Slot;
use crate::pumpfun::model::Curve;
use crate::pumpfun::repo::CurveRepo;
use base::model::{Amount, TokenPairId};
use common::model::{Percent, UpdatedAt};
use common::repo::{RepoResult, Tx};
use sqlx::Row;

impl CurveRepo {
    pub async fn get<'a>(
        &self,
        tx: &mut Tx<'a>,
        token_pair: impl Into<TokenPairId> + Send,
    ) -> RepoResult<Curve> {
        let query = r#"
            select id, slot, virtual_base_reserves, virtual_quote_reserves, progress, complete, updated_at from pumpfun.curve where id = $1
        "#;

        Ok(sqlx::query(query)
            .bind(token_pair.into())
            .fetch_one(&mut **tx)
            .await
            .map(|r| Curve {
                id: r.get::<TokenPairId, _>("id"),
                slot: r.get::<Slot, _>("slot"),
                virtual_base_reserves: r.get::<Amount, _>("virtual_base_reserves"),
                virtual_quote_reserves: r.get::<Amount, _>("virtual_quote_reserves"),
                progress: r.get::<Percent, _>("progress"),
                complete: r.get::<bool, _>("complete"),
                updated_at: r.get::<UpdatedAt, _>("updated_at"),
            })?)
    }
}

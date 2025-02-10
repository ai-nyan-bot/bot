// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Strategy, StrategyId, StrategyName, StrategyVersion};
use crate::repo::strategy::StrategyRepo;
use crate::model::UserId;
use common::model::{CreatedAt, UpdatedAt};
use common::repo::{RepoResult, Tx};
use sqlx::types::JsonValue;
use sqlx::{query, Row};

impl StrategyRepo {
    pub async fn get_by_id<'a>(&self, tx: &mut Tx<'a>, id: impl Into<StrategyId> + Send) -> RepoResult<Strategy> {
        Ok(query("select * from nyanbot.strategy where id = $1;")
            .bind(id.into())
            .fetch_one(&mut **tx)
            .await
            .map(|r| Strategy {
                id: r.get::<StrategyId, _>("id"),
                version: r.get::<StrategyVersion, _>("version"),
                name: r.get::<StrategyName, _>("name"),
                user: r.get::<UserId, _>("user_id"),
                sequence: r.get::<JsonValue, _>("sequence").into(),
                created_at: r.get::<CreatedAt, _>("created_at"),
                updated_at: r.get::<UpdatedAt, _>("updated_at"),
            })?)
    }
}

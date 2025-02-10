// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Strategy, StrategyId, StrategyName, StrategyVersion};
use crate::repo::strategy::{StrategyQueryAll, StrategyQueryUser, StrategyRepo};
use crate::model::UserId;
use common::model::{CreatedAt, UpdatedAt};
use common::repo::{RepoResult, Tx};
use sqlx::types::JsonValue;
use sqlx::Row;

impl StrategyRepo {
    pub async fn list_all<'a>(&self, tx: &mut Tx<'a>, query: StrategyQueryAll) -> RepoResult<Box<[Strategy]>> {
        Ok(sqlx::query("select * from solana.strategy order by id desc limit $1;")
            .bind(query.limit)
            .fetch_all(&mut **tx)
            .await?
            .iter()
            .map(|r| Strategy {
                id: r.get::<StrategyId, _>("id"),
                version: r.get::<StrategyVersion, _>("version"),
                name: r.get::<StrategyName, _>("name"),
                user: r.get::<UserId, _>("user_id"),
                sequence: r.get::<JsonValue, _>("sequence").into(),
                created_at: r.get::<CreatedAt, _>("created_at"),
                updated_at: r.get::<UpdatedAt, _>("updated_at"),
            })
            .collect::<Vec<_>>()
            .into_boxed_slice())
    }

    pub async fn list_user<'a>(&self, tx: &mut Tx<'a>, query: StrategyQueryUser) -> RepoResult<Box<[Strategy]>> {
        Ok(sqlx::query("select * from solana.strategy where user_id = $1 order by id desc limit $2;")
            .bind(query.user)
            .bind(query.limit)
            .fetch_all(&mut **tx)
            .await?
            .iter()
            .map(|r| Strategy {
                id: r.get::<StrategyId, _>("id"),
                version: r.get::<StrategyVersion, _>("version"),
                name: r.get::<StrategyName, _>("name"),
                user: r.get::<UserId, _>("user_id"),
                sequence: r.get::<JsonValue, _>("sequence").into(),
                created_at: r.get::<CreatedAt, _>("created_at"),
                updated_at: r.get::<UpdatedAt, _>("updated_at"),
            })
            .collect::<Vec<_>>()
            .into_boxed_slice())
    }
}

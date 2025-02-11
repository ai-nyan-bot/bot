// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Rule, RuleId, RuleName, RuleVersion};
use crate::repo::rule::{RuleQueryAll, RuleQueryUser, RuleRepo};
use crate::model::UserId;
use common::model::{CreatedAt, UpdatedAt};
use common::repo::{RepoResult, Tx};
use sqlx::types::JsonValue;
use sqlx::Row;

impl RuleRepo {
    pub async fn list_all<'a>(&self, tx: &mut Tx<'a>, query: RuleQueryAll) -> RepoResult<Box<[Rule]>> {
        Ok(sqlx::query("select * from solana.rule order by id desc limit $1;")
            .bind(query.limit)
            .fetch_all(&mut **tx)
            .await?
            .iter()
            .map(|r| Rule {
                id: r.get::<RuleId, _>("id"),
                version: r.get::<RuleVersion, _>("version"),
                name: r.get::<RuleName, _>("name"),
                user: r.get::<UserId, _>("user_id"),
                sequence: r.get::<JsonValue, _>("sequence").into(),
                created_at: r.get::<CreatedAt, _>("created_at"),
                updated_at: r.get::<UpdatedAt, _>("updated_at"),
            })
            .collect::<Vec<_>>()
            .into_boxed_slice())
    }

    pub async fn list_user<'a>(&self, tx: &mut Tx<'a>, query: RuleQueryUser) -> RepoResult<Box<[Rule]>> {
        Ok(sqlx::query("select * from solana.rule where user_id = $1 order by id desc limit $2;")
            .bind(query.user)
            .bind(query.limit)
            .fetch_all(&mut **tx)
            .await?
            .iter()
            .map(|r| Rule {
                id: r.get::<RuleId, _>("id"),
                version: r.get::<RuleVersion, _>("version"),
                name: r.get::<RuleName, _>("name"),
                user: r.get::<UserId, _>("user_id"),
                sequence: r.get::<JsonValue, _>("sequence").into(),
                created_at: r.get::<CreatedAt, _>("created_at"),
                updated_at: r.get::<UpdatedAt, _>("updated_at"),
            })
            .collect::<Vec<_>>()
            .into_boxed_slice())
    }
}

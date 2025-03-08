// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::UserId;
use crate::model::{Rule, RuleId, RuleName, RuleStatus, RuleVersion};
use crate::repo::rule::RuleRepo;
use common::model::{CreatedAt, UpdatedAt};
use common::repo::{RepoResult, Tx};
use sqlx::types::JsonValue;
use sqlx::{query, Row};

impl RuleRepo {
    pub async fn get_by_id<'a>(
        &self,
        tx: &mut Tx<'a>,
        id: impl Into<RuleId> + Send,
    ) -> RepoResult<Rule> {
        Ok(query("select * from solana.rule where id = $1;")
            .bind(id.into())
            .fetch_one(&mut **tx)
            .await
            .map(|r| Rule {
                id: r.get::<RuleId, _>("id"),
                version: r.get::<RuleVersion, _>("version"),
                status: r.get::<RuleStatus, _>("status"),
                name: r.get::<RuleName, _>("name"),
                user: r.get::<UserId, _>("user_id"),
                sequence: r.get::<JsonValue, _>("sequence").into(),
                created_at: r.get::<CreatedAt, _>("created_at"),
                updated_at: r.get::<UpdatedAt, _>("updated_at"),
            })?)
    }
}

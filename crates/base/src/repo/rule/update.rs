// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Rule, RuleId, RuleName, RuleStatus, Sequence, UserId};
use crate::repo::RuleRepo;
use common::repo::{RepoResult, Tx};
use sqlx::types::JsonValue;
use sqlx::{query, Row};

#[derive(Debug)]
pub struct RuleUpdateCmd {
    pub id: RuleId,
    pub user: UserId,
    pub name: RuleName,
    pub sequence: Sequence,
    pub status: RuleStatus,
}

impl RuleRepo {
    pub async fn update<'a>(&self, tx: &mut Tx<'a>, cmd: RuleUpdateCmd) -> RepoResult<Rule> {
        let rule_id = query("update solana.rule set name = $3, status = $4, sequence = $5 where id = $1 and user_id = $2 returning id, updated_at = now();")
            .bind(cmd.id)
            .bind(cmd.user)
            .bind(cmd.name)
            .bind(cmd.status)
            .bind::<JsonValue>(cmd.sequence.into())
            .fetch_one(&mut **tx)
            .await
            .map(|r| r.get::<RuleId, _>("id"))?;

        self.get_by_id(tx, rule_id).await
    }
}

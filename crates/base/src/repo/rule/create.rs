// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Rule, RuleId, RuleName, Sequence};
use crate::model::{RuleStatus, UserId};
use crate::repo::rule::RuleRepo;
use common::repo::{RepoResult, Tx};
use sqlx::types::JsonValue;
use sqlx::{query, Row};

pub struct RuleCreateCmd {
    pub user: UserId,
    pub name: RuleName,
    pub sequence: Sequence,
}

impl RuleRepo {
    pub async fn create<'a>(&self, tx: &mut Tx<'a>, cmd: RuleCreateCmd) -> RepoResult<Rule> {
        let rule_id = query("insert into solana.rule (version, status, name, sequence, user_id) values (1, $1, $2, $3, $4) returning id")
            .bind(RuleStatus::Inactive)
            .bind(cmd.name)
            .bind::<JsonValue>(cmd.sequence.into())
            .bind(cmd.user)
            .fetch_one(&mut **tx)
            .await
            .map(|r| r.get::<RuleId, _>("id"))?;

        self.get_by_id(tx, rule_id).await
    }
}

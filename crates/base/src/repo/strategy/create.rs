// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::UserId;
use crate::model::{Sequence, Strategy, StrategyId, StrategyName};
use crate::repo::strategy::StrategyRepo;
use common::repo::{RepoResult, Tx};
use sqlx::types::JsonValue;
use sqlx::{query, Row};

pub struct StrategyCreateCmd {
    pub user: UserId,
    pub name: StrategyName,
    pub sequence: Sequence,
}

impl StrategyRepo {
    pub async fn create<'a>(&self, tx: &mut Tx<'a>, cmd: StrategyCreateCmd) -> RepoResult<Strategy> {
        let strategy_id = query("insert into nyanbot.strategy (version, name, sequence, user_id) values (1, $1, $2, $3) returning id")
            .bind(cmd.name)
            .bind::<JsonValue>(cmd.sequence.into())
            .bind(cmd.user)
            .fetch_one(&mut **tx)
            .await
            .map(|r| r.get::<StrategyId, _>("id"))?;

        self.get_by_id(tx, strategy_id).await
    }
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Invocation, InvocationId, Sequence};
use crate::model::{StrategyId, UserId};
use crate::repo::invocation::InvocationRepo;
use common::model::TokenPairId;
use common::repo::{RepoResult, Tx};
use sqlx::types::JsonValue;
use sqlx::{query, Row};

pub struct InvocationCreateCmd {
    pub user: UserId,
    pub strategy: StrategyId,
    pub token_pair: TokenPairId,
    pub sequence: Sequence,
}

impl InvocationRepo {
    pub async fn create<'a>(&self, tx: &mut Tx<'a>, cmd: InvocationCreateCmd) -> RepoResult<Invocation> {
        let invocation_id = query("insert into solana.invocation (user_id, strategy_id, token_pair_id, sequence) values ($1, $2, $3, $4) returning id")
            .bind(cmd.user)
            .bind(cmd.strategy)
            .bind(cmd.token_pair)
            .bind::<JsonValue>(cmd.sequence.into())
            .fetch_one(&mut **tx)
            .await
            .map(|r| r.get::<InvocationId, _>("id"))?;

        self.get_by_id(tx, invocation_id).await
    }
}

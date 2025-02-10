// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Invocation, InvocationId};
use crate::model::{StrategyId, UserId};
use crate::repo::invocation::InvocationRepo;
use common::model::{CreatedAt, TokenPairId};
use common::repo::{RepoResult, Tx};
use sqlx::types::JsonValue;
use sqlx::{query, Row};

impl InvocationRepo {
    pub async fn get_by_id<'a>(&self, tx: &mut Tx<'a>, id: impl Into<InvocationId> + Send) -> RepoResult<Invocation> {
        Ok(query("select * from solana.invocation where id = $1;")
            .bind(id.into())
            .fetch_one(&mut **tx)
            .await
            .map(|r| Invocation {
                id: r.get::<InvocationId, _>("id"),
                user: r.get::<UserId, _>("user_id"),
                strategy: r.get::<StrategyId, _>("strategy_id"),
                token_pair: r.get::<TokenPairId, _>("token_pair_id"),
                sequence: r.get::<JsonValue, _>("sequence").into(),
                created_at: r.get::<CreatedAt, _>("created_at"),
            })?)
    }
}

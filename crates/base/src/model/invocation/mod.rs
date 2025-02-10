// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::model::{CreatedAt, TokenPairId};

use crate::model::{Sequence, StrategyId, UserId};
pub use id::*;

mod id;

pub struct Invocation {
    pub id: InvocationId,
    pub strategy: StrategyId,
    pub token_pair: TokenPairId,
    pub user: UserId,
    pub sequence: Sequence,
    pub created_at: CreatedAt,
}

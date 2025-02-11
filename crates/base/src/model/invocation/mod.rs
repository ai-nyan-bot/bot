// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::model::CreatedAt;

use crate::model::{Sequence, RuleId, TokenPairId, UserId};
pub use id::*;

mod id;

pub struct Invocation {
    pub id: InvocationId,
    pub rule: RuleId,
    pub token_pair: TokenPairId,
    pub user: UserId,
    pub next: Option<Sequence>,
    pub created_at: CreatedAt,
}

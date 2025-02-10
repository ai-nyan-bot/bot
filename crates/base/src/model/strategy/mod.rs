// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

pub use crate::model::strategy::id::StrategyId;
pub use crate::model::strategy::name::StrategyName;
pub use crate::model::strategy::version::StrategyVersion;
use crate::model::{Sequence, UserId};
use common::model::{CreatedAt, UpdatedAt};

mod id;
mod name;
mod version;

#[derive(Debug)]
pub struct Strategy {
    pub id: StrategyId,
    pub version: StrategyVersion,
    pub name: StrategyName,
    pub sequence: Sequence,
    pub user: UserId,
    // active
    // executes max 10 times
    // start at
    // maybe ends at
    pub created_at: CreatedAt,
    pub updated_at: UpdatedAt,
}

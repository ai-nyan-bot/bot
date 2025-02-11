// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

pub use crate::model::rule::id::RuleId;
pub use crate::model::rule::name::RuleName;
pub use crate::model::rule::version::RuleVersion;
use crate::model::{Sequence, UserId};
use common::model::{CreatedAt, UpdatedAt};

mod id;
mod name;
mod version;

#[derive(Debug)]
pub struct Rule {
    pub id: RuleId,
    pub version: RuleVersion,
    pub name: RuleName,
    pub sequence: Sequence,
    pub user: UserId,
    // active
    // executes max 10 times
    // start at
    // maybe ends at
    pub created_at: CreatedAt,
    pub updated_at: UpdatedAt,
}

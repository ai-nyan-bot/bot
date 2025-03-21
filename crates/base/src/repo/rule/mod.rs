// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::UserId;
use common::model::Limit;
pub use create::*;
use std::ops::Deref;
use std::sync::Arc;
pub use update::*;

mod count;
mod create;
mod delete;
mod get;
mod list;
mod update;

pub struct RuleQueryAll {
    pub limit: Limit,
}

pub struct RuleQueryUser {
    pub user: UserId,
    pub limit: Limit,
}

#[derive(Debug, Clone)]
pub struct RuleRepo(pub Arc<RuleRepoInner>);

impl Deref for RuleRepo {
    type Target = RuleRepoInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct RuleRepoInner {}

impl Default for RuleRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl RuleRepo {
    pub fn new() -> Self {
        Self(Arc::new(RuleRepoInner {}))
    }
}

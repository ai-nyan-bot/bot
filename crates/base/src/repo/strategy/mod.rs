// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::UserId;
use common::model::Limit;
pub use count::*;
pub use create::*;
pub use delete::*;
pub use get::*;
pub use list::*;
use std::ops::Deref;
use std::sync::Arc;
pub use update::*;

mod count;
mod create;
mod delete;
mod get;
mod list;
mod update;

pub struct StrategyQueryAll {
    pub limit: Limit,
}

pub struct StrategyQueryUser {
    pub user: UserId,
    pub limit: Limit,
}

#[derive(Debug, Clone)]
pub struct StrategyRepo(pub Arc<StrategyRepoInner>);

impl Deref for StrategyRepo {
    type Target = StrategyRepoInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct StrategyRepoInner {}

impl StrategyRepo {
    pub fn new() -> Self {
        Self(Arc::new(StrategyRepoInner {}))
    }
}

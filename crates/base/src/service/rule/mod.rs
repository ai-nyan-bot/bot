// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

pub use create::RuleCreateCmd;
pub use update::RuleUpdateCmd;

mod create;
mod get;
mod list;
mod update;

use crate::repo::RuleRepo;
use sqlx::PgPool;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct RuleService(pub Arc<RuleServiceInner>);

impl Deref for RuleService {
    type Target = RuleServiceInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct RuleServiceInner {
    pool: PgPool,
    repo: RuleRepo,
}

impl RuleService {
    pub fn new(pool: PgPool, repo: RuleRepo) -> Self {
        Self(Arc::new(RuleServiceInner { pool, repo }))
    }

    pub fn testing(pool: PgPool) -> Self {
        Self(Arc::new(RuleServiceInner {
            pool,
            repo: RuleRepo::new(),
        }))
    }
}

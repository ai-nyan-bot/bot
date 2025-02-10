// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod list;
mod create;

use crate::repo::StrategyRepo;
use sqlx::PgPool;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct StrategyService(pub Arc<StrategyServiceInner>);

impl Deref for StrategyService {
    type Target = StrategyServiceInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct StrategyServiceInner {
    pool: PgPool,
    repo: StrategyRepo,
}

impl StrategyService {
    pub fn new(pool: PgPool, repo: StrategyRepo) -> Self {
        Self(Arc::new(StrategyServiceInner { pool, repo }))
    }
}

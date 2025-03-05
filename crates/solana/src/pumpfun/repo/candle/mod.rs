// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::model::Limit;
use std::ops::Deref;
use std::sync::Arc;

mod calculate;
mod calculate_progress;
mod calculate_usd;
mod count;

pub struct CandleQuery {
    pub limit: Limit,
}

#[derive(Debug, Clone)]
pub struct CandleRepo(pub Arc<CandlePairRepoInner>);

impl Deref for CandleRepo {
    type Target = CandlePairRepoInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct CandlePairRepoInner {}

impl CandleRepo {
    pub fn new() -> Self {
        Self(Arc::new(CandlePairRepoInner {}))
    }
}

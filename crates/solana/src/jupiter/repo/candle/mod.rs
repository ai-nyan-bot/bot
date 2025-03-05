// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::model::Limit;
use std::ops::Deref;
use std::sync::Arc;

mod calculate_price;
mod count;
mod calculate_price_usd;

pub struct CandleQuery {
    pub limit: Limit,
}

#[derive(Debug, Clone)]
pub struct CandleRepo(pub Arc<CandleRepoInner>);

impl Deref for CandleRepo {
    type Target = CandleRepoInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct CandleRepoInner {}

impl CandleRepo {
    pub fn new() -> Self {
        Self(Arc::new(CandleRepoInner {}))
    }
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::model::Limit;
use std::ops::Deref;
use std::sync::Arc;

use base::repo::ReadTokenPairRepo;
pub use calculate::*;

mod calculate;
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
pub struct CandlePairRepoInner {
    token_pair_repo: ReadTokenPairRepo,
}

impl CandleRepo {
    pub fn new(token_pair_repo: ReadTokenPairRepo) -> Self {
        Self(Arc::new(CandlePairRepoInner { token_pair_repo }))
    }
}

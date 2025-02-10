// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::model::Limit;
use std::ops::Deref;
use std::sync::Arc;

use crate::repo::solana::ReadTokenPairRepo;
pub use calculate::*;
pub use clean::*;

mod calculate;
mod clean;
mod count;

pub struct SummaryQueryAll {
    pub limit: Limit,
}

#[derive(Debug, Clone)]
pub struct SummaryRepo(pub Arc<SummaryPairRepoInner>);

impl Deref for SummaryRepo {
    type Target = SummaryPairRepoInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct SummaryPairRepoInner {
    token_pair_repo: ReadTokenPairRepo,
}

impl SummaryRepo {
    pub fn new(token_pair_repo: ReadTokenPairRepo) -> Self {
        Self(Arc::new(SummaryPairRepoInner { token_pair_repo }))
    }
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::model::{Limit, Timeframe};
use std::ops::Deref;
use std::sync::Arc;

mod calculate;
mod clean;
mod count;
mod list;

pub struct SummaryQuery {
    pub limit: Limit,
    pub timeframe: Timeframe,
}

#[derive(Debug, Clone)]
pub struct SummaryRepo(pub Arc<SummaryRepoInner>);

impl Deref for SummaryRepo {
    type Target = SummaryRepoInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct SummaryRepoInner {}

impl Default for SummaryRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl SummaryRepo {
    pub fn new() -> Self {
        Self(Arc::new(SummaryRepoInner {}))
    }
}

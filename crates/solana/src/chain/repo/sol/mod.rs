// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::model::Limit;
use std::ops::Deref;
use std::sync::Arc;

mod calculate;

pub struct SolQuery {
    pub limit: Limit,
}

#[derive(Debug, Clone)]
pub struct SolRepo(pub Arc<SolRepoInner>);

impl Deref for SolRepo {
    type Target = SolRepoInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct SolRepoInner {}

impl SolRepo {
    pub fn new() -> Self {
        Self(Arc::new(SolRepoInner {}))
    }
}

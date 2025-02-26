// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod upsert;
mod count;
mod list;

use common::model::Limit;
use std::ops::Deref;
use std::sync::Arc;

pub struct CurveQuery {
    pub limit: Limit,
}

#[derive(Debug, Clone)]
pub struct CurveRepo(pub Arc<CurveRepoInner>);

impl Deref for CurveRepo {
    type Target = CurveRepoInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct CurveRepoInner {}

impl Default for CurveRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl CurveRepo {
    pub fn new() -> Self {
        Self(Arc::new(CurveRepoInner {}))
    }

    pub fn testing() -> Self {
        Self(Arc::new(CurveRepoInner {}))
    }
}

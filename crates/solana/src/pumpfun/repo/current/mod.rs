// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod count;
mod get;
mod list;
mod upsert;

use common::model::Limit;
use std::ops::Deref;
use std::sync::Arc;

pub struct CurrentQuery {
    pub limit: Limit,
}

#[derive(Debug, Clone)]
pub struct CurrentRepo(pub Arc<CurrentRepoInner>);

impl Deref for CurrentRepo {
    type Target = CurrentRepoInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct CurrentRepoInner {}

impl Default for CurrentRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl CurrentRepo {
    pub fn new() -> Self {
        Self(Arc::new(CurrentRepoInner {}))
    }

    pub fn testing() -> Self {
        Self(Arc::new(CurrentRepoInner {}))
    }
}

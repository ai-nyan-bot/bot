// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::model::Limit;
use std::ops::Deref;
use std::sync::Arc;

mod calculate;
mod count;

pub struct TwapQuery {
    pub limit: Limit,
}

#[derive(Debug, Clone)]
pub struct TwapRepo(pub Arc<TwapRepoInner>);

impl Deref for TwapRepo {
    type Target = TwapRepoInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct TwapRepoInner {}

impl TwapRepo {
    pub fn new() -> Self {
        Self(Arc::new(TwapRepoInner {}))
    }
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::UserId;
use common::model::Limit;
pub use create::*;

use std::ops::Deref;
use std::sync::Arc;

mod count;
mod create;
mod get;

pub struct InvocationQueryAll {
    pub limit: Limit,
}

pub struct InvocationQueryUser {
    pub user: UserId,
    pub limit: Limit,
}

#[derive(Debug, Clone)]
pub struct InvocationRepo(pub Arc<InvocationRepoInner>);

impl Deref for InvocationRepo {
    type Target = InvocationRepoInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct InvocationRepoInner {}

impl Default for InvocationRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl InvocationRepo {
    pub fn new() -> Self {
        Self(Arc::new(InvocationRepoInner {}))
    }
}

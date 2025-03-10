// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::repo::{AddressRepo, TokenPairRepo, TokenRepo};
use base::LoadTokenInfo;
use common::model::Limit;
pub use insert::{SlotSwap, SlotSwaps};
use std::ops::Deref;
use std::sync::Arc;

mod count;
mod insert;
mod list;

pub struct SwapQueryAll {
    pub limit: Limit,
}

#[derive(Clone)]
pub struct SwapRepo(pub Arc<SwapRepoInner>);

impl Deref for SwapRepo {
    type Target = SwapRepoInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

pub struct SwapRepoInner {
    token_pair_repo: TokenPairRepo,
    address_repo: AddressRepo,
}

impl SwapRepo {
    pub fn new(token_pair_repo: TokenPairRepo, address_repo: AddressRepo) -> Self {
        Self(Arc::new(SwapRepoInner {
            token_pair_repo,
            address_repo,
        }))
    }

    pub fn testing(loader: Box<dyn LoadTokenInfo>) -> Self {
        Self::new(
            TokenPairRepo::testing(TokenRepo::testing(loader)),
            AddressRepo::new(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct ReadSwapRepo(pub Arc<ReadSwapRepoInner>);

impl Deref for ReadSwapRepo {
    type Target = ReadSwapRepoInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct ReadSwapRepoInner {}

impl Default for ReadSwapRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl ReadSwapRepo {
    pub fn new() -> Self {
        Self(Arc::new(ReadSwapRepoInner {}))
    }
}

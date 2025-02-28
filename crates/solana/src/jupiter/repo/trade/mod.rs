// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::Mint;
use base::repo::{AddressRepo, TokenPairRepo, TokenRepo};
use base::LoadTokenInfo;
use common::model::Limit;
pub use insert::{SlotTrade, SlotTrades};
use std::ops::Deref;
use std::sync::Arc;

mod count;
mod insert;
mod list;

pub struct TradeQueryAll {
    pub limit: Limit,
}

#[derive(Debug, Clone)]
pub struct TradeRepo<L: LoadTokenInfo<Mint>>(pub Arc<TradePairRepoInner<L>>);

impl<L: LoadTokenInfo<Mint>> Deref for TradeRepo<L> {
    type Target = TradePairRepoInner<L>;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct TradePairRepoInner<L: LoadTokenInfo<Mint>> {
    token_pair_repo: TokenPairRepo<L>,
    address_repo: AddressRepo,
}

impl<L: LoadTokenInfo<Mint>> TradeRepo<L> {
    pub fn new(token_pair_repo: TokenPairRepo<L>, address_repo: AddressRepo) -> Self {
        Self(Arc::new(TradePairRepoInner {
            token_pair_repo,
            address_repo,
        }))
    }

    pub fn testing(loader: L) -> Self {
        Self::new(
            TokenPairRepo::testing(TokenRepo::testing(loader)),
            AddressRepo::new(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct ReadTradeRepo(pub Arc<ReadTradeRepoInner>);

impl Deref for ReadTradeRepo {
    type Target = ReadTradeRepoInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct ReadTradeRepoInner {}

impl Default for ReadTradeRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl ReadTradeRepo {
    pub fn new() -> Self {
        Self(Arc::new(ReadTradeRepoInner {}))
    }
}

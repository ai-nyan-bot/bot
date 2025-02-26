// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::repo::{AddressRepo, TokenPairRepo};
use base::LoadTokenInfo;
use common::model::Limit;
pub use insert::{SlotTrade, SlotTrades};
use std::ops::Deref;
use std::sync::Arc;

mod count;
mod insert;

pub struct TradeQueryAll {
    pub limit: Limit,
}

#[derive(Debug, Clone)]
pub struct TradeRepo<L: LoadTokenInfo>(pub Arc<TradePairRepoInner<L>>);

impl<L: LoadTokenInfo> Deref for TradeRepo<L> {
    type Target = TradePairRepoInner<L>;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct TradePairRepoInner<L: LoadTokenInfo> {
    token_pair_repo: TokenPairRepo<L>,
    address_repo: AddressRepo,
}

impl<L: LoadTokenInfo> TradeRepo<L> {
    pub fn new(token_pair_repo: TokenPairRepo<L>, address_repo: AddressRepo) -> Self {
        Self(Arc::new(TradePairRepoInner {
            token_pair_repo,
            address_repo,
        }))
    }
}

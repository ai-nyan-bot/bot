// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::model::Limit;
use std::ops::Deref;
use std::sync::Arc;

use crate::repo::solana::{AddressRepo, TokenPairRepo};
use crate::token_info::LoadTokenInfo;
pub use insert::{SlotTrade, SlotTrades};

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
            address_repo: address_repo,
        }))
    }
}

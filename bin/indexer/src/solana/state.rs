// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::repo::{AddressRepo, TokenPairRepo, TokenRepo};
use solana::token_info::rpc::RpcTokenInfoLoader;
use sqlx::PgPool;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct State(pub Arc<StateInner>);

impl Deref for State {
    type Target = StateInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct StateInner {
    pub pool: PgPool,
    pub token_repo: TokenRepo<RpcTokenInfoLoader>,
    pub token_pair_repo: TokenPairRepo<RpcTokenInfoLoader>,
    pub wallet_repo: AddressRepo,
    pub pumpfun_trade_repo: solana::pumpfun::repo::TradeRepo<RpcTokenInfoLoader>,
    pub pumpfun_curve_repo: solana::pumpfun::repo::CurveRepo,
    pub jupiter_trade_repo: solana::jupiter::repo::TradeRepo<RpcTokenInfoLoader>,
}

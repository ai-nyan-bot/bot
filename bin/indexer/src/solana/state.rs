// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use sqlx::PgPool;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone)]
pub struct State(pub Arc<StateInner>);

impl Deref for State {
    type Target = StateInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

pub struct StateInner {
    pub pool: PgPool,
    // pub token_repo: TokenRepo<RpcTokenInfoLoader>,
    // pub token_pair_repo: TokenPairRepo<RpcTokenInfoLoader>,
    // pub wallet_repo: AddressRepo,
    pub pumpfun_swap_repo: solana::pumpfun::repo::SwapRepo,
    pub pumpfun_curve_repo: solana::pumpfun::repo::CurveRepo,
    pub jupiter_swap_repo: solana::jupiter::repo::SwapRepo,
}

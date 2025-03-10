// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::Mint;
use base::LoadTokenInfo;
use sqlx::PgPool;
use std::ops::Deref;
use std::sync::Arc;
use base::repo::TokenRepo;

#[derive(Debug, Clone)]
pub struct State<L: LoadTokenInfo<Mint>>(pub Arc<StateInner<L>>);

impl<L: LoadTokenInfo<Mint>> Deref for State<L> {
    type Target = StateInner<L>;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct StateInner<L: LoadTokenInfo<Mint>> {
    pub pool: PgPool,
    // pub token_repo: TokenRepo<RpcTokenInfoLoader>,
    // pub token_pair_repo: TokenPairRepo<RpcTokenInfoLoader>,
    // pub wallet_repo: AddressRepo,
    pub pumpfun_swap_repo: solana::pumpfun::repo::SwapRepo<L>,
    pub pumpfun_curve_repo: solana::pumpfun::repo::CurveRepo,
    pub jupiter_swap_repo: solana::jupiter::repo::SwapRepo<L>,
}

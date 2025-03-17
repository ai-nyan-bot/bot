// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod summary;

use crate::pumpfun::repo::{CurrentRepo, SummaryRepo};
use base::repo::{TokenPairRepo, TokenRepo};
use base::test::NeverCalledTokenInfoLoader;
use sqlx::PgPool;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone)]
pub struct TokenService(pub Arc<TokenServiceInner>);

impl Deref for TokenService {
    type Target = TokenServiceInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

pub struct TokenServiceInner {
    pool: PgPool,
    pair: TokenPairRepo,
    current: CurrentRepo,
    summary: SummaryRepo,
}

impl TokenService {
    pub fn new(
        pool: PgPool,
        token_pair_repo: TokenPairRepo,
        curve_repo: CurrentRepo,
        summary_repo: SummaryRepo,
    ) -> Self {
        Self(Arc::new(TokenServiceInner {
            pool,
            pair: token_pair_repo,
            current: curve_repo,
            summary: summary_repo,
        }))
    }

    pub fn testing(pool: PgPool) -> Self {
        Self(Arc::new(TokenServiceInner {
            pool,
            pair: TokenPairRepo::testing(TokenRepo::testing(Box::new(
                NeverCalledTokenInfoLoader {},
            ))),
            current: CurrentRepo::testing(),
            summary: SummaryRepo::new(),
        }))
    }
}

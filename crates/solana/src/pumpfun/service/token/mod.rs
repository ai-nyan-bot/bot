// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod summary;

use crate::pumpfun::repo::{CurveRepo, SummaryRepo};
use base::repo::ReadTokenPairRepo;
use sqlx::PgPool;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TokenService(pub Arc<TokenServiceInner>);

impl Deref for TokenService {
    type Target = TokenServiceInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct TokenServiceInner {
    pool: PgPool,
    token_pair_repo: ReadTokenPairRepo,
    curve_repo: CurveRepo,
    summary_repo: SummaryRepo,
}

impl TokenService {
    pub fn new(
        pool: PgPool,
        token_pair_repo: ReadTokenPairRepo,
        curve_repo: CurveRepo,
        summary_repo: SummaryRepo,
    ) -> Self {
        Self(Arc::new(TokenServiceInner {
            pool,
            token_pair_repo,
            curve_repo,
            summary_repo,
        }))
    }

    pub fn testing(pool: PgPool) -> Self {
        Self(Arc::new(TokenServiceInner {
            pool,
            token_pair_repo: ReadTokenPairRepo::testing(),
            curve_repo: CurveRepo::testing(),
            summary_repo: SummaryRepo::new(),
        }))
    }
}

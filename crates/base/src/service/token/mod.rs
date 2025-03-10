// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::repo::{TokenPairRepo, TokenRepo};
use crate::test::NeverCalledTokenInfoLoader;
use sqlx::PgPool;
use std::ops::Deref;
use std::sync::Arc;

mod get;

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
    token_pair_repo: TokenPairRepo,
}

impl TokenService {
    pub fn new(pool: PgPool, token_pair_repo: TokenPairRepo) -> Self {
        Self(Arc::new(TokenServiceInner {
            pool,
            token_pair_repo,
        }))
    }

    pub fn testing(pool: PgPool) -> Self {
        Self(Arc::new(TokenServiceInner {
            pool,
            token_pair_repo: TokenPairRepo::testing(TokenRepo::testing(Box::new(
                NeverCalledTokenInfoLoader {},
            ))),
        }))
    }
}

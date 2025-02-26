// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::repo::{ReadTokenPairRepo, ReadTokenRepo};
use sqlx::PgPool;
use std::ops::Deref;
use std::sync::Arc;

mod get;

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
}

impl TokenService {
    pub fn new(pool: PgPool, token_pair_repo: ReadTokenPairRepo) -> Self {
        Self(Arc::new(TokenServiceInner {
            pool,
            token_pair_repo,
        }))
    }

    pub fn testing(pool: PgPool) -> Self {
        Self(Arc::new(TokenServiceInner {
            pool,
            token_pair_repo: ReadTokenPairRepo::new(ReadTokenRepo::new()),
        }))
    }
}

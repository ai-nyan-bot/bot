// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Mint, Token, TokenId};
use crate::repo::cache::Cache;
use common::model::Limit;
pub use insert::TokenToInsert;
use std::ops::Deref;
use std::sync::Arc;

use crate::test::NeverCalledTokenInfoLoader;
use crate::LoadTokenInfo;

mod count;
mod get;
mod get_or_populate;
mod insert;
mod list;
mod list_or_populate;
mod populate;
mod shared;

pub struct TokenQuery {
    pub limit: Limit,
}

#[derive(Clone)]
pub struct TokenRepo(Arc<TokenRepoInner>);

impl Deref for TokenRepo {
    type Target = TokenRepoInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

pub struct TokenRepoInner {
    info_loader: Box<dyn LoadTokenInfo>,
    cache: Cache<TokenId, Mint, Token>,
}

impl TokenRepo {
    pub fn new(info_loader: Box<dyn LoadTokenInfo>) -> Self {
        Self(Arc::new(TokenRepoInner {
            info_loader,
            cache: Cache::default(),
        }))
    }

    pub fn new_read_only() -> Self {
        Self::new(Box::new(NeverCalledTokenInfoLoader {}))
    }

    pub fn testing(info_loader: Box<dyn LoadTokenInfo>) -> Self {
        Self(Arc::new(TokenRepoInner {
            info_loader,
            cache: Cache::default(),
        }))
    }

    pub fn testing_no_token_info() -> Self {
        Self(Arc::new(TokenRepoInner {
            info_loader: Box::new(NeverCalledTokenInfoLoader {}),
            cache: Cache::default(),
        }))
    }
}

impl TokenRepo {
    pub async fn populate_cache(&self, tokens: impl Iterator<Item = &Token>) {
        self.cache
            .put_all(tokens.map(|t| (t.id, t.mint.clone(), t.clone())))
            .await
    }
}

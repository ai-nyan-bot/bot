// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::ops::Deref;
use std::sync::Arc;

mod count;
mod get;
mod get_or_populate;
mod insert;
mod list;
mod list_or_populate;
mod shared;

use crate::model::{TokenId, TokenPair, TokenPairId, TokenPairMint};
use crate::repo::cache::Cache;
use crate::repo::TokenRepo;
use common::model::Limit;

pub struct TokenPairQuery {
    pub limit: Limit,
}

#[derive(Debug, Clone)]
pub struct CachedTokenPair {
    pub id: TokenPairId,
    pub base_id: TokenId,
    pub quote_id: TokenId,
}

#[derive(Clone)]
pub struct TokenPairRepo(pub Arc<TokenPairRepoInner>);

impl Deref for TokenPairRepo {
    type Target = TokenPairRepoInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

pub struct TokenPairRepoInner {
    token_repo: TokenRepo,
    cache: Cache<TokenPairId, TokenPairMint, CachedTokenPair>,
}

impl TokenPairRepo {
    pub fn new(token_repo: TokenRepo) -> Self {
        Self(Arc::new(TokenPairRepoInner {
            token_repo,
            cache: Cache::default(),
        }))
    }

    pub fn testing(token_repo: TokenRepo) -> Self {
        Self(Arc::new(TokenPairRepoInner {
            token_repo,
            cache: Cache::default(),
        }))
    }
}

impl TokenPairRepo {
    pub async fn populate_cache(&self, pairs: impl Iterator<Item = &TokenPair>) {
        self.cache
            .put_all(pairs.map(|pair| {
                (
                    pair.id,
                    (pair.base.mint.clone(), pair.quote.mint.clone()),
                    CachedTokenPair {
                        id: pair.id,
                        base_id: pair.base.id,
                        quote_id: pair.quote.id,
                    },
                )
            }))
            .await;
    }
}

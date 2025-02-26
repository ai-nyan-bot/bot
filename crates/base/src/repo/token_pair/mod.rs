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
use crate::repo::{ReadTokenRepo, TokenRepo};
use crate::LoadTokenInfo;
use common::model::Limit;
use crate::repo::cache::Cache;

pub struct TokenPairQuery {
    pub limit: Limit,
}

#[derive(Debug, Clone)]
pub struct CachedTokenPair {
    pub id: TokenPairId,
    pub mint: TokenPairMint,
    pub base_id: TokenId,
    pub quote_id: TokenId,
}

#[derive(Debug, Clone)]
pub struct TokenPairRepo<L: LoadTokenInfo>(pub Arc<TokenPairRepoInner<L>>);

impl<L: LoadTokenInfo> Deref for TokenPairRepo<L> {
    type Target = TokenPairRepoInner<L>;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct TokenPairRepoInner<L: LoadTokenInfo> {
    token_repo: TokenRepo<L>,
    read: ReadTokenPairRepo,
}

impl<L: LoadTokenInfo> TokenPairRepo<L> {
    pub fn new(token_repo: TokenRepo<L>, read: ReadTokenPairRepo) -> Self {
        Self(Arc::new(TokenPairRepoInner { token_repo, read }))
    }

    pub fn testing(token_repo: TokenRepo<L>) -> Self {
        Self(Arc::new(TokenPairRepoInner {
            token_repo,
            read: ReadTokenPairRepo::new(ReadTokenRepo::new()),
        }))
    }
}

#[derive(Debug, Clone)]
pub struct ReadTokenPairRepo(pub Arc<ReadTokenPairRepoInner>);

impl Deref for ReadTokenPairRepo {
    type Target = ReadTokenPairRepoInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct ReadTokenPairRepoInner {
    token_repo: ReadTokenRepo,
    cache: Cache<TokenPairId, TokenPairMint, CachedTokenPair>,
}

impl ReadTokenPairRepo {
    pub fn new(read_token_repo: ReadTokenRepo) -> Self {
        Self(Arc::new(ReadTokenPairRepoInner {
            token_repo: read_token_repo,
            cache: Cache::default(),
        }))
    }
}

impl ReadTokenPairRepo {
    pub async fn populate_cache(&self, pairs: impl Iterator<Item = &TokenPair>) {
        self.cache
            .put_all(pairs.map(|pair| {
                (
                    pair.id,
                    (pair.base.mint.clone(), pair.quote.mint.clone()),
                    CachedTokenPair {
                        id: pair.id,
                        mint: (pair.base.mint.clone(), pair.quote.mint.clone()),
                        base_id: pair.base.id,
                        quote_id: pair.quote.id,
                    },
                )
            }))
            .await;
    }
}

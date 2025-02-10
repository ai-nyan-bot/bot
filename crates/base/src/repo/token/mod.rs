// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::model::Limit;
use common::model::{Token, TokenId, TokenMint};
use common::repo::cache::Cache;
use std::ops::Deref;
use std::sync::Arc;

use crate::LoadTokenInfo;
pub use count::*;
pub use get::*;
pub use get_or_populate::*;
pub use insert::*;
pub use list::*;
pub use list_or_populate::*;

mod count;
mod get;
mod get_or_populate;
mod insert;
mod list;
mod list_or_populate;
mod shared;

pub struct TokenQuery {
    pub limit: Limit,
}

#[derive(Debug, Clone)]
pub struct TokenRepo<L: LoadTokenInfo>(pub Arc<TokenRepoInner<L>>);

impl<L: LoadTokenInfo> Deref for TokenRepo<L> {
    type Target = TokenRepoInner<L>;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct TokenRepoInner<L: LoadTokenInfo> {
    info_loader: L,
    read: ReadTokenRepo,
}

impl<L: LoadTokenInfo> TokenRepo<L> {
    pub fn new(info_loader: L, read: ReadTokenRepo) -> Self {
        Self(Arc::new(TokenRepoInner { info_loader, read }))
    }

    pub fn testing(info_loader: L) -> Self {
        Self(Arc::new(TokenRepoInner {
            info_loader,
            read: ReadTokenRepo::new(),
        }))
    }
}

#[derive(Debug, Clone)]
pub struct ReadTokenRepo(pub Arc<ReadTokenRepoInner>);

impl Deref for ReadTokenRepo {
    type Target = ReadTokenRepoInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct ReadTokenRepoInner {
    cache: Cache<TokenId, TokenMint, Token>,
}

impl ReadTokenRepo {
    pub fn new() -> Self {
        Self(Arc::new(ReadTokenRepoInner { cache: Cache::default() }))
    }
}

impl ReadTokenRepo {
    pub async fn populate_cache(&self, tokens: impl Iterator<Item = &Token>) {
        self.cache.put_all(tokens.map(|t| (t.id.clone(), t.mint.clone(), t.clone()))).await
    }
}

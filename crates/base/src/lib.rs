#![deny(warnings)]

use crate::model::{Decimals, TokenMint, TokenName, TokenSymbol};
use async_trait::async_trait;
use futures_util::future::join_all;

pub mod model;
pub mod repo;
pub mod service;
pub mod test;

#[async_trait]
pub trait LoadTokenInfo: Send + Sync {
    async fn load(&self, mint: impl Into<TokenMint> + Send) -> Option<TokenInfo>;
}

pub async fn load_all<L: LoadTokenInfo>(
    loader: &L,
    mints: impl IntoIterator<Item = impl Into<TokenMint>>,
) -> Vec<Option<TokenInfo>> {
    let handles = mints
        .into_iter()
        .map(|mint| async move { loader.load(mint.into()).await });

    join_all(handles).await
}

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub mint: TokenMint,
    pub name: TokenName,
    pub symbol: TokenSymbol,
    pub decimals: Decimals,
}

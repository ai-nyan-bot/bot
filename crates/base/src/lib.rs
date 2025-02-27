#![cfg_attr(not(debug_assertions), deny(warnings))]

use crate::model::{Decimals, Description, Mint, Name, Supply, Symbol, Uri};
use async_trait::async_trait;
use futures_util::future::join_all;

pub mod model;
pub mod repo;
pub mod service;
pub mod test;

#[async_trait]
pub trait LoadTokenInfo: Send + Sync {
    async fn load(&self, mint: impl Into<Mint> + Send) -> Option<TokenInfo>;
}

pub async fn load_all<L: LoadTokenInfo>(
    loader: &L,
    mints: impl IntoIterator<Item = impl Into<Mint>>,
) -> Vec<Option<TokenInfo>> {
    let handles = mints
        .into_iter()
        .map(|mint| async move { loader.load(mint.into()).await });

    join_all(handles).await
}

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub mint: Mint,
    pub name: Name,
    pub symbol: Symbol,
    pub decimals: Decimals,
    pub supply: Supply,
    pub description: Option<Description>,
    pub metadata: Option<Uri>,
    pub image: Option<Uri>,
    pub website: Option<Uri>,
}

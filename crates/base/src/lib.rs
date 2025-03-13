#![cfg_attr(not(debug_assertions), deny(warnings))]

use crate::model::{Amount, Decimals, Description, Mint, Name, Symbol, Uri};
use async_trait::async_trait;
use futures_util::future::join_all;
use log::debug;
use tokio::time::Instant;

pub mod model;
pub mod repo;
pub mod service;
pub mod test;

#[async_trait]
pub trait LoadTokenInfo: Send + Sync {
    async fn load(&self, mint: Mint) -> Option<TokenInfo>;
}

pub async fn load_all(
    loader: &dyn LoadTokenInfo,
    mints: impl IntoIterator<Item = impl Into<Mint>>,
) -> Vec<Option<TokenInfo>> {
    let start = Instant::now();

    let handles = mints
        .into_iter()
        .map(|mint| async move { loader.load(mint.into()).await });

    let result = join_all(handles).await;

    debug!(
        "Downloading {} token infos took {} ms",
        result.len(),
        start.elapsed().as_millis()
    );

    result
}

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub mint: Option<Mint>,
    pub name: Option<Name>,
    pub symbol: Option<Symbol>,
    pub decimals: Option<Decimals>,
    pub supply: Option<Amount>,
    pub description: Option<Description>,
    pub metadata: Option<Uri>,
    pub image: Option<Uri>,
    pub website: Option<Uri>,
}

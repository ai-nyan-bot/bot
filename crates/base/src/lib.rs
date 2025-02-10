use async_trait::async_trait;
use common::model::{Decimals, TokenMint, TokenName, TokenSymbol};

pub mod model;
pub mod repo;
pub mod service;
pub mod test;

#[async_trait]
pub trait LoadTokenInfo: Send + Sync {
    async fn load(&self, mint: impl Into<TokenMint> + Send) -> Option<TokenInfo>;
}

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub mint: TokenMint,
    pub name: TokenName,
    pub symbol: TokenSymbol,
    pub decimals: Decimals,
}

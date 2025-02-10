// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

pub mod rpc;
pub mod test;

use async_trait::async_trait;
use common::model::{Decimals, TokenMint, TokenName, TokenSymbol};

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

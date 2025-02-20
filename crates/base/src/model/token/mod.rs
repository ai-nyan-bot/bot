// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

pub use amount::*;
pub use decimals::Decimals;
pub use id::{TokenId, TokenPairId};
pub use mint::{determine_mints, TokenMint, TokenPairMint};
pub use name::TokenName;
pub use price::*;
use std::fmt::{Display, Formatter, Write};
pub use symbol::TokenSymbol;
pub use trades::*;
pub use uri::TokenUri;
pub use volume::*;

mod amount;
mod decimals;
mod id;
mod mint;
mod name;
mod price;
mod symbol;
mod trades;
mod uri;
mod volume;

#[derive(Debug, Clone)]
pub struct Token {
    pub id: TokenId,
    pub mint: TokenMint,
    pub name: TokenName,
    pub symbol: TokenSymbol,
    pub decimals: Decimals,
}

#[derive(Debug, Clone)]
pub struct TokenPair {
    pub id: TokenPairId,
    pub base: Token,
    pub quote: Token,
}

impl Display for TokenPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}/{}", self.base.symbol, self.quote.symbol))
    }
}

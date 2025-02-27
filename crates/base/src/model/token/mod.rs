// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

pub use amount::*;
pub use decimals::Decimals;
pub use description::Description;
pub use id::{TokenId, TokenPairId};
pub use mint::{determine_mints, Mint, TokenPairMint};
pub use name::Name;
pub use percent::Percent;
pub use price::*;
use std::fmt::{Display, Formatter};
pub use supply::Supply;
pub use symbol::Symbol;
pub use trades::*;
pub use uri::Uri;
pub use volume::*;

mod amount;
mod decimals;
mod description;
mod id;
mod mint;
mod name;
mod percent;
mod price;
mod supply;
mod symbol;
mod trades;
mod uri;
mod volume;

#[derive(Debug, Clone)]
pub struct Token {
    pub id: TokenId,
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

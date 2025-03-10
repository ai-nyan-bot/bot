// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::AddressId;
pub use amount::*;
use common::model::BlockId;
pub use decimals::Decimals;
pub use description::Description;
pub use id::{TokenId, TokenPairId};
pub use mint::{determine_mints, Mint, TokenPairMint};
pub use name::Name;
use std::fmt::{Display, Formatter};
pub use symbol::Symbol;
pub use uri::Uri;

mod amount;
mod decimals;
mod description;
mod id;
mod mint;
mod name;
mod symbol;
mod uri;

#[derive(Debug, Clone)]
pub struct Token {
    pub id: TokenId,
    pub mint: Mint,
    pub name: Option<Name>,
    pub symbol: Option<Symbol>,
    pub decimals: Decimals,
    pub supply: Option<DecimalAmount>,
    pub description: Option<Description>,
    pub metadata: Option<Uri>,
    pub image: Option<Uri>,
    pub website: Option<Uri>,
    pub creator: Option<AddressId>,
    pub block: Option<BlockId>,
}

#[derive(Debug, Clone)]
pub struct TokenPair {
    pub id: TokenPairId,
    pub base: Token,
    pub quote: Token,
}

impl TokenPair {
    pub fn symbol(&self) -> Symbol {
        Symbol(format!(
            "{}/{}",
            self.base.symbol.as_ref().unwrap(),
            self.quote.symbol.as_ref().unwrap()
        ))
    }
}

impl Display for TokenPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.symbol()))
    }
}

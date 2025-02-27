// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::model::PublicKey;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::fmt::{Display, Formatter};

pub const USDC_MINT_STR: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
pub const USDT_MINT_STR: &str = "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB";
pub const WSOL_MINT_STR: &str = "So11111111111111111111111111111111111111112";

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Mint(pub PublicKey);

impl Display for Mint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<&str> for Mint {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<&Mint> for Mint {
    fn eq(&self, other: &&Mint) -> bool {
        self.0 == other.0
    }
}

impl AsRef<Mint> for Mint {
    fn as_ref(&self) -> &Mint {
        self
    }
}

impl Mint {
    pub fn new(value: impl Into<String>) -> Self {
        Self(PublicKey(value.into()))
    }
}

impl From<String> for Mint {
    fn from(value: String) -> Self {
        Self(PublicKey(value))
    }
}

impl From<&str> for Mint {
    fn from(value: &str) -> Self {
        Self(PublicKey(value.to_string()))
    }
}

impl From<Pubkey> for Mint {
    fn from(value: Pubkey) -> Self {
        Self(PublicKey(value.to_string()))
    }
}

impl From<Mint> for Pubkey {
    fn from(value: Mint) -> Self {
        value.0.into()
    }
}

impl From<&Mint> for Mint {
    fn from(value: &Mint) -> Self {
        value.clone()
    }
}

impl Mint {
    pub fn usdc() -> Self {
        USDC_MINT_STR.to_string().into()
    }

    pub fn usdt() -> Self {
        USDT_MINT_STR.to_string().into()
    }

    pub fn wsol() -> Self {
        WSOL_MINT_STR.to_string().into()
    }
}

pub type TokenPairMint = (Mint, Mint);

pub fn determine_mints(a: impl Into<Mint>, b: impl Into<Mint>) -> Option<(Mint, Mint)> {
    let a = a.into();
    let b = b.into();
    if a == USDC_MINT_STR {
        Some((b, a))
    } else if b == USDC_MINT_STR {
        Some((a, b))
    } else if a == USDT_MINT_STR {
        Some((b, a))
    } else if b == USDT_MINT_STR {
        Some((a, b))
    } else if a == WSOL_MINT_STR {
        Some((b, a))
    } else if b == WSOL_MINT_STR {
        Some((a, b))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {

    mod determine_mints {
        use crate::model::token::mint::{determine_mints, USDC_MINT_STR, USDT_MINT_STR, WSOL_MINT_STR};

        #[test]
        fn test_wsol_usdc() {
            let Some((base, quote)) = determine_mints(USDC_MINT_STR, WSOL_MINT_STR) else {
                panic!()
            };
            assert_eq!(base, WSOL_MINT_STR);
            assert_eq!(quote, USDC_MINT_STR);
        }

        #[test]
        fn test_usdc_wsol() {
            let Some((base, quote)) = determine_mints(WSOL_MINT_STR, USDC_MINT_STR) else {
                panic!()
            };
            assert_eq!(base, WSOL_MINT_STR);
            assert_eq!(quote, USDC_MINT_STR);
        }

        #[test]
        fn test_usdt_bonk() {
            let Some((base, quote)) = determine_mints(USDT_MINT_STR, BONK_MINT_STR) else {
                panic!()
            };
            assert_eq!(base, BONK_MINT_STR);
            assert_eq!(quote, USDT_MINT_STR);
        }

        #[test]
        fn test_bonk_wsol() {
            let Some((base, quote)) = determine_mints(BONK_MINT_STR, WSOL_MINT_STR) else {
                panic!()
            };
            assert_eq!(base, BONK_MINT_STR);
            assert_eq!(quote, WSOL_MINT_STR);
        }

        #[test]
        fn test_unsupported_quote() {
            let result = determine_mints(BONK_MINT_STR, BONK_MINT_STR);
            assert_eq!(result, None)
        }

        const BONK_MINT_STR: &str = "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263";
    }
}

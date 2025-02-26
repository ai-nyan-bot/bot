// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/nhuxhr/pumpfun-rs (MIT License).
// Original MIT License Copyright (c) nhuxhr 2024.


pub(crate) mod ids {
    use solana_sdk::pubkey::Pubkey;
    use solana_sdk::pubkey;

    pub(crate) const CPI_ID: Pubkey = pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");
}

pub(crate) mod seeds {
    /// Seed for the global state PDA
    pub(crate) const GLOBAL_SEED: &[u8] = b"global";

    // Seed for the mint authority PDA
    // pub(crate) const MINT_AUTHORITY_SEED: &[u8] = b"mint-authority";

    /// Seed for bonding curve PDAs
    pub(crate) const BONDING_CURVE_SEED: &[u8] = b"bonding-curve";

    // Seed for metadata PDAs
    // pub(crate) const METADATA_SEED: &[u8] = b"metadata";
}

pub(crate) mod accounts {
    // use solana_sdk::{pubkey, pubkey::Pubkey};

    // pub(crate) const PUMPFUN: Pubkey = pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");
    // pub(crate) const MPL_TOKEN_METADATA: Pubkey = pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
    // pub(crate) const EVENT_AUTHORITY: Pubkey = pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1");
    // pub(crate) const SYSTEM_PROGRAM: Pubkey = pubkey!("11111111111111111111111111111111");
    // pub(crate) const TOKEN_PROGRAM: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    // pub(crate) const ASSOCIATED_TOKEN_PROGRAM: Pubkey = pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
    // pub(crate) const RENT: Pubkey = pubkey!("SysvarRent111111111111111111111111111111111");
}

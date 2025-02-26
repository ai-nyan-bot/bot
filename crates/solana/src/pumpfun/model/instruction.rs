// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{
    Amount, PublicKey, TokenMint, TokenName,
    TokenSymbol, TokenUri,
};
use common::model::Timestamp;

#[derive(Debug)]
pub enum Instruction {
    Create {
        name: TokenName,
        symbol: TokenSymbol,
        uri: TokenUri,
        mint: TokenMint,
        bonding_curve: PublicKey,
        user: PublicKey,
    },
    Trade {
        mint: TokenMint,
        sol_amount: Amount,
        token_amount: Amount,
        is_buy: bool,
        user: PublicKey,
        timestamp: Timestamp,
        virtual_sol_reserves: Amount,
        virtual_token_reserves: Amount,
    },
}


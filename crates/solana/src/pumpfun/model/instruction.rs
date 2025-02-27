// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{
    Amount, PublicKey, Mint, Name,
    Symbol, Uri,
};
use common::model::Timestamp;

#[derive(Debug)]
pub enum Instruction {
    Create {
        name: Name,
        symbol: Symbol,
        uri: Uri,
        mint: Mint,
        bonding_curve: PublicKey,
        user: PublicKey,
    },
    Trade {
        mint: Mint,
        sol_amount: Amount,
        token_amount: Amount,
        is_buy: bool,
        user: PublicKey,
        timestamp: Timestamp,
        virtual_sol_reserves: Amount,
        virtual_token_reserves: Amount,
    },
}


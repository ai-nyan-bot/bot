// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/nhuxhr/pumpfun-rs (MIT License).
// Original MIT License Copyright (c) nhuxhr 2024.

use base::model::Amount;

pub struct Curve {
    /// Unique identifier for the bonding curve
    pub discriminator: u64,
    /// Virtual token reserves used for price calculations
    pub virtual_token_reserves: Amount,
    /// Virtual SOL reserves used for price calculations
    pub virtual_sol_reserves: Amount,
    /// Actual token reserves available for trading
    pub real_token_reserves: Amount,
    /// Actual SOL reserves available for trading
    pub real_sol_reserves: Amount,
    /// Total supply of tokens
    pub token_total_supply: Amount,
    /// Whether the bonding curve is complete/finalized
    pub complete: bool,
}

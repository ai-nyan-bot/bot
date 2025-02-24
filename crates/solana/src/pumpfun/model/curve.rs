// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/nhuxhr/pumpfun-rs (MIT License).
// Original MIT License Copyright (c) nhuxhr 2024.

use base::model::{Amount, TokenPairId};

pub struct Curve {
    pub token_pair: TokenPairId,
    pub virtual_token_reserves: Amount,
    pub virtual_sol_reserves: Amount,
    pub real_token_reserves: Amount,
    pub real_sol_reserves: Amount,
    pub token_total_supply: Amount,
    pub fee_basis_points: u64,
    pub progress: f32,
    pub market_cap: u64,
    pub complete: bool,
}

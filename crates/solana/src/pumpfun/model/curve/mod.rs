// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{Amount, Percent, TokenPairId};
use std::ops::{Div, Mul, Sub};

pub use progress::CalculateProgress;

mod progress;

pub struct Curve {
    pub token_pair: TokenPairId,
    pub virtual_token_reserves: Amount,
    pub virtual_sol_reserves: Amount,
    pub real_token_reserves: Amount,
    pub real_sol_reserves: Amount,
    pub token_total_supply: Amount,
    pub progress: Percent,
    pub complete: bool,
}

#[derive(Debug, Clone)]
pub struct CurveInfo {
    pub virtual_token_reserves: Amount,
    pub virtual_sol_reserves: Amount,
    pub real_token_reserves: Amount,
    pub real_sol_reserves: Amount,
    pub token_total_supply: Amount,
    pub complete: bool,
}

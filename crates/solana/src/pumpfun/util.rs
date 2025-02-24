// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/nhuxhr/pumpfun-rs (MIT License).
// Original MIT License Copyright (c) nhuxhr 2024.

use crate::pumpfun::constant::ids::CPI_ID;
use crate::pumpfun::constant::seeds::{BONDING_CURVE_SEED, GLOBAL_SEED};
use solana_sdk::pubkey::Pubkey;

pub(crate) fn global_pda() -> Pubkey {
    let seeds: &[&[u8]; 1] = &[GLOBAL_SEED];
    let program_id: &Pubkey = &CPI_ID;
    Pubkey::find_program_address(seeds, program_id).0
}

pub(crate) fn curve_pda(key: impl Into<Pubkey>) -> Option<Pubkey> {
    let key = key.into();
    let seeds: &[&[u8]; 2] = &[BONDING_CURVE_SEED, key.as_ref()];
    let program_id: &Pubkey = &CPI_ID;
    let pda: Option<(Pubkey, u8)> = Pubkey::try_find_program_address(seeds, program_id);
    pda.map(|pubkey| pubkey.0)
}

/// Calculates the maximum amount to pay when buying tokens, accounting for slippage tolerance
///
/// # Arguments
/// * `amount` - The base amount in lamports (1 SOL = 1,000,000,000 lamports)
/// * `basis_points` - The slippage tolerance in basis points (1% = 100 basis points)
///
/// # Returns
/// The maximum amount to pay, including slippage tolerance
pub fn calculate_with_slippage_buy(amount: u64, basis_points: u64) -> u64 {
    amount + (amount * basis_points) / 10000
}

/// Calculates the minimum amount to receive when selling tokens, accounting for slippage tolerance
///
/// # Arguments
/// * `amount` - The base amount in lamports (1 SOL = 1,000,000,000 lamports)
/// * `basis_points` - The slippage tolerance in basis points (1% = 100 basis points)
///
/// # Returns
/// The minimum amount to receive, accounting for slippage tolerance
pub fn calculate_with_slippage_sell(amount: u64, basis_points: u64) -> u64 {
    amount - (amount * basis_points) / 10000
}

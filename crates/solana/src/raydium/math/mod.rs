// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/raydium-io/raydium-amm (Apache 2.0 License).
// Original Apache 2.0 License Copyright (c) raydium.io 2024.

pub use u128::*;

use crate::raydium;
use crate::raydium::amm::Amm;
use crate::raydium::Error::MathError;
use crate::raydium::{SwapDirection, SwapExecutionMode};

mod u128;

pub fn calc_total_without_take_pnl_no_orderbook(
    pc_amount: u64,
    coin_amount: u64,
    amm: &Amm,
) -> raydium::Result<(u64, u64)> {
    let total_pc_without_take_pnl = pc_amount
        .checked_sub(amm.state_data.need_take_pnl_pc)
        .ok_or(MathError)?;

    let total_coin_without_take_pnl = coin_amount
        .checked_sub(amm.state_data.need_take_pnl_coin)
        .ok_or(MathError)?;

    Ok((total_pc_without_take_pnl, total_coin_without_take_pnl))
}

pub fn swap_with_slippage(
    pc_vault_amount: u64,
    coin_vault_amount: u64,
    swap_fee_numerator: u64,
    swap_fee_denominator: u64,
    direction: SwapDirection,
    input_amount: u64,
    mode: SwapExecutionMode,
    slippage_bps: u64,
) -> raydium::Result<(u64, u64)> {
    let output_amount = swap_exact_amount(
        pc_vault_amount,
        coin_vault_amount,
        swap_fee_numerator,
        swap_fee_denominator,
        direction,
        input_amount,
        mode,
    )?;

    let output_amount_with_slippage = match mode {
        SwapExecutionMode::ExactIn => min_amount_with_slippage(output_amount, slippage_bps),
        SwapExecutionMode::ExactOut => max_amount_with_slippage(output_amount, slippage_bps),
    };

    Ok((output_amount, output_amount_with_slippage))
}

fn swap_exact_amount(
    pc_vault_amount: u64,
    coin_vault_amount: u64,
    swap_fee_numerator: u64,
    swap_fee_denominator: u64,
    swap_direction: SwapDirection,
    input_amount: u64,
    mode: SwapExecutionMode,
) -> raydium::Result<u64> {
    let output_amount = if mode == SwapExecutionMode::ExactIn {
        let swap_fee = U128::from(input_amount)
            .checked_mul(swap_fee_numerator.into())
            .unwrap()
            .checked_ceil_div(swap_fee_denominator.into())
            .unwrap()
            .0;

        let swap_in_after_deduct_fee = U128::from(input_amount).checked_sub(swap_fee).unwrap();

        let swap_amount_out = swap_token_amount_base_in(
            swap_in_after_deduct_fee,
            pc_vault_amount.into(),
            coin_vault_amount.into(),
            swap_direction,
        )?
        .as_u64();

        swap_amount_out
    } else {
        assert_eq!(mode, SwapExecutionMode::ExactOut);

        let swap_in_before_add_fee = swap_token_amount_base_out(
            input_amount.into(),
            pc_vault_amount.into(),
            coin_vault_amount.into(),
            swap_direction,
        );

        let swap_in_after_add_fee = swap_in_before_add_fee?
            .checked_mul(swap_fee_denominator.into())
            .unwrap()
            .checked_ceil_div(
                (swap_fee_denominator
                    .checked_sub(swap_fee_numerator)
                    .unwrap())
                .into(),
            )
            .unwrap()
            .0
            .as_u64();

        swap_in_after_add_fee
    };

    Ok(output_amount)
}

pub fn swap_token_amount_base_in(
    amount_in: U128,
    total_pc_without_take_pnl: U128,
    total_coin_without_take_pnl: U128,
    swap_direction: SwapDirection,
) -> raydium::Result<U128> {
    match swap_direction {
        SwapDirection::Coin2PC => {
            // (x + delta_x) * (y + delta_y) = x * y
            // (coin + amount_in) * (pc - amount_out) = coin * pc
            // => amount_out = pc - coin * pc / (coin + amount_in)
            // => amount_out = ((pc * coin + pc * amount_in) - coin * pc) / (coin + amount_in)
            // => amount_out =  pc * amount_in / (coin + amount_in)
            let denominator = total_coin_without_take_pnl
                .checked_add(amount_in)
                .ok_or(MathError)?;
            Ok(total_pc_without_take_pnl
                .checked_mul(amount_in)
                .ok_or(MathError)?
                .checked_div(denominator)
                .ok_or(MathError)?)
        }
        SwapDirection::PC2Coin => {
            // (x + delta_x) * (y + delta_y) = x * y
            // (pc + amount_in) * (coin - amount_out) = coin * pc
            // => amount_out = coin - coin * pc / (pc + amount_in)
            // => amount_out = (coin * pc + coin * amount_in - coin * pc) / (pc + amount_in)
            // => amount_out = coin * amount_in / (pc + amount_in)
            let denominator = total_pc_without_take_pnl
                .checked_add(amount_in)
                .ok_or(MathError)?;
            Ok(total_coin_without_take_pnl
                .checked_mul(amount_in)
                .ok_or(MathError)?
                .checked_div(denominator)
                .ok_or(MathError)?)
        }
    }
}

pub fn swap_token_amount_base_out(
    amount_out: U128,
    total_pc_without_take_pnl: U128,
    total_coin_without_take_pnl: U128,
    swap_direction: SwapDirection,
) -> raydium::Result<U128> {
    match swap_direction {
        SwapDirection::Coin2PC => {
            // (x + delta_x) * (y + delta_y) = x * y
            // (coin + amount_in) * (pc - amount_out) = coin * pc
            // => amount_in = coin * pc / (pc - amount_out) - coin
            // => amount_in = (coin * pc - pc * coin + amount_out * coin) / (pc - amount_out)
            // => amount_in = (amount_out * coin) / (pc - amount_out)
            let denominator = total_pc_without_take_pnl
                .checked_sub(amount_out)
                .ok_or(MathError)?;
            Ok(total_coin_without_take_pnl
                .checked_mul(amount_out)
                .ok_or(MathError)?
                .checked_ceil_div(denominator)
                .ok_or(MathError)?
                .0)
        }
        SwapDirection::PC2Coin => {
            // (x + delta_x) * (y + delta_y) = x * y
            // (pc + amount_in) * (coin - amount_out) = coin * pc
            // => amount_out = coin - coin * pc / (pc + amount_in)
            // => amount_out = (coin * pc + coin * amount_in - coin * pc) / (pc + amount_in)
            // => amount_out = coin * amount_in / (pc + amount_in)

            // => amount_in = coin * pc / (coin - amount_out) - pc
            // => amount_in = (coin * pc - pc * coin + pc * amount_out) / (coin - amount_out)
            // => amount_in = (pc * amount_out) / (coin - amount_out)
            let denominator = total_coin_without_take_pnl
                .checked_sub(amount_out)
                .ok_or(MathError)?;
            Ok(total_pc_without_take_pnl
                .checked_mul(amount_out)
                .ok_or(MathError)?
                .checked_ceil_div(denominator)
                .ok_or(MathError)?
                .0)
        }
    }
}

pub const TEN_THOUSAND: u64 = 10_000;

fn max_amount_with_slippage(input_amount: u64, slippage_bps: u64) -> u64 {
    input_amount
        .checked_mul(slippage_bps.checked_add(TEN_THOUSAND).unwrap())
        .unwrap()
        .checked_div(TEN_THOUSAND)
        .unwrap()
}

fn min_amount_with_slippage(input_amount: u64, slippage_bps: u64) -> u64 {
    input_amount
        .checked_mul(TEN_THOUSAND.checked_sub(slippage_bps).unwrap())
        .unwrap()
        .checked_div(TEN_THOUSAND)
        .unwrap()
}

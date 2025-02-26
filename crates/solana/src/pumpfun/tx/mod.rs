// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/nhuxhr/pumpfun-rs (MIT License).
// Original MIT License Copyright (c) nhuxhr 2024.

//! Instructions for interacting with the Pump.fun program.
//!
//! This module contains instruction builders for creating Solana instructions to interact with the
//! Pump.fun program. Each function takes the required accounts and instruction data and returns a
//! properly formatted Solana instruction.
//!
//! # Instructions
//!
//! - `create`: Instruction to create a new token with an associated bonding curve.
//! - `buy`: Instruction to buy tokens from a bonding curve by providing SOL.
//! - `sell`: Instruction to sell tokens back to the bonding curve in exchange for SOL.
// 
// use crate::pumpfun::constant::accounts::{
//     ASSOCIATED_TOKEN_PROGRAM, EVENT_AUTHORITY, PUMPFUN, RENT, SYSTEM_PROGRAM, TOKEN_PROGRAM,
// };
// use crate::pumpfun::util::{curve_pda, global_pda};
// use solana_sdk::{
//     instruction::{AccountMeta, Instruction},
//     pubkey::Pubkey,
//     signature::Keypair,
//     signer::Signer,
// };
// use spl_associated_token_account::get_associated_token_address;
// 
// mod buy;
// mod sell;
// 
// const BUY: [u8; 8] = [0x66, 0x06, 0x3d, 0x12, 0x01, 0xda, 0xeb, 0xea];
// const SELL: [u8; 8] = [0x33, 0xe6, 0x85, 0xa4, 0x01, 0x7f, 0x83, 0xad];
// 
// pub struct BuyInstructionArgs {
//     // _amount: u64,
//     pub amount: u64,
//     // _max_sol_cost: u64,
//     pub max_sol_cost: u64,
// }
// 
// /// Creates an instruction to buy tokens from a bonding curve
// ///
// /// Buys tokens by providing SOL. The amount of tokens received is calculated based on
// /// the bonding curve formula. A portion of the SOL is taken as a fee and sent to the
// /// fee recipient account.
// ///
// /// # Arguments
// ///
// /// * `payer` - Keypair that will provide the SOL to buy tokens
// /// * `mint` - Public key of the token mint to buy
// /// * `fee_recipient` - Public key of the account that will receive the transaction fee
// /// * `args` - Buy instruction data containing the SOL amount and maximum acceptable token price
// ///
// /// # Returns
// ///
// /// Returns a Solana instruction that when executed will buy tokens from the bonding curve
// pub fn create_buy_instruction(
//     payer: &Keypair,
//     mint: &Pubkey,
//     fee_recipient: &Pubkey,
//     args: BuyInstructionArgs,
// ) -> Instruction {
//     let bonding_curve: Pubkey = curve_pda(*mint).unwrap();
// 
//     // let expected_size = 1 + 8 + 8;
//     let mut data = Vec::with_capacity(24);
//     // data.extend_from_slice(&[9u8]);
//     data.extend_from_slice(&BUY);
//     data.extend_from_slice(&args.amount.to_le_bytes());
//     data.extend_from_slice(&args.max_sol_cost.to_le_bytes());
//     // assert_eq!(data.len(), expected_size);
// 
//     Instruction {
//         program_id: PUMPFUN,
//         data,
//         accounts: vec![
//             AccountMeta::new_readonly(global_pda(), false),
//             AccountMeta::new(*fee_recipient, false),
//             AccountMeta::new_readonly(*mint, false),
//             AccountMeta::new(bonding_curve, false),
//             AccountMeta::new(get_associated_token_address(&bonding_curve, mint), false),
//             AccountMeta::new(get_associated_token_address(&payer.pubkey(), mint), false),
//             AccountMeta::new(payer.pubkey(), true),
//             AccountMeta::new_readonly(SYSTEM_PROGRAM, false),
//             AccountMeta::new_readonly(TOKEN_PROGRAM, false),
//             AccountMeta::new_readonly(RENT, false),
//             AccountMeta::new_readonly(EVENT_AUTHORITY, false),
//             AccountMeta::new_readonly(PUMPFUN, false),
//         ],
//     }
// }
// 
// pub struct SellInstructionArgs {
//     // _amount,
//     pub amount: u64,
//     // _min_sol_output,
//     pub min_sol_output: u64,
// }
// 
// /// Creates an instruction to sell tokens back to a bonding curve
// ///
// /// Sells tokens back to the bonding curve in exchange for SOL. The amount of SOL received
// /// is calculated based on the bonding curve formula. A portion of the SOL is taken as
// /// a fee and sent to the fee recipient account.
// ///
// /// # Arguments
// ///
// /// * `payer` - Keypair that owns the tokens to sell
// /// * `mint` - Public key of the token mint to sell
// /// * `fee_recipient` - Public key of the account that will receive the transaction fee
// /// * `args` - Sell instruction data containing token amount and minimum acceptable SOL output
// ///
// /// # Returns
// ///
// /// Returns a Solana instruction that when executed will sell tokens to the bonding curve
// pub fn create_sell_instruction(
//     payer: &Keypair,
//     mint: &Pubkey,
//     fee_recipient: &Pubkey,
//     args: SellInstructionArgs,
// ) -> Instruction {
//     let bonding_curve: Pubkey = curve_pda(*mint).unwrap();
// 
//     // let expected_size = 1 + 8 + 8;
//     let mut data = Vec::with_capacity(24);
//     // data.extend_from_slice(&[9u8]);
//     data.extend_from_slice(&SELL);
//     data.extend_from_slice(&args.amount.to_le_bytes());
//     data.extend_from_slice(&args.min_sol_output.to_le_bytes());
//     // assert_eq!(data.len(), expected_size);
// 
//     Instruction {
//         program_id: PUMPFUN,
//         data,
//         accounts: vec![
//             AccountMeta::new_readonly(global_pda(), false),
//             AccountMeta::new(*fee_recipient, false),
//             AccountMeta::new_readonly(*mint, false),
//             AccountMeta::new(bonding_curve, false),
//             AccountMeta::new(get_associated_token_address(&bonding_curve, mint), false),
//             AccountMeta::new(get_associated_token_address(&payer.pubkey(), mint), false),
//             AccountMeta::new(payer.pubkey(), true),
//             AccountMeta::new_readonly(SYSTEM_PROGRAM, false),
//             AccountMeta::new_readonly(ASSOCIATED_TOKEN_PROGRAM, false),
//             AccountMeta::new_readonly(TOKEN_PROGRAM, false),
//             AccountMeta::new_readonly(EVENT_AUTHORITY, false),
//             AccountMeta::new_readonly(PUMPFUN, false),
//         ],
//     }
// }
// 
// /// Calculates the maximum amount to pay when buying tokens, accounting for slippage tolerance
// ///
// /// # Arguments
// /// * `amount` - The base amount in lamports (1 SOL = 1,000,000,000 lamports)
// /// * `basis_points` - The slippage tolerance in basis points (1% = 100 basis points)
// ///
// /// # Returns
// /// The maximum amount to pay, including slippage tolerance
// pub fn calculate_with_slippage_buy(amount: u64, basis_points: u64) -> u64 {
//     amount + (amount * basis_points) / 10000
// }
// 
// /// Calculates the minimum amount to receive when selling tokens, accounting for slippage tolerance
// ///
// /// # Arguments
// /// * `amount` - The base amount in lamports (1 SOL = 1,000,000,000 lamports)
// /// * `basis_points` - The slippage tolerance in basis points (1% = 100 basis points)
// ///
// /// # Returns
// /// The minimum amount to receive, accounting for slippage tolerance
// pub fn calculate_with_slippage_sell(amount: u64, basis_points: u64) -> u64 {
//     amount - (amount * basis_points) / 10000
// }

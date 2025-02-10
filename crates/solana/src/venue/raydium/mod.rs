// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/0xcrust/raydium-swap (MIT License).
// Original MIT License Copyright (c) 0xcrust 2024.
//
// This file includes portions of code from https://github.com/raydium-io/raydium-amm (Apache 2.0 License).
// Original Apache 2.0 License Copyright (c) raydium.io 2024.

use std::fmt::Display;
use std::str::FromStr;

use common::model::PublicKey;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;

pub use error::Error;
pub use http::v3::client::*;

use crate::rpc::RpcClient;
use crate::venue::raydium::amm::AmmKeys;

mod amm;
mod error;
mod http;
mod ix;
mod math;
mod quote;
mod swap;

pub(crate) const RAYDIUM_LIQUIDITY_POOL_V4_PROGRAM_ID: Pubkey =
    pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");
pub(crate) const RAYDIUM_AUTHORITY: Pubkey =
    pubkey!("5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1");

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u64)]
pub enum SwapDirection {
    /// Input token pc, output token coin
    PC2Coin = 1u64,
    /// Input token coin, output token pc
    Coin2PC = 2u64,
}

#[derive(Clone, Debug)]
pub struct RaydiumSwap {
    pub input_token_mint: PublicKey,
    pub output_token_mint: PublicKey,
    pub slippage_bps: u16,
    pub amount: u64,
    pub mode: SwapExecutionMode,
    pub market: Option<PublicKey>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SwapExecutionMode {
    ExactIn,
    ExactOut,
}

#[derive(Clone, Debug)]
pub struct RaydiumQuote {
    /// The address of the amm pool
    pub market: Pubkey,
    /// The input mint
    pub input_mint: Pubkey,
    /// The output mint,
    pub output_mint: Pubkey,
    /// The amount specified
    pub input_amount: u64,
    /// The output amount
    pub output_amount: u64,
    /// The output amount with slippage
    pub output_amount_with_slippage: u64,
    // /// The input mint decimals
    // pub input_mint_decimals: u8,
    // /// The output mint decimals
    // pub output_mint_decimals: u8,
    /// Amm keys
    pub keys: AmmKeys,
    // Market keys
    // pub market_keys: MarketKeys,
}

#[derive(Copy, Clone, Debug, Default)]
pub enum ComputeUnitLimits {
    #[default]
    Dynamic,
    Fixed(u64),
}

#[derive(Copy, Clone, Debug)]
pub enum PriorityFeeConfig {
    DynamicMultiplier(u64),
    FixedCuPrice(u64),
    // JitoTip(u64),
}

#[derive(Clone, Debug, Default)]
pub struct SwapConfigOverrides {
    pub priority_fee: Option<PriorityFeeConfig>,
    pub cu_limits: Option<ComputeUnitLimits>,
    pub wrap_and_unwrap_sol: Option<bool>,
    pub destination_token_account: Option<Pubkey>,
    pub as_legacy_transaction: Option<bool>,
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Raydium {
    http_client: HttpClient,
    rpc_client: RpcClient,
    config: SwapConfig,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct SwapConfig {
    pub priority_fee: Option<PriorityFeeConfig>,
    pub cu_limits: Option<ComputeUnitLimits>,
    pub wrap_and_unwrap_sol: Option<bool>,
    pub as_legacy_transaction: Option<bool>,
}

impl Default for Raydium {
    fn default() -> Self {
        Self {
            http_client: HttpClient::default(),
            rpc_client: RpcClient::default(),
            config: SwapConfig::default(),
        }
    }
}

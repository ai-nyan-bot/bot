// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::Deserialize;
use spl_token::solana_program::pubkey::Pubkey;

use crate::venue::raydium::http::v3::serde::field_as_string;

#[derive(Debug, Deserialize)]
pub struct PoolKey {
    #[serde(rename = "programId", with = "field_as_string")]
    pub program_id: Pubkey,
    #[serde(with = "field_as_string")]
    pub id: Pubkey,
    #[serde(rename = "mintA")]
    pub mint_a: PoolKeyMint,
    #[serde(rename = "mintB")]
    pub mint_b: PoolKeyMint,
    #[serde(rename = "lookupTableAccount")]
    pub lookup_table_account: String,
    #[serde(rename = "openTime")]
    pub open_time: String,
    pub vault: PoolKeyVault,
    pub config: Option<PoolKeyConfig>,
    #[serde(rename = "rewardInfos")]
    pub reward_infos: Option<Vec<PoolKeyRewardInfo>>,
}

#[derive(Debug, Deserialize)]
pub struct PoolKeyMint {
    #[serde(rename = "chainId")]
    pub chain_id: u32,
    #[serde(with = "field_as_string")]
    pub address: Pubkey,
    #[serde(rename = "programId")]
    pub program_id: String,
    #[serde(rename = "logoURI")]
    pub logo_uri: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u32,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PoolKeyVault {
    #[serde(rename = "A", with = "field_as_string")]
    pub a: Pubkey,
    #[serde(rename = "B", with = "field_as_string")]
    pub b: Pubkey,
}

#[derive(Debug, Deserialize)]
pub struct PoolKeyConfig {
    pub id: String,
    pub index: u32,
    #[serde(rename = "protocolFeeRate")]
    pub protocol_fee_rate: u64,
    #[serde(rename = "tradeFeeRate")]
    pub trade_fee_rate: u32,
    #[serde(rename = "tickSpacing")]
    pub tick_spacing: u32,
    #[serde(rename = "fundFeeRate")]
    pub fund_fee_rate: u32,
    #[serde(rename = "defaultRange")]
    pub default_range: f64,
    #[serde(rename = "defaultRangePoint")]
    pub default_range_point: Vec<f64>,
}

#[derive(Debug, Deserialize)]
pub struct PoolKeyRewardInfo {
    pub mint: PoolKeyMint,
    pub vault: String,
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::Deserialize;
use solana_sdk::pubkey::Pubkey;

use crate::raydium::http::v3::serde::field_as_string;

#[derive(Debug, Deserialize)]
pub struct PoolPage {
    #[serde(rename = "count")]
    pub count: u32,
    #[serde(rename = "data")]
    pub data: Vec<Pool>,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
}

#[derive(Debug, Deserialize)]
pub struct Pool {
    #[serde(rename = "type")]
    pub pool_type: String,
    #[serde(rename = "programId", with = "field_as_string")]
    pub program_id: Pubkey,
    #[serde(with = "field_as_string")]
    pub id: Pubkey,
    #[serde(rename = "mintA")]
    pub mint_a: PoolMint,
    #[serde(rename = "mintB")]
    pub mint_b: PoolMint,
    #[serde(rename = "rewardDefaultPoolInfos")]
    pub reward_default_pool_infos: Option<String>,
    #[serde(rename = "rewardDefaultInfos")]
    pub reward_default_infos: Option<Vec<PoolRewardInfo>>,
    pub price: f64,
    #[serde(rename = "mintAmountA")]
    pub mint_amount_a: f64,
    #[serde(rename = "mintAmountB")]
    pub mint_amount_b: f64,
    #[serde(rename = "feeRate")]
    pub fee_rate: f64,
    #[serde(rename = "openTime")]
    pub open_time: String,
    pub tvl: f64,
    pub day: Option<PoolStats>,
    pub week: Option<PoolStats>,
    pub month: Option<PoolStats>,
    #[serde(rename = "pooltype")]
    pub pool_type_list: Vec<String>,
    #[serde(rename = "farmUpcomingCount")]
    pub farm_upcoming_count: u32,
    #[serde(rename = "farmOngoingCount")]
    pub farm_ongoing_count: u32,
    #[serde(rename = "farmFinishedCount")]
    pub farm_finished_count: u32,
    pub config: Option<PoolConfig>,
    #[serde(rename = "burnPercent")]
    pub burn_percent: f64,
}

#[derive(Debug, Deserialize)]
pub struct PoolMint {
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
pub struct PoolRewardInfo {
    pub mint: PoolMint,
    #[serde(rename = "perSecond")]
    pub per_second: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
}

#[derive(Debug, Deserialize)]
pub struct PoolStats {
    pub volume: f64,
    #[serde(rename = "volumeQuote")]
    pub volume_quote: f64,
    #[serde(rename = "volumeFee")]
    pub volume_fee: f64,
    pub apr: f64,
    #[serde(rename = "feeApr")]
    pub fee_apr: f64,
    #[serde(rename = "priceMin")]
    pub price_min: f64,
    #[serde(rename = "priceMax")]
    pub price_max: f64,
    #[serde(rename = "rewardApr")]
    pub reward_apr: Vec<f64>,
}

#[derive(Debug, Deserialize)]
pub struct PoolConfig {
    pub id: String,
    pub index: u32,
    #[serde(rename = "protocolFeeRate")]
    pub protocol_fee_rate: u64,
    #[serde(rename = "tradeFeeRate")]
    pub trade_fee_rate: u32,
    #[serde(rename = "tickSpacing")]
    pub tick_spacing: Option<u32>,
    #[serde(rename = "fundFeeRate")]
    pub fund_fee_rate: u32,
    #[serde(rename = "defaultRange")]
    pub default_range: Option<f64>,
    #[serde(rename = "defaultRangePoint")]
    pub default_range_point: Option<Vec<f64>>,
}

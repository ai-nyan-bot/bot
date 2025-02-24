// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/0xcrust/raydium-swap (MIT License).
// Original MIT License Copyright (c) 0xcrust 2024.


use std::fmt;

use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

use crate::raydium::http::v3::serde::field_as_string;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenList {
    #[serde(default)]
    pub mint_list: Vec<Token>,
    #[serde(default)]
    pub blacklist: Vec<Token>,
    #[serde(default)]
    pub whitelist: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub chain_id: u64,
    #[serde(with = "field_as_string")]
    pub address: Pubkey,
    #[serde(default, with = "field_as_string")]
    pub program_id: Pubkey,
    #[serde(default, rename = "logoURI")]
    pub logo_uri: String,
    #[serde(default)]
    pub symbol: String,
    #[serde(default)]
    pub name: String,
    pub decimals: u8,
    #[serde(default)]
    pub tags: Vec<TokenTag>,
    pub extensions: Extensions,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    pub coingecko_id: Option<String>,
    pub fee_config: Option<TransferFeeDatabaseType>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferFeeDatabaseType {
    #[serde(with = "field_as_string")]
    pub transfer_fee_config_authority: Pubkey,
    #[serde(with = "field_as_string")]
    pub withdraw_withheld_authority: Pubkey,
    pub withheld_amount: String,
    pub older_transfer_fee: TransferFee,
    pub newer_transfer_fee: TransferFee,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferFee {
    #[serde(with = "field_as_string")]
    pub epoch: u64,
    #[serde(with = "field_as_string")]
    pub maximum_fee: u64,
    pub transfer_fee_basis_points: u16,
}

#[derive(Debug)]
pub enum TokenTagError {
    InvalidTag(String),
}

impl fmt::Display for TokenTagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenTagError::InvalidTag(tag) => write!(f, "Invalid token tag: {}", tag),
        }
    }
}

impl std::error::Error for TokenTagError {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TokenTag {
    #[serde(rename = "hasFreeze")]
    HasFreeze,
    #[serde(rename = "hasTransferFee")]
    HasTransferFee,
    #[serde(rename = "token-2022")]
    Token2022,
    #[serde(rename = "community")]
    Community,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(untagged)]
    UnrecognizedTag(String),
}

impl std::str::FromStr for TokenTag {
    type Err = TokenTagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hasFreeze" => Ok(Self::HasFreeze),
            "hasTransferFee" => Ok(Self::HasTransferFee),
            "token-2022" => Ok(Self::Token2022),
            "community" => Ok(Self::Community),
            "unknown" => Ok(Self::Unknown),
            x => Ok(Self::UnrecognizedTag(x.to_string())),
        }
    }
}

impl std::fmt::Display for TokenTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HasFreeze => f.write_str("hasFreeze"),
            Self::HasTransferFee => f.write_str("hasTransferFee"),
            Self::Token2022 => f.write_str("token-2022"),
            Self::Community => f.write_str("community"),
            Self::Unknown => f.write_str("unknown"),
            Self::UnrecognizedTag(x) => f.write_fmt(format_args!("{}", x)),
        }
    }
}

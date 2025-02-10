// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/0xcrust/raydium-swap (MIT License).
// Original MIT License Copyright (c) 0xcrust 2024.

use crate::venue::raydium::http::v3::response::token::Token;
use ::serde::{Deserialize, Serialize};

pub mod error;
pub mod pool;
pub mod pool_keys;
pub mod token;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub id: String,
    pub success: bool,
    pub data: T,
}

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
pub struct PoolsPage<T> {
    pub count: u64,
    pub has_next_page: bool,
    #[serde(rename = "data")]
    pub pools: Vec<T>,
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/0xcrust/raydium-swap (MIT License).
// Original MIT License Copyright (c) 0xcrust 2024.


use ::serde::{Deserialize, Serialize};

pub mod client;
pub mod response;
pub mod serde;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PoolType {
    #[default]
    All,
    Standard,
    Concentrated,
    AllFarm,
    StandardFarm,
    ConcentratedFarm,
}

impl std::fmt::Display for PoolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PoolType::All => f.write_str("all"),
            PoolType::Standard => f.write_str("standard"),
            PoolType::Concentrated => f.write_str("concentrated"),
            PoolType::AllFarm => f.write_str("allFarm"),
            PoolType::StandardFarm => f.write_str("standardFarm"),
            PoolType::ConcentratedFarm => f.write_str("concentratedFarm"),
        }
    }
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/0xcrust/raydium-swap (MIT License).
// Original MIT License Copyright (c) 0xcrust 2024.

use serde::de::DeserializeOwned;
use solana_client::client_error::reqwest;
use solana_sdk::pubkey::Pubkey;

use crate::raydium::http::error::HttpError;
use crate::raydium::http::v3::response::error::ErrorResponse;
use crate::raydium::http::v3::response::pool::PoolPage;
use crate::raydium::http::v3::response::pool_keys::PoolKey;
use crate::raydium::http::v3::response::token::{Token, TokenList};
use crate::raydium::http::v3::response::Response;
use crate::raydium::http::v3::PoolType;

#[derive(Clone, Debug, Default)]
pub struct ListPoolRequest {
    pub pool_type: PoolType,
    pub pool_sort: PoolSort,
    pub sort_type: PoolSortOrder,
    pub page_size: u16,
    pub page: u16,
    pub mint_one: Pubkey,
    pub mint_two: Pubkey,
}

#[derive(Clone, Debug, Default)]
pub enum PoolSort {
    #[default]
    Liquidity,
    Volume24h,
    Volume7d,
    Volume30d,
    Fee24h,
    Fee7d,
    Fee30d,
    Apr24h,
    Apr7d,
    Apr30d,
}

impl std::fmt::Display for PoolSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PoolSort::Liquidity => f.write_str("liquidity"),
            PoolSort::Volume24h => f.write_str("volume24h"),
            PoolSort::Volume7d => f.write_str("volume7d"),
            PoolSort::Volume30d => f.write_str("volume30d"),
            PoolSort::Fee24h => f.write_str("fee24h"),
            PoolSort::Fee7d => f.write_str("fee7d"),
            PoolSort::Fee30d => f.write_str("fee30d"),
            PoolSort::Apr24h => f.write_str("apr24h"),
            PoolSort::Apr7d => f.write_str("apr7d"),
            PoolSort::Apr30d => f.write_str("apr30d"),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum PoolSortOrder {
    Ascending,
    #[default]
    Descending,
}

impl std::fmt::Display for PoolSortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PoolSortOrder::Ascending => f.write_str("asc"),
            PoolSortOrder::Descending => f.write_str("desc"),
        }
    }
}

pub struct HttpClient {
    base_url: String,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self {
            base_url: "https://api-v3.raydium.io".to_string(),
        }
    }
}

impl HttpClient {
    pub async fn list_token(&self) -> Result<TokenList, HttpError> {
        let url = format!("{}/mint/list", &self.base_url);
        Ok(Self::execute(reqwest::get(url).await?).await?.data)
    }

    pub async fn list_tokens_by_mint(&self, mints: Vec<String>) -> Result<Vec<Token>, HttpError> {
        let mints = mints.join(",");
        let url = format!("{}/mint/ids?mints={}", &self.base_url, mints);
        Ok(Self::execute(reqwest::get(url).await?).await?.data)
    }

    pub async fn list_pools(&self, req: impl Into<ListPoolRequest>) -> Result<PoolPage, HttpError> {
        let req = req.into();

        let url = format!(
            "{}/pools/info/mint?mint1={}&mint2={}&poolType={}&poolSortField={}&sortType={}&pageSize={}&page={}",
            &self.base_url, req.mint_one, req.mint_two, req.pool_type, req.pool_sort, req.sort_type, req.page_size, req.page,
        );

        Ok(Self::execute(reqwest::get(url).await?).await?.data)
    }

    pub async fn list_pool_keys(&self, ids: Vec<String>) -> Result<Vec<PoolKey>, HttpError> {
        let ids = ids.join(",");
        let url = format!("{}/pools/key/ids?ids={}", &self.base_url, ids);
        Ok(Self::execute(reqwest::get(url).await?).await?.data)
    }

    async fn execute<T>(response: reqwest::Response) -> Result<Response<T>, HttpError>
    where
        T: DeserializeOwned,
    {
        let response = response.error_for_status()?;
        let json = response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| HttpError::DeserializationError { message: e.to_string() })?;

        let success = json.get("success").and_then(|v| v.as_bool()).ok_or(HttpError::UnexpectedResponse)?;

        if success {
            Ok(serde_json::from_value::<Response<T>>(json).map_err(|e| HttpError::DeserializationError { message: e.to_string() })?)
        } else {
            Err(serde_json::from_value::<ErrorResponse>(json)
                .map_err(|e| HttpError::DeserializationError { message: e.to_string() })?
                .into())
        }
    }
}

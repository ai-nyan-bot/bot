// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/0xcrust/raydium-swap (MIT License).
// Original MIT License Copyright (c) 0xcrust 2024.

use std::sync::Arc;

use crate::rpc::error::RpcClientError;
use common::model::RpcUrl;
use solana_rpc_client::nonblocking::rpc_client;

mod account;
mod block;
mod error;
mod slot;

#[derive(Clone)]
pub struct RpcClient {
    client: Arc<rpc_client::RpcClient>,
}

impl RpcClient {
    pub fn new(rpc_url: impl Into<RpcUrl>) -> Self {
        Self {
            client: Arc::new(rpc_client::RpcClient::new(rpc_url.into().to_string())),
        }
    }
}

impl Default for RpcClient {
    fn default() -> Self {
        Self {
            client: Arc::new(rpc_client::RpcClient::new("https://api.mainnet-beta.solana.com".to_string())),
        }
    }
}

pub type RpcResult<T> = Result<T, RpcClientError>;

impl RpcClient {}

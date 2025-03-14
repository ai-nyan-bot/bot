// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/0xcrust/raydium-swap (MIT License).
// Original MIT License Copyright (c) 0xcrust 2024.

use crate::model::Slot;
use crate::rpc::block::GetBlockWithConfigFn;
use crate::rpc::error::RpcClientError;
use common::model::RpcUrl;
use log::debug;
use solana_client::rpc_config::RpcBlockConfig;
use solana_rpc_client::nonblocking::rpc_client;
use std::ops::Deref;
use std::sync::Arc;
use tokio::time::Instant;

mod account;
mod block;
mod error;
mod slot;

#[derive(Clone)]
pub struct RpcClient(pub Arc<RpcClientInner>);

impl RpcClient {
    pub fn new(rpc_url: impl Into<RpcUrl>) -> Self {
        Self(Arc::new(RpcClientInner {
            delegate: Arc::new(rpc_client::RpcClient::new(rpc_url.into().to_string())),
            get_block_with_config: Arc::new(
                move |delegate: Arc<rpc_client::RpcClient>, slot: Slot, config: RpcBlockConfig| {
                    Box::pin(async move {
                        let start = Instant::now();
                        let result = delegate.get_block_with_config(slot.0 as u64, config).await;
                        debug!(
                            "download of block {} took {} ms",
                            slot.0,
                            start.elapsed().as_millis()
                        );
                        result
                    })
                },
            ),
        }))
    }
}

impl Default for RpcClient {
    fn default() -> Self {
        Self(Arc::new(RpcClientInner {
            delegate: Arc::new(rpc_client::RpcClient::new(
                "https://api.mainnet-beta.solana.com".to_string(),
            )),
            get_block_with_config: Arc::new(
                move |delegate: Arc<rpc_client::RpcClient>, slot: Slot, config: RpcBlockConfig| {
                    Box::pin(
                        async move { delegate.get_block_with_config(slot.0 as u64, config).await },
                    )
                },
            ),
        }))
    }
}

impl Deref for RpcClient {
    type Target = RpcClientInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
pub struct RpcClientInner {
    pub(crate) delegate: Arc<rpc_client::RpcClient>,
    pub(crate) get_block_with_config: Arc<GetBlockWithConfigFn>,
}

pub type RpcResult<T> = Result<T, RpcClientError>;

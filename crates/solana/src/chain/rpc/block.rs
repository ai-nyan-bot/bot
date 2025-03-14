// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::convert::convert_transaction;
use crate::model::{Block, Slot, Transaction};
use crate::rpc::{RpcClient, RpcResult};
use common::model::{BlockTime, Timestamp};
use log::{error, warn};
use rayon::prelude::*;
use solana_client::client_error::ClientError;
use solana_client::rpc_config::RpcBlockConfig;
use solana_client::rpc_request::RpcError;
use solana_rpc_client::nonblocking::rpc_client;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::{TransactionDetails, UiConfirmedBlock, UiTransactionEncoding};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::task::spawn_blocking;
use tokio::time::{sleep, Instant};
use tracing::debug;

pub(crate) type GetBlockWithConfigFn = dyn Fn(
        Arc<rpc_client::RpcClient>,
        Slot,
        RpcBlockConfig,
    ) -> Pin<Box<dyn Future<Output = Result<UiConfirmedBlock, ClientError>> + Send>>
    + Send
    + Sync;

impl RpcClient {
    pub async fn get_block(&self, slot: impl Into<Slot>) -> RpcResult<Option<Block>> {
        let slot = slot.into();
        loop {
            let result = (self.get_block_with_config)(
                self.delegate.clone(),
                slot,
                RpcBlockConfig {
                    encoding: Some(UiTransactionEncoding::Base58),
                    transaction_details: Some(TransactionDetails::Full),
                    rewards: None,
                    commitment: Some(CommitmentConfig::confirmed()),
                    max_supported_transaction_version: Some(0),
                },
            )
            .await;

            match result {
                Ok(block) => {
                    let start = Instant::now();

                    let transactions: Vec<Transaction> = spawn_blocking(move || {
                        block
                            .transactions
                            .unwrap_or_default()
                            .into_par_iter()
                            .map(convert_transaction)
                            .collect()
                    })
                    .await
                    .unwrap();

                    debug!(
                        "converting {} transactions took {} ms",
                        transactions.len(),
                        start.elapsed().as_millis()
                    );

                    return Ok(Some(Block {
                        slot,
                        timestamp: BlockTime(
                            Timestamp::from_epoch_second(block.block_time.unwrap()).unwrap(),
                        ),
                        transactions,
                    }));
                }
                Err(err) => {
                    error!("{err}");

                    match err.kind {
                        solana_client::client_error::ClientErrorKind::Io(_) => {
                            unimplemented!()
                        }
                        solana_client::client_error::ClientErrorKind::Reqwest(err) => {
                            if err.is_timeout() {
                                warn!(
                                    "time out while downloading {slot} - tries again in 1 second"
                                );
                                sleep(Duration::from_millis(1000)).await;
                            } else {
                                unimplemented!()
                            }
                        }
                        solana_client::client_error::ClientErrorKind::Middleware(_) => {
                            unimplemented!()
                        }
                        solana_client::client_error::ClientErrorKind::RpcError(ref rpc_error) => {
                            match rpc_error {
                                RpcError::RpcRequestError(_) => unimplemented!(),
                                RpcError::RpcResponseError { code, .. } => {
                                    if *code == -32004 {
                                        // Block not available for slot x
                                        return Ok(None);
                                    }

                                    if *code == -32009 {
                                        // Slot xxxxx was skipped, or missing in long-term storage
                                        // that might not be correctly handled, because maybe the node just does not have that block even though it exists
                                        return Ok(None);
                                    }

                                    if *code == -32007 {
                                        // was skipped, or missing due to ledger jump to recent snapshot
                                        return Ok(None);
                                    }
                                    unimplemented!()
                                }
                                RpcError::ParseError(_) => unimplemented!(),
                                RpcError::ForUser(_) => unimplemented!(),
                            }
                        }
                        solana_client::client_error::ClientErrorKind::SerdeJson(_) => {
                            unimplemented!()
                        }
                        solana_client::client_error::ClientErrorKind::SigningError(_) => {
                            unimplemented!()
                        }
                        solana_client::client_error::ClientErrorKind::TransactionError(_) => {
                            unimplemented!()
                        }
                        solana_client::client_error::ClientErrorKind::Custom(_) => unimplemented!(),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::Slot;
    use crate::rpc::{RpcClient, RpcClientInner};
    use common::model::{BlockTime, Timestamp};
    use solana_client::client_error::ClientError;
    use solana_client::rpc_config::RpcBlockConfig;
    use solana_client::rpc_request::{RpcError, RpcResponseErrorData};
    use solana_rpc_client::nonblocking::rpc_client;
    use solana_transaction_status::UiConfirmedBlock;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_ok() {
        let test_instance = RpcClient(Arc::new(RpcClientInner {
            delegate: Arc::new(rpc_client::RpcClient::new("https://".to_string())),
            get_block_with_config: Arc::new(
                move |_delegate: Arc<rpc_client::RpcClient>,
                      _slot: Slot,
                      _config: RpcBlockConfig| {
                    Box::pin(async move {
                        Ok(UiConfirmedBlock {
                            previous_blockhash: "".to_string(),
                            blockhash: "".to_string(),
                            parent_slot: 0,
                            transactions: Some(vec![]),
                            signatures: None,
                            rewards: None,
                            num_reward_partitions: None,
                            block_time: Some(1737344750),
                            block_height: None,
                        })
                    })
                },
            ),
        }));

        let result = test_instance.get_block(1).await.unwrap();
        let result = result.unwrap();
        assert_eq!(
            result.timestamp,
            BlockTime(Timestamp::from_epoch_second(1737344750).unwrap())
        )
    }

    #[tokio::test]
    async fn test_block_not_available() {
        let test_instance = RpcClient(Arc::new(RpcClientInner {
            delegate: Arc::new(rpc_client::RpcClient::new("https://".to_string())),
            get_block_with_config: Arc::new(
                move |_delegate: Arc<rpc_client::RpcClient>,
                      slot: Slot,
                      _config: RpcBlockConfig| {
                    Box::pin(async move {
                        Err(ClientError::from(RpcError::RpcResponseError {
                            code: -32004,
                            message: format!("Block not available for slot {slot}"),
                            data: RpcResponseErrorData::Empty,
                        }))
                    })
                },
            ),
        }));

        let result = test_instance.get_block(1).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_block_does_not_exists() {
        let test_instance = RpcClient(Arc::new(RpcClientInner {
            delegate: Arc::new(rpc_client::RpcClient::new("https://".to_string())),
            get_block_with_config: Arc::new(
                move |_delegate: Arc<rpc_client::RpcClient>,
                      slot: Slot,
                      _config: RpcBlockConfig| {
                    Box::pin(async move {
                        Err(
                            ClientError::from(RpcError::RpcResponseError {
                                code: -32007,
                                message: format!("Slot {slot} was skipped, or missing due to ledger jump to recent snapshot"),
                                data: RpcResponseErrorData::Empty,
                            })
                        )
                    })
                },
            ),
        }));

        let result = test_instance.get_block(1).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_block_does_not_exists_in_long_term_storage() {
        let test_instance = RpcClient(Arc::new(RpcClientInner {
            delegate: Arc::new(rpc_client::RpcClient::new("https://".to_string())),
            get_block_with_config: Arc::new(
                move |_delegate: Arc<rpc_client::RpcClient>,
                      slot: Slot,
                      _config: RpcBlockConfig| {
                    Box::pin(async move {
                        Err(ClientError::from(RpcError::RpcResponseError {
                            code: -32009,
                            message: format!(
                                "Slot {slot} was skipped, or missing in long-term storage"
                            ),
                            data: RpcResponseErrorData::Empty,
                        }))
                    })
                },
            ),
        }));

        let result = test_instance.get_block(1).await.unwrap();
        assert!(result.is_none());
    }
}

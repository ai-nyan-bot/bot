// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::convert::convert_transaction;
use crate::model::{Block, Slot};
use crate::rpc::{RpcClient, RpcResult};
use common::model::Timestamp;
use log::{error, info, trace, warn};
use solana_client::rpc_config::RpcBlockConfig;
use solana_client::rpc_request::RpcError;
use solana_rpc_client::rpc_client::SerializableTransaction;
use solana_sdk::bs58;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::option_serializer::OptionSerializer;
use solana_transaction_status::UiInstruction::{Compiled, Parsed};
use solana_transaction_status::{TransactionDetails, UiTransactionEncoding};

impl RpcClient {
    pub async fn block(&self, slot: impl Into<Slot>) -> RpcResult<Block> {
        let slot = slot.into();
        loop {
            trace!("get block for {slot}");
            match self
                .client
                .get_block_with_config(
                    slot.0 as u64,
                    RpcBlockConfig {
                        encoding: Some(UiTransactionEncoding::Base58),
                        transaction_details: Some(TransactionDetails::Full),
                        rewards: None,
                        commitment: Some(CommitmentConfig::confirmed()),
                        max_supported_transaction_version: Some(0),
                    },
                )
                .await
            {
                Ok(block) => {
                    let Some(transactions) = block.transactions else { unreachable!() };
                    let mut txes = Vec::with_capacity(transactions.len());
                    for tx in transactions {
                        txes.push(convert_transaction(tx))
                    }

                    return Ok(Block {
                        slot,
                        timestamp: Timestamp::from_solana_time(block.block_time.unwrap()),
                        transactions: txes,
                    });
                }
                Err(err) => {
                    error!("{err}");
                    // Error fetching block 315134307, Error { request: Some(GetBlock), kind: RpcError(RpcResponseError { code: -32007, message: "Slot 315136164 was skipped, or missing due to ledger jump to recent snapshot", data: Empty }) }
                    match err.kind {
                        solana_client::client_error::ClientErrorKind::Io(_) => {
                            eprintln!("Error fetching block {}, {:?}", slot, err);
                            // return Err(err.into());
                        }
                        solana_client::client_error::ClientErrorKind::Reqwest(err) => {
                            if err.is_timeout() {
                                warn!("time out while downloading {slot}");
                                // continue;
                            } else {
                                eprintln!("Error fetching block {}, {:?}", slot, err);
                                // panic!();
                            }
                        }
                        solana_client::client_error::ClientErrorKind::Middleware(_) => {
                            eprintln!("Error fetching block {}, {:?}", slot, err);
                            // return Err(err.into());
                        }
                        solana_client::client_error::ClientErrorKind::RpcError(ref rpc_error) => {
                            match rpc_error {
                                RpcError::RpcRequestError(_) => unimplemented!(),
                                RpcError::RpcResponseError { code, message, data } => {
                                    //  { code: -32007, message: "Slot 315136164 was skipped, or missing due to ledger jump to recent snapshot", data: Empty }
                                    if *code == -32007 {
                                        info!("{message}");
                                    }
                                }
                                RpcError::ParseError(_) => unimplemented!(),
                                RpcError::ForUser(_) => unimplemented!(),
                            }

                            eprintln!("Error fetching block {}, {:?}", slot, err);
                            // return Err(err.into());
                        }
                        solana_client::client_error::ClientErrorKind::SerdeJson(_) => {
                            eprintln!("Error fetching block {}, {:?}", slot, err);
                            // return Err(err.into());
                        }
                        solana_client::client_error::ClientErrorKind::SigningError(_) => {
                            eprintln!("Error fetching block {}, {:?}", slot, err);
                            // return Err(err.into());
                        }
                        solana_client::client_error::ClientErrorKind::TransactionError(_) => {
                            eprintln!("Error fetching block {}, {:?}", slot, err);
                            // return Err(err.into());
                        }
                        solana_client::client_error::ClientErrorKind::Custom(_) => {
                            eprintln!("Error fetching block {}, {:?}", slot, err);
                            // return Err(err.into());
                        }
                    }
                }
            }
        }
    }
}

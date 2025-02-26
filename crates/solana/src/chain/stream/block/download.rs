// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Block, Slot};
use crate::rpc::RpcClient;
use crate::stream::{RpcBlockStream, SlotStream};
use common::Signal;
use futures_util::future::join_all;
use log::{debug, error};
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, Semaphore};

pub(crate) async fn download_blocks(
    rpc_client: RpcClient,
    slots: Vec<Slot>,
    concurrency: usize,
    signal: Signal,
) -> Vec<Block> {
    let semaphore = Arc::new(Semaphore::new(concurrency));

    let mut handles = Vec::new();
    let blocks = Arc::new(Mutex::new(BTreeMap::new()));

    for slot in slots {
        let rpc_client = rpc_client.clone();
        let semaphore = semaphore.clone();
        let blocks = blocks.clone();
        let signal = signal.clone();

        let handle = tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            debug!("Downloading block of slot: {}", slot);

            match rpc_client.get_block(slot).await {
                Ok(Some(block)) => {
                    let mut res = blocks.lock().await;
                    res.insert(slot, block);
                }
                Ok(None) => {}
                Err(err) => {
                    error!("Failed to fetch block for slot: {} - {}", slot, err);
                    signal.terminate("RpcBlockStream failed to fetch block");
                }
            }
        });

        handles.push(handle);
    }

    join_all(handles).await;

    let mut blocks = blocks.lock().await;
    let mut result = Vec::with_capacity(blocks.len());
    while let Some((_slot, block)) = blocks.pop_first() {
        result.push(block)
    }
    result
}

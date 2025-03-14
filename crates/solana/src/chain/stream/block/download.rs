// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Block, Slot};
use crate::rpc::RpcClient;
use common::Limiter;
use futures_util::future::join_all;
use log::debug;
use tokio::sync::mpsc::Sender;

pub(crate) enum DownloadResult {
    Ok(Slot, Block),
    Skip(Slot),
    Error(Slot, String),
}

pub(crate) async fn download_blocks(
    rpc_client: RpcClient,
    slots: Vec<Slot>,
    limiter: Limiter,
    tx: Sender<DownloadResult>,
) {
    let mut handles = Vec::new();

    for slot in slots {
        let rpc_client = rpc_client.clone();
        let tx = tx.clone();
        let limiter = limiter.clone();

        handles.push(tokio::spawn(async move {
            limiter.limit().await;
            debug!("start download of block {}", slot);
            match rpc_client.get_block(slot).await {
                Ok(Some(block)) => {
                    tx.send(DownloadResult::Ok(slot, block)).await.unwrap();
                }
                Ok(None) => {
                    tx.send(DownloadResult::Skip(slot)).await.unwrap();
                }
                Err(err) => {
                    tx.send(DownloadResult::Error(slot, err.to_string()))
                        .await
                        .unwrap();
                }
            }
        }));
    }
    join_all(handles).await;
}

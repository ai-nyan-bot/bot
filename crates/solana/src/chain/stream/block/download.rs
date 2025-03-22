// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::solana::{Block, Slot};
use crate::rpc::RpcClient;
use common::Limiter;
use futures_util::future::join_all;
use log::debug;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::Semaphore;
use tokio::time::Instant;

pub(crate) enum DownloadResult {
    Ok(Slot, Block),
    Skip(Slot),
    Error(Slot, String),
}

#[derive(Clone)]
pub(crate) struct Downloader(Arc<DownloaderInner>);

impl Deref for Downloader {
    type Target = DownloaderInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) struct DownloaderInner {
    rpc_client: RpcClient,
    limiter: Limiter,
    tx: Sender<DownloadResult>,
    semaphore: Arc<Semaphore>,
}

impl Downloader {
    pub fn new(rpc_client: RpcClient, tx: Sender<DownloadResult>, concurrency: usize) -> Self {
        Self(Arc::new(DownloaderInner {
            rpc_client,
            limiter: Limiter::new_per_second(concurrency),
            tx,
            semaphore: Arc::new(Semaphore::new(concurrency)),
        }))
    }

    pub(crate) async fn download(&self, slots: Vec<Slot>) {
        let mut handles = Vec::new();

        let rpc_client = self.rpc_client.clone();
        let limiter = self.limiter.clone();
        let tx = self.tx.clone();

        for slot in slots {
            let rpc_client = rpc_client.clone();
            let tx = tx.clone();
            let limiter = limiter.clone();

            let semaphore = self.semaphore.clone();
            handles.push(tokio::spawn(async move {
                let permit = semaphore.acquire().await.unwrap();
                limiter.limit().await;

                let start = Instant::now();
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
                debug!(
                    "download of block {} took {} ms",
                    slot,
                    start.elapsed().as_millis()
                );

                drop(permit);
            }));
        }
        join_all(handles).await;
    }
}

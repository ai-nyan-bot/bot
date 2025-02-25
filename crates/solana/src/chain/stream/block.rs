// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Block, Slot};
use crate::rpc::RpcClient;
use crate::stream::SlotStream;
use async_trait::async_trait;
use common::model::RpcUrl;
use common::{Signal, SignalKind};
use futures_util::future::join_all;
use log::{debug, error, warn};
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::select;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{Mutex, Semaphore};
use tokio::task::JoinHandle;

#[async_trait]
pub trait BlockStream: Send {
    async fn stream<S: SlotStream>(
        self,
        slot_stream: S,
        signal: Signal,
    ) -> (Receiver<Block>, JoinHandle<()>);
}

pub struct RpcBlockStreamConfig {
    pub url: RpcUrl,
    pub concurrency: usize,
}

pub struct RpcBlockStream {
    cfg: RpcBlockStreamConfig,
    tx: Sender<Block>,
    rx: Receiver<Block>,
}

impl RpcBlockStream {
    pub fn new(cfg: RpcBlockStreamConfig) -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        Self { cfg, tx, rx }
    }
}

impl Default for RpcBlockStream {
    fn default() -> Self {
        Self::new(RpcBlockStreamConfig {
            url: "http://api.mainnet-beta.solana.com".into(),
            concurrency: 1,
        })
    }
}

#[async_trait]
impl BlockStream for RpcBlockStream {
    async fn stream<S: SlotStream>(
        self,
        slot_stream: S,
        mut signal: Signal,
    ) -> (Receiver<Block>, JoinHandle<()>) {
        let rpc = RpcClient::new(self.cfg.url);

        let downloader =
            DownloadAndSendBlock::new(rpc.clone(), self.tx, self.cfg.concurrency, signal.clone());

        let mut previous_slot = Slot(0);

        let (mut rx, _) = slot_stream.stream(signal.clone()).await;
        (
            self.rx,
            tokio::spawn(async move {
                loop {
                    select! {
                        signal = signal.recv() => {
                            match signal {
                                SignalKind::Shutdown => {
                                    debug!("{signal}");
                                }
                                SignalKind::Terminate(_) => {
                                    warn!("{signal}")
                                }
                            }
                            break
                        }
                        Some(current) = rx.recv() => {
                            if current > previous_slot {
                                let mut slots_to_download = vec![];
                                if previous_slot != 0 {
                                    for slot in previous_slot.0 + 1..=current.0 {
                                        slots_to_download.push(Slot(slot));
                                    }
                                } else {
                                    slots_to_download.push(current);
                                }
                                previous_slot = current;
                                
                                debug!("slots to download {slots_to_download:#?}");
                                downloader.download_and_send_blocks(slots_to_download).await;
                            }
                        }
                    }
                }
            }),
        )
    }
}

pub struct DownloadAndSendBlock {
    rpc: RpcClient,
    tx: Sender<Block>,
    semaphore: Arc<Semaphore>,
    signal: Signal,
}

impl DownloadAndSendBlock {
    fn new(rpc: RpcClient, tx: Sender<Block>, parallel_downloads: usize, signal: Signal) -> Self {
        Self {
            rpc,
            tx,
            semaphore: Arc::new(Semaphore::new(parallel_downloads)),
            signal,
        }
    }
}

impl DownloadAndSendBlock {
    pub async fn download_and_send_blocks(&self, slots: Vec<Slot>) {
        let mut handles = Vec::new();
        let results = Arc::new(Mutex::new(BTreeMap::new()));

        for slot in slots {
            let rpc = self.rpc.clone();
            let semaphore = Arc::clone(&self.semaphore);
            let results = Arc::clone(&results);
            let signal = self.signal.clone();

            let handle = tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                debug!("Downloading block of slot: {}", slot);

                match rpc.block(slot).await {
                    Ok(block) => {
                        let mut res = results.lock().await;
                        res.insert(slot, block);
                    }
                    Err(err) => {
                        error!("Failed to fetch block for slot: {} - {}", slot, err);
                        signal.terminate("RpcBlockStream failed to fetch block");
                    }
                }
            });

            handles.push(handle);
        }

        join_all(handles).await;

        let mut res = results.lock().await;
        while let Some((_slot, block)) = res.pop_first() {
            if let Err(_) = self.tx.send(block).await {
                error!("Failed to send block to channel");
                self.signal
                    .terminate("RpcBlockStream failed to send to channel");
            }
        }
    }
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod download;
mod slot;

use base::model::solana::{Block, Slot};
use crate::rpc::RpcClient;
use crate::stream::block::download::{DownloadResult, Downloader};
use crate::stream::block::slot::SlotsToDownload;
use crate::stream::SlotStream;
use async_trait::async_trait;
use common::model::RpcUrl;
use common::Signal;
use log::error;
use std::collections::BTreeMap;
use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tokio::{select, try_join};

#[async_trait]
pub trait BlockStream: Send {
    async fn stream(self, signal: Signal) -> (Receiver<Block>, JoinHandle<()>);
}

pub struct RpcBlockStreamConfig {
    pub url: RpcUrl,
    pub concurrency: usize,
}

pub struct RpcBlockStream<S: SlotStream> {
    cfg: RpcBlockStreamConfig,
    slot_stream: S,
    tx: Sender<Block>,
    rx: Receiver<Block>,
    previous_slot: Option<Slot>,
}

impl<S: SlotStream> RpcBlockStream<S> {
    pub fn new(cfg: RpcBlockStreamConfig, slot_stream: S, previous_slot: Option<Slot>) -> Self {
        let (tx, rx) = channel(1_000);
        Self {
            cfg,
            slot_stream,
            tx,
            rx,
            previous_slot,
        }
    }
}

#[async_trait]
impl<S: SlotStream> BlockStream for RpcBlockStream<S> {
    async fn stream(self, signal: Signal) -> (Receiver<Block>, JoinHandle<()>) {
        let slots_to_download = SlotsToDownload::new(self.cfg.concurrency, self.previous_slot);

        let mut slot_signal = signal.clone();
        let updater = slots_to_download.clone();
        let slot_handle = tokio::spawn(async move {
            let (mut rx, _) = self.slot_stream.stream(slot_signal.clone()).await;
            loop {
                select! {
                    _ = slot_signal.recv() => { break }
                    Some(current) = rx.recv() => { updater.update(current).await }
                }
            }
        });

        let slots_to_download = slots_to_download.clone();

        let (download_tx, mut download_rx) = channel::<DownloadResult>(1);

        let download_signal = signal.clone();
        let download_block_handle = tokio::spawn(async move {
            let rpc_client = RpcClient::new(self.cfg.url);
            let downloader = Downloader::new(rpc_client, download_tx.clone(), self.cfg.concurrency);

            let mut download_signal = download_signal.clone();
            loop {
                if let Some(_) = download_signal.recv_maybe().await {
                    break;
                }

                let slots_to_download = slots_to_download.next_slots().await;

                let downloader = downloader.clone();
                tokio::spawn(async move {
                    downloader.download(slots_to_download).await;
                });

                sleep(Duration::from_millis(10)).await;
            }
        });

        let receive_block_handle = tokio::spawn(async move {
            let mut buffer = BTreeMap::new();
            let mut expected_slot = Slot(0);

            while let Some(result) = download_rx.recv().await {
                match result {
                    DownloadResult::Ok(slot, block) => {
                        if expected_slot == Slot(0) {
                            expected_slot = slot;
                        }
                        buffer.insert(slot, Some(block));
                    }
                    DownloadResult::Skip(slot) => {
                        if expected_slot == Slot(0) {
                            expected_slot = slot;
                        }
                        buffer.insert(slot, None);
                    }
                    DownloadResult::Error(slot, msg) => {
                        error!("failed to fetch block for slot: {} - {}", slot, msg);
                        signal.terminate("RpcBlockStream failed to fetch block");
                    }
                }

                while buffer.contains_key(&expected_slot) {
                    match buffer.remove(&expected_slot).unwrap() {
                        None => {}
                        Some(block) => {
                            if let Err(_) = self.tx.send(block).await {
                                error!("Failed to send block to channel");
                                signal.terminate("RpcBlockStream failed to send to channel");
                                return;
                            }
                        }
                    }
                    expected_slot = expected_slot.next();
                }
            }
        });

        (
            self.rx,
            tokio::spawn(async move {
                let _ = try_join!(slot_handle, download_block_handle, receive_block_handle);
            }),
        )
    }
}

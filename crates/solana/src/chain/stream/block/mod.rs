// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod download;
mod slot;

use crate::model::{Block, Slot};
use crate::rpc::RpcClient;
use crate::stream::block::download::download_blocks;
use crate::stream::block::slot::SlotsToDownload;
use crate::stream::SlotStream;
use async_trait::async_trait;
use common::model::RpcUrl;
use common::Signal;
use log::{debug, error};
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::JoinHandle;
use tokio::time::{sleep, Instant};
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
        let (tx, rx) = tokio::sync::mpsc::channel(1_000);
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
        let block_handle = tokio::spawn(async move {
            let rpc_client = RpcClient::new(self.cfg.url);
            let concurrency = self.cfg.concurrency;
            let mut signal = signal.clone();

            loop {
                if let Some(_) = signal.recv_maybe().await {
                    break;
                }

                let rpc_client = rpc_client.clone();

                let slots_to_download = slots_to_download.next_slots().await;

                if !slots_to_download.is_empty() {
                    let start = Instant::now();

                    let blocks =
                        download_blocks(rpc_client, slots_to_download, concurrency, signal.clone())
                            .await;
                    let number_of_blocks = blocks.len();

                    debug!(
                        "downloading {} blocks took {} ms",
                        number_of_blocks,
                        start.elapsed().as_millis()
                    );

                    let start = Instant::now();
                    for block in blocks {
                        if let Err(_) = self.tx.send(block).await {
                            error!("Failed to send block to channel");
                            signal.terminate("RpcBlockStream failed to send to channel");
                            return;
                        }
                    }
                    debug!( 
                        "sending {} blocks took {} ms",
                        number_of_blocks,
                        start.elapsed().as_millis()
                    );
                }
                sleep(Duration::from_millis(10)).await;
            }
        });

        (
            self.rx,
            tokio::spawn(async move {
                let _ = try_join!(slot_handle, block_handle);
            }),
        )
    }
}

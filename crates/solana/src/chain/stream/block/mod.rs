// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod download;

use crate::model::{Block, Slot};
use crate::rpc::RpcClient;
use crate::stream::block::download::download_blocks;
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
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        Self {
            // rpc_client: RpcClient::new(cfg.url),
            // concurrency: cfg.concurrency,
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
    async fn stream(self, mut signal: Signal) -> (Receiver<Block>, JoinHandle<()>) {
        let rpc_client = RpcClient::new(self.cfg.url);

        let mut previous_slot = self.previous_slot.unwrap_or(Slot(0));

        let (mut rx, _) = self.slot_stream.stream(signal.clone()).await;
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
                                        previous_slot = slot.into();
                                        if slots_to_download.len() >= 4{
                                            break
                                        }
                                    }
                                } else {
                                    slots_to_download.push(current);
                                    previous_slot = current;
                                }

                                debug!("slots to download {slots_to_download:#?}");
                                let blocks = download_blocks(rpc_client.clone(),slots_to_download, self.cfg.concurrency).await;

                                for block in blocks{
                                        if let Err(_) = self.tx.send(block).await {
                                            error!("Failed to send block to channel");
                                            signal.terminate("RpcBlockStream failed to send to channel");
                                        }
                                }
                            }
                        }
                    }
                }
            }),
        )
    }
}

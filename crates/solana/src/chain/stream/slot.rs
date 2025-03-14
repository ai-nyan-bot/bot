// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Slot;
use crate::rpc::RpcClient;
use crate::ws::WsClient;
use async_trait::async_trait;
use common::model::{RpcUrl, WsUrl};
use common::{Signal, SignalType};
use log::{debug, error, trace, warn};
use std::time::Duration;
use tokio::select;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::task::JoinHandle;
use tokio::time::sleep;

#[async_trait]
pub trait SlotStream: Send + Sync + 'static {
    async fn stream(self, signal: Signal) -> (Receiver<Slot>, JoinHandle<()>);
}

pub struct RpcSlotStream {
    url: RpcUrl,
    tx: Sender<Slot>,
    rx: Receiver<Slot>,
}

impl RpcSlotStream {
    pub fn new(url: impl Into<RpcUrl>) -> Self {
        let url = url.into();
        let (tx, rx) = channel(1);
        Self { url, tx, rx }
    }
}

impl Default for RpcSlotStream {
    fn default() -> Self {
        Self::new("http://api.mainnet-beta.solana.com")
    }
}

#[async_trait]
impl SlotStream for RpcSlotStream {
    async fn stream(self, mut signal: Signal) -> (Receiver<Slot>, JoinHandle<()>) {
        let tx = self.tx;
        let rpc = RpcClient::new(self.url);
        (
            self.rx,
            tokio::spawn(async move {
                let mut max = Slot::from(0);
                loop {
                    select! {
                        signal = signal.recv() => {
                            match signal{
                                SignalType::Shutdown => {
                                    debug!("{signal}");
                                }
                                SignalType::Terminate(_) => {
                                    warn!("{signal}")
                                }
                            }
                            break
                        }
                        _ = sleep(Duration::from_millis(400)) => {
                            match rpc.slot().await {
                                Ok(current) => {
                                    if current > max {
                                        max = current;
                                        trace!("{current}");
                                        match tx.send_timeout(current, Duration::from_millis(100)).await {
                                            Ok(_) => {}
                                            Err(_) => {
                                                warn!("downstream did not pick up message after 100ms")
                                            }
                                        }
                                    }
                                }
                                Err(err) => {
                                    error!("failed to retrieve slot: {err}")
                                }
                            }
                        }
                    }
                }
            }),
        )
    }
}

pub struct WsSlotStream {
    client: WsClient,
    tx: Sender<Slot>,
    rx: Receiver<Slot>,
}

impl WsSlotStream {
    pub async fn new(url: impl Into<WsUrl>) -> Self {
        let (tx, rx) = channel(1);
        Self {
            client: WsClient::new(url.into()).await.unwrap(),
            tx,
            rx,
        }
    }
}

#[async_trait]
impl SlotStream for WsSlotStream {
    async fn stream(self, mut signal: Signal) -> (Receiver<Slot>, JoinHandle<()>) {
        let tx = self.tx;
        (
            self.rx,
            tokio::spawn(async move {
                let (mut slot_info_rx, _) = self.client.subscribe_slot().await.unwrap();

                loop {
                    select! {
                        signal = signal.recv() => {
                            match signal{
                                SignalType::Shutdown => {
                                    debug!("{signal}");
                                }
                                SignalType::Terminate(_) => {
                                    warn!("{signal}")
                                }
                            }
                            break
                        }
                        Some(info) = slot_info_rx.recv() => {
                            tx.send(info.slot).await.unwrap();
                        }
                    }
                }
            }),
        )
    }
}

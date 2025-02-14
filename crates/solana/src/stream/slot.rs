// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Slot;
use crate::rpc::RpcClient;
use async_trait::async_trait;
use common::model::RpcUrl;
use common::{Signal, SignalKind};
use log::{debug, error, warn};
use std::time::Duration;
use tokio::select;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::task::JoinHandle;
use tokio::time::sleep;

#[async_trait]
pub trait SlotStream: Send {
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
                let max = Slot::from(0);
                loop {
                    select! {
                        signal = signal.recv() => {
                            match signal{
                            SignalKind::Shutdown => {
                                debug!("{signal}");
                            }
                            SignalKind::Terminate(_) => {
                                warn!("{signal}")
                            }
                            }
                            break
                        }
                        _ = sleep(Duration::from_millis(400)) => {
                            match rpc.slot().await {
                                Ok(current) => {
                                    // if current > max {
                                    //     max = current;
                                        debug!("{current}");
                                        // tx.send_async(current).await.unwrap();
                                        // tx.send(current).await.unwrap();
                                        match tx.send_timeout(current, Duration::from_millis(1)).await {
                                            Ok(_) => {}
                                            Err(err) => {
                                                warn!("downstream did not pick up message after 100ms")
                                            }
                                        }
                                    // }
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

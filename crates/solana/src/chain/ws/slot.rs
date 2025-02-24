// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::ws::response::SlotInfo;
use crate::ws::{WsClient, WsClientResult};
use futures_util::StreamExt;
use log::trace;
use tokio::sync::mpsc::{channel, Receiver};
use tokio::task::JoinHandle;

impl WsClient {
    pub async fn subscribe_slot(&self) -> WsClientResult<(Receiver<SlotInfo>, JoinHandle<()>)> {
        let (tx, rx) = channel(100);

        let client = self.client.clone();

        let handle = tokio::spawn(async move {
            let (mut stream, unsubscribe) = client.slot_subscribe().await.unwrap();
            while let Some(slot_info) = stream.next().await {
                trace!("received: {:?}", slot_info);
                let _ = tx.send(slot_info.into()).await;
            }
        });

        Ok((rx, handle))
    }
}

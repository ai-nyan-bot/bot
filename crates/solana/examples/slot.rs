// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use solana::ws::WsClient;

#[tokio::main]
async fn main() {
    let client = WsClient::new("wss://api.mainnet-beta.solana.com")
        .await
        .unwrap();

    let (mut rx, _) = client.subscribe_slot().await.unwrap();

    while let Some(slot) = rx.recv().await {
        println!("slot {:#?}", slot);
    }
}

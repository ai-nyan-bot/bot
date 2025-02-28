// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::Signal;
use solana::model::Slot;
use solana::stream::{
    BlockStream, RpcBlockStream, RpcBlockStreamConfig, RpcSlotStream, SlotStream,
};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "solana=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let signal = Signal::new();

    let (mut rx, handle) = RpcBlockStream::new(
        RpcBlockStreamConfig {
            url: "http://api.mainnet-beta.solana.com".into(),
            concurrency: 1,
        },
        RpcSlotStream::default(),
        Some(Slot(323544552)),
    )
    .stream(signal.clone())
    .await;

    while let Some(block) = rx.recv().await {
        println!("process {}", block.slot);
        // signal.shutdown();
    }
}

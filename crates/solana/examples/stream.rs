// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::Signal;
use solana::stream::{BlockStream, RpcBlockStream, RpcBlockStreamConfig, RpcSlotStream, SlotStream};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "web3=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // let rpc = RpcClient::new("http://api.mainnet-beta.solana.com".to_string());
    // let (mut blocks, unsubscriber) = client
    //     // .slot_updates_subscribe()
    //     // .slot_subscribe()
    //     .root_subscribe()
    //     // .logs_subscribe(
    //     //     RpcTransactionLogsFilter::Mentions(vec![pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P").to_string()]),
    //     //     RpcTransactionLogsConfig {
    //     //         commitment: Some(CommitmentConfig::confirmed()),
    //     //     },
    //     // )
    //     .await
    //     .unwrap();
    //
    // while let Some(response) = blocks.next().await {
    //     println!("{:?}", response);
    // }

    // let (mut blocks, unsubscriber) = client.slot_updates_subscribe().await.unwrap();

    // while let Some(response) = blocks.next().await {
    //     println!("{:?}", response);
    // }
    //
    // let (mut blocks, unsubscriber) = client
    //     .program_subscribe(
    //         &pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
    //         Some(RpcProgramAccountsConfig {
    //             filters: None,
    //             account_config: Default::default(),
    //             with_context: Some(true),
    //             sort_results: None,
    //         }),
    //     )
    //     // .slot_subscribe()
    //     // .block_subscribe(
    //     //     RpcBlockSubscribeFilter::All,
    //     //     Some(RpcBlockSubscribeConfig {
    //     //         encoding: Some(UiTransactionEncoding::Base58),
    //     //         transaction_details: Some(TransactionDetails::Full),
    //     //         commitment: Some(CommitmentConfig::confirmed()),
    //     //         max_supported_transaction_version: Some(0),
    //     //         show_rewards: None,
    //     //     }),
    //     // )
    //     .await
    //     .unwrap();

    // unsubscriber();

    // while let Some(response) = blocks.next().await {
    //     println!("{:?}", response);
    // }

    // let latest_slot = RwLock::new(Slot::from(0));
    // loop {
    //     let rpc = RpcClient::new("http://api.mainnet-beta.solana.com");
    //     let slot = rpc.slot().await.unwrap();
    //
    //     let mut lock = latest_slot.write().unwrap();
    //     if slot > *lock {
    //         *lock = slot;
    //     }
    //     drop(lock);
    //
    //     println!("current {} latest {}", slot, latest_slot.read().unwrap());
    //     sleep(Duration::from_millis(400)).await;
    //
    //     // tokio::spawn(async move {
    //     //     let block = rpc.full_block(slot).await;
    //     //     println!("downloaded {:#?}", block.slot);
    //     // });
    // }

    let signal = Signal::new();

    let (mut rx, handle) = RpcBlockStream::new(RpcBlockStreamConfig {
        url: "http://api.mainnet-beta.solana.com".into(),
        concurrency: 1,
    })
    .stream(RpcSlotStream::default(), signal.clone())
    .await;

    while let Some(block) = rx.recv().await {
        println!("process {}", block.slot);
        // signal.shutdown();
    }
    //
    //
    // let (rx, handle) = ws.subscribe_slot().await.unwrap();
    //
    // while let Ok(slot_info) = rx.recv_async().await {
    //     println!("info {:#?}", slot_info.slot);
    //
    //     let rpc = RpcClient::new("http://api.mainnet-beta.solana.com");
    //     println!("current {}", rpc.slot().await);
    //
    //     // tokio::spawn(async move {
    //     //     let block = rpc.full_block(slot_info.root).await;
    //     //     println!("downloaded {:#?}", block.slot);
    //     // });
    // }
    //
    // let _ = join!(async { handle.await });
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::{join, select};

use crate::config::Config;
use crate::solana::block::index_block;
use crate::solana::indexer::IndexerRepo;
use crate::solana::state::{State, StateInner};
use base::repo::{AddressRepo, TokenPairRepo, TokenRepo};
use common::repo::pool::setup_pool;
use common::{ResolveOr, Signal};
use solana::stream::{BlockStream, RpcBlockStream, RpcBlockStreamConfig, WsSlotStream};
use solana::token_info::rpc::TokenInfoRpcLoader;
use tokio::signal::unix::SignalKind;
use tracing::info;

pub mod block;
pub mod indexer;
mod jupiter;
mod pumpfun;
pub mod state;

pub fn index_solana(runtime: Runtime, config: Config) {
    runtime.block_on(async move {
        let signal = Signal::new();

        let pool = setup_pool(&config.postgres).await;

        // let mut tx = pool.begin().await.unwrap();
        let indexer_repo = IndexerRepo::default();
        // let indexer = indexer_repo.get(&mut tx).await.unwrap();

        // tx.commit().await.unwrap();

        let token_info_loader = TokenInfoRpcLoader::new(config.rpc.url.resolve());
        let token_repo = TokenRepo::new(Box::new(token_info_loader));
        let token_pair_repo = TokenPairRepo::new(token_repo.clone());

        let wallet_repo = AddressRepo::new();

        let pumpfun_swap_repo =
            solana::pumpfun::repo::SwapRepo::new(token_pair_repo.clone(), wallet_repo.clone());
        let jupiter_swap_repo =
            solana::jupiter::repo::SwapRepo::new(token_pair_repo.clone(), wallet_repo.clone());

        let state = State(Arc::new(StateInner {
            pool: pool.clone(),
            token_repo: token_repo.clone(),
            address_repo: AddressRepo::new(),
            pumpfun_swap_repo,
            pumpfun_curve_repo: solana::pumpfun::repo::CurveRepo::new(),
            jupiter_swap_repo,
        }));

        // let jupiter_parser = JupiterParser::new();
        // let pumpfun_parser = PumpFunParser::new();

        let sig = signal.clone();
        tokio::spawn(async move {
            let mut sigterm = tokio::signal::unix::signal(SignalKind::terminate()).unwrap();
            select! {
                _ = sigterm.recv() => {
                    info!("Received SIGTERM. Cleaning up resources...");
                    // exit.send(()).expect("Unable to initiate shutdown");
                    sig.shutdown();
                }
            }
        });

        let slot_stream = WsSlotStream::new(
            config
                .slotstream
                .url
                .resolve_or("wss://api.mainnet-beta.solana.com".to_string()),
        )
        .await;

        let mut tx = pool.begin().await.unwrap();
        let previous_slot = indexer_repo.get(&mut tx).await.map(|i| i.slot).ok();
        tx.commit().await.unwrap();

        let (mut blocks, block_stream_handle) = RpcBlockStream::new(
            RpcBlockStreamConfig {
                url: config
                    .blockstream
                    .url
                    .resolve_or("http://api.mainnet-beta.solana.com".to_string())
                    .into(),
                concurrency: config.blockstream.concurrency.resolve_or(1usize),
            },
            slot_stream,
            previous_slot,
        )
        .stream(signal.clone())
        .await;

        // let mut exit = exit_tx.subscribe();
        let mut signal = signal.clone();
        let handle = tokio::spawn(async move {
            loop {
                select! {
                     Some(block) = blocks.recv() => {
                        index_block(state.clone(),block).await;
                     },
                    _ = signal.recv() => {
                        break
                    }
                }
            }
        });

        //
        //
        let _ = join!(block_stream_handle, handle);

        // let (rx, handle) = RpcBlockStream::new(RpcBlockStreamConfig {
        //     url: config.rpc.url_1.resolve().into(),
        //     parallel_downloads: 1,
        // })
        // .stream(RpcSlotStream::default(), signal.clone())
        // .await;
        //
        // while let Ok(block) = rx.recv_async().await {}
    });
}

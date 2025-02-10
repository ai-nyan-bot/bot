// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::model::RpcUrl;
use std::process::exit;
use std::str::FromStr;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::{join, select};

use crate::config::Config;
use crate::solana::indexer::IndexerRepo;
use crate::solana::pumpfun::index_trade;
use crate::solana::state::{State, StateInner};
use base::model::PublicKey;
use base::repo::{AddressRepo, ReadTokenPairRepo, ReadTokenRepo, TokenPairRepo, TokenRepo};
use common::repo::pool::setup_pool;
use common::{model, ResolveOr, Signal};
use solana::model::TransactionStatus;
use solana::repo;
use solana::stream::{BlockStream, RpcBlockStream, RpcBlockStreamConfig, RpcSlotStream};
use solana::token_info::rpc::RpcTokenInfoLoader;
use solana::venue::jupiter::JupiterParser;
use solana::venue::pumpfun::PumpFunParser;
use solana::venue::Parser;
use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Acquire, ConnectOptions, PgPool};
use std::time::Duration;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::broadcast;
use tracing::log::LevelFilter;
use tracing::{debug, info};

pub mod indexer;
mod jupiter;
mod pumpfun;
mod state;

pub(crate) fn index_solana(runtime: Runtime, config: Config) {
    runtime.block_on(async move {
        let signal = Signal::new();

        let pool = setup_pool(&config.postgres).await;

        let mut tx = pool.begin().await.unwrap();
        let indexer_repo = IndexerRepo::default();
        let indexer = indexer_repo.get(&mut tx).await.unwrap();

        let _ = tx.commit().await.unwrap();

        let token_info_loader = RpcTokenInfoLoader::new(config.rpc.url.resolve());
        let read_token_repo = ReadTokenRepo::new();
        let token_repo = TokenRepo::new(token_info_loader, read_token_repo.clone());

        let read_token_pair_repo = ReadTokenPairRepo::new(read_token_repo);
        let token_pair_repo = TokenPairRepo::new(token_repo.clone(), read_token_pair_repo);

        let wallet_repo = AddressRepo::new();

        let pumpfun_trade_repo = repo::pumpfun::TradeRepo::new(token_pair_repo.clone(), wallet_repo.clone());
        let jupiter_trade_repo = repo::jupiter::TradeRepo::new(token_pair_repo.clone(), wallet_repo.clone());

        let state = State(Arc::new(StateInner {
            pool: pool.clone(),
            token_repo: token_repo.clone(),
            token_pair_repo: token_pair_repo,
            wallet_repo: wallet_repo,
            pumpfun_trade_repo,
            jupiter_trade_repo,
        }));

        let jupiter_parser = JupiterParser::new();
        let pumpfun_parser = PumpFunParser::new();

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

        let slot_stream = RpcSlotStream::new(config.slotstream.url.resolve_or("http://api.mainnet-beta.solana.com".to_string()));

        // FIXME ensure blocks arriving in chronological order
        let (mut blocks, block_stream_handle) = RpcBlockStream::new(RpcBlockStreamConfig {
            url: config.blockstream.url.resolve_or("http://api.mainnet-beta.solana.com".to_string()).into(),
            concurrency: config.blockstream.concurrency.resolve_or(1usize),
        })
        .stream(slot_stream, signal.clone())
        .await;

        let pumpfun_account = PublicKey::from_str("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P").unwrap();
        let jupiter_account = PublicKey::from_str("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4").unwrap();

        // let mut exit = exit_tx.subscribe();
        let mut signal = signal.clone();
        let handle = tokio::spawn(async move {
            loop {
                select! {
                     Some(block) = blocks.recv() => {
                        debug!("index {}", block.slot);

                        // FIXME it would be interesting to see what the time difference is between indexing a block and the actual block time

                        let mut jupiter_slot_trades = repo::jupiter::SlotTrades{
                            slot: block.slot.clone(),
                            timestamp: block.timestamp.clone(),
                            trades: vec![],
                        };

                        let mut pumpfun_slot_trades = repo::pumpfun::SlotTrades{
                            slot: block.slot.clone(),
                            timestamp: block.timestamp.clone(),
                            trades: vec![],
                        };

                        for transaction in block.transactions{
                            if transaction.status == TransactionStatus::Success {
                                if transaction.account_keys.contains(&pumpfun_account){
                                    for instruction in  pumpfun_parser.parse(&transaction).unwrap(){
                                        match instruction{
                                        solana::model::pumpfun::Instruction::Create{ .. } => {}
                                        solana::model::pumpfun::Instruction::Trade{
                                            mint,
                                            sol_amount,
                                            token_amount,
                                            is_buy,
                                            user,
                                            virtual_sol_reserves,
                                            virtual_token_reserves,
                                            ..
                                        } => {
                                                pumpfun_slot_trades.trades.push(repo::pumpfun::SlotTrade{
                                                        mint,
                                                        base_amount: token_amount,
                                                        quote_amount: sol_amount,
                                                        is_buy,
                                                        wallet: user,
                                                        virtual_base_reserves: virtual_token_reserves,
                                                        virtual_quote_reserves: virtual_sol_reserves,
                                                        signature: transaction.signature.clone()
                                                });

                                            }
                                        }
                                    }
                                }

                                if transaction.account_keys.contains(&jupiter_account){
                                      for instruction in  jupiter_parser.parse(&transaction).unwrap(){
                                        match instruction{
                                            solana::model::jupiter::Instruction::Trade{
                                            swaps,
                                            signer
                                        } => {
                                                let first = swaps.first().unwrap();
                                                let last = swaps.last().unwrap();
                                                jupiter_slot_trades.trades.push(repo::jupiter::SlotTrade{
                                                        input_mint: first.input_mint.clone(),
                                                        input_amount: first.input_amount.clone(),
                                                        output_mint: last.output_mint.clone(),
                                                        output_amount: last.output_amount.clone(),
                                                        wallet: signer,
                                                        signature: transaction.signature.clone()
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        let mut tx = pool.begin().await.unwrap();
                        let slot = block.slot.clone();

                        pumpfun::index_trade(&mut tx, state.clone(), pumpfun_slot_trades).await;
                        jupiter::index_trade(&mut tx, state.clone(), jupiter_slot_trades).await;

                        indexer_repo.set(&mut tx, slot).await.unwrap();
                        let _ = tx.commit().await.unwrap();
                     },
                    _ = signal.recv() => {
                        break
                    }
                }
            }
        });

        //
        //
        let _ = join!(async { block_stream_handle.await }, async { handle.await });

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

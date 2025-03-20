// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

#![cfg_attr(not(debug_assertions), deny(warnings))]

use crate::config::Config;
use common::repo::pool::setup_pool;
use futures::future::join_all;
use log::{error, info};
use tokio::runtime::Builder;
use tokio::task::JoinHandle;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

mod config;
mod jupiter;
mod pumpfun;
mod solana;
mod time;

fn main() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::load();

    let runtime = Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let pg_pool = setup_pool(&config.postgres).await;
        let pg_pool_summary = setup_pool(&config.postgres).await;

        let jupiter_refresh_candles = jupiter::RefreshCandles::new(pg_pool.clone());
        let jupiter_refresh_twaps = jupiter::RefreshTwaps::new(pg_pool.clone());

        let pumpfun_refresh_candles = pumpfun::RefreshCandles::new(pg_pool.clone());
        // let pumpfun_refresh_summaries = pumpfun::RefreshSummaries::new(pg_pool.clone());
        let pumpfun_refresh_summaries = pumpfun::RefreshSummaries::new(pg_pool_summary.clone());
        let pumpfun_refresh_twaps = pumpfun::RefreshTwaps::new(pg_pool.clone());

        let solana_refresh_sol = solana::RefreshSol::new(pg_pool.clone());

        let handles: Vec<JoinHandle<()>> = vec![
            solana_refresh_sol.refresh().await,
            jupiter_refresh_candles.refresh().await,
            jupiter_refresh_twaps.refresh().await,
            pumpfun_refresh_candles.refresh().await,
            pumpfun_refresh_twaps.refresh().await,
            pumpfun_refresh_summaries.refresh().await,
        ]
        .into_iter()
        .flatten()
        .collect();

        for result in join_all(handles).await {
            if let Err(e) = result {
                error!("Task failed: {:?}", e);
            }
        }

        info!("all tasks have been stopped, exiting...");
    });
}

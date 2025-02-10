// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::config::Config;
use crate::pumpfun::{RefreshCandles, RefreshSummary};
use common::repo::pool::setup_pool;
use common::ResolveOr;
use log::{info, LevelFilter};
use base::repo::{ReadTokenPairRepo, ReadTokenRepo};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool};
use std::time::Duration;
use tokio::runtime::Builder;
use tokio::try_join;
use tracing::error;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;

mod config;
mod pumpfun;

fn main() {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_span_events(FmtSpan::CLOSE)
        .init();

    println!("Starting the aggregator");

    let config = Config::load();

    let runtime = Builder::new_multi_thread().worker_threads(4).enable_all().build().unwrap();

    runtime.block_on(async {
        let pg_pool = setup_pool(&config.postgres).await;

        let token_pair_repo = ReadTokenPairRepo::new(ReadTokenRepo::new());

        let refresh_candles = RefreshCandles::new(pg_pool.clone(), token_pair_repo.clone());
        let refresh_summary = RefreshSummary::new(pg_pool.clone(), token_pair_repo.clone());

        let _ = try_join!(
            async { refresh_candles.s1().await },
            async { refresh_candles.m1().await },
            async { refresh_candles.m5().await },
            async { refresh_candles.m15().await },
            async { refresh_candles.h1().await },
            async { refresh_candles.h4().await },
            async { refresh_candles.d1().await },
            async { refresh_summary.m1().await },
            async { refresh_summary.m5().await },
            async { refresh_summary.m15().await },
            async { refresh_summary.h1().await },
            async { refresh_summary.h4().await },
            async { refresh_summary.d1().await }
        );

        error!("All tasks have stopped, exiting...");
    });
}

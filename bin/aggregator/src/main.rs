// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

#![cfg_attr(not(debug_assertions), deny(warnings))]

use crate::config::Config;
use common::repo::pool::setup_pool;
use tokio::runtime::Builder;
use tokio::try_join;
use tracing::error;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;

mod config;
mod jupiter;
mod pumpfun;

fn main() {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let config = Config::load();

    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let pg_pool = setup_pool(&config.postgres).await;

        let jupiter_refresh_candles = jupiter::RefreshCandles::new(pg_pool.clone());
        let jupiter_refresh_sol = jupiter::RefreshSol::new(pg_pool.clone());
        let jupiter_refresh_twaps = jupiter::RefreshTwaps::new(pg_pool.clone());

        let pumpfun_refresh_candles = pumpfun::RefreshCandles::new(pg_pool.clone());

        let _ = try_join!(
            // jupiter candle
            async { jupiter_refresh_candles.s1().await },
            async { jupiter_refresh_candles.m1().await },
            async { jupiter_refresh_candles.m5().await },
            async { jupiter_refresh_candles.m15().await },
            async { jupiter_refresh_candles.h1().await },
            async { jupiter_refresh_candles.h6().await },
            async { jupiter_refresh_candles.d1().await },
            // jupiter sol
            async { jupiter_refresh_sol.m1().await },
            async { jupiter_refresh_sol.m5().await },
            async { jupiter_refresh_sol.m15().await },
            async { jupiter_refresh_sol.h1().await },
            async { jupiter_refresh_sol.h6().await },
            async { jupiter_refresh_sol.d1().await },
            // jupiter twap
            async { jupiter_refresh_twaps.m1().await },
            async { jupiter_refresh_twaps.m5().await },
            async { jupiter_refresh_twaps.m15().await },
            async { jupiter_refresh_twaps.h1().await },
            async { jupiter_refresh_twaps.h6().await },
            async { jupiter_refresh_twaps.d1().await },
            // pumpfun candle
            async { pumpfun_refresh_candles.s1().await },
            async { pumpfun_refresh_candles.m1().await },
            async { pumpfun_refresh_candles.m5().await },
            async { pumpfun_refresh_candles.m15().await },
            async { pumpfun_refresh_candles.h1().await },
            async { pumpfun_refresh_candles.h6().await },
            async { pumpfun_refresh_candles.d1().await },
            // async { refresh_summary.m1().await },
            // async { refresh_summary.m5().await },
            // async { refresh_summary.m15().await },
            // async { refresh_summary.h1().await },
            // async { refresh_summary.h4().await },
            // async { refresh_summary.d1().await }
        );

        error!("All tasks have stopped, exiting...");
    });
}

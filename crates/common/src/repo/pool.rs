// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::{ConfigValue, ResolveOr};
use log::{info, LevelFilter};
use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool};
use std::time::Duration;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct PostgresConfig {
    pub connection_string: ConfigValue,
    pub pool_min: ConfigValue,
    pub pool_max: ConfigValue,
    pub timeout_acquire_ms: ConfigValue,
}

pub async fn setup_pool(postgres_config: &PostgresConfig) -> PgPool {
    let pg_pool_options = PgPoolOptions::new()
        .acquire_slow_threshold(Duration::from_secs(5))
        .min_connections(postgres_config.pool_min.resolve_or(1) as u32)
        .max_connections(postgres_config.pool_max.resolve_or(8) as u32)
        .test_before_acquire(true)
        .before_acquire(|conn, _meta| {
            Box::pin(async move {
                sqlx::query("SET work_mem TO '256MB'")
                    .execute(&mut *conn)
                    .await?;
                Ok(true)
            })
        })
        .after_release(|conn, _meta| {
            Box::pin(async move {
                sqlx::query("SET work_mem TO '20MB'")
                    .execute(&mut *conn)
                    .await?;
                Ok(true)
            })
        })
        .acquire_timeout(Duration::from_millis(
            postgres_config.timeout_acquire_ms.resolve_or(5_000) as u64,
        ));

    let connection_string = &postgres_config.connection_string.resolve();

    let mut pg_options: PgConnectOptions = connection_string.parse().unwrap();
    pg_options = pg_options.log_statements(LevelFilter::Debug);
    pg_options = PgConnectOptions::log_slow_statements(
        pg_options,
        LevelFilter::Warn,
        Duration::from_secs(1),
    );

    info!(
        "Connecting to Postgres: {}:{}",
        pg_options.get_host(),
        pg_options.get_port()
    );

    pg_pool_options.connect_with(pg_options).await.unwrap()
}

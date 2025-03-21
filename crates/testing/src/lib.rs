// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

#![cfg_attr(not(debug_assertions), deny(warnings))]

use crate::user::{get_or_create_another_user, get_or_create_test_user};
use ::base::model::Mint;
use common::repo::Tx;
use futures_util::FutureExt;
pub use hash::hash_for_testing;
use log::info;
use rand::{rng, Rng};
use rule::create_inactive_rule_for_another_user;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Executor, PgPool};
use std::future::Future;
use std::panic;
use token_pair::get_or_create_token_pair;
use tokio::sync::OnceCell;

pub mod address;
pub mod auth;
mod hash;
pub mod invocation;
pub mod jupiter;
pub mod notification;
pub mod pumpfun;
pub mod rule;
pub mod token;
pub mod token_pair;
pub mod user;
pub mod wallet;

fn generate_db_name() -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    let mut rng = rng();
    let name: String = (0..32)
        .map(|_| CHARSET[rng.random_range(0..CHARSET.len())] as char)
        .collect();
    format!("test_{}", name)
}

fn generate_snapshot_db_name() -> String {
    let mut rng = rng();
    let charset: Vec<char> = ('a'..='z').collect();
    format!(
        "snapshot_{}",
        (0..32)
            .map(|_| charset[rng.random_range(0..charset.len())])
            .collect::<String>()
    )
}

static SNAPSHOT: OnceCell<String> = OnceCell::const_new();

async fn with_snapshot() -> &'static str {
    SNAPSHOT
        .get_or_init(|| async {
            let admin_pool = PgPool::connect_with(
                PgConnectOptions::new()
                    .host("localhost")
                    .port(2345)
                    .username("user")
                    .password("pass")
                    .database("postgres"),
            )
            .await
            .unwrap();

            let snapshot_db_name = generate_snapshot_db_name();

            admin_pool
                .execute(format!("create database {}", snapshot_db_name).as_str())
                .await
                .unwrap();
            info!("Created snapshot database: {}", snapshot_db_name);

            let snapshot_pool = PgPoolOptions::new()
                .connect_with(
                    PgConnectOptions::new()
                        .host("localhost")
                        .port(2345)
                        .username("user")
                        .password("pass")
                        .database(snapshot_db_name.as_str()),
                )
                .await
                .unwrap();

            sqlx::migrate!("../../migrations")
                .run(&snapshot_pool)
                .await
                .unwrap();

            drop(snapshot_pool);

            snapshot_db_name
        })
        .await
}

pub async fn get_test_pool() -> PgPool {
    let admin_pool = PgPool::connect_with(
        PgConnectOptions::new()
            .host("localhost")
            .port(2345)
            .username("user")
            .password("pass")
            .database("postgres"),
    )
    .await
    .unwrap();

    let snapshot = with_snapshot().await;
    let test_db = generate_db_name();

    admin_pool
        .execute(format!("create database {test_db} template {snapshot};").as_str())
        .await
        .unwrap();

    info!("Created test database: {}", test_db);

    let pool = PgPoolOptions::new()
        .connect_with(
            PgConnectOptions::new()
                .host("localhost")
                .port(2345)
                .username("user")
                .password("pass")
                .database(test_db.as_str()),
        )
        .await
        .unwrap();

    pool
}

pub async fn run_test<'a, T, TFut>(test: T)
where
    T: FnOnce(Tx<'a>) -> TFut + Send + 'static,
    TFut: Future + Send,
{
    let result = panic::AssertUnwindSafe(async move {
        let pool = get_test_pool().await;
        let mut tx = pool.begin().await.unwrap();
        initialise_database(&mut tx).await;
        let _ = tx.commit().await;

        let tx = pool.begin().await.unwrap();
        test(tx).await;
    })
    .catch_unwind()
    .await
    .err();

    // propagate error if test or preparation failed
    if let Some(err) = result {
        panic::resume_unwind(err)
    }
}

pub async fn initialise_database<'a>(tx: &mut Tx<'a>) {
    get_or_create_test_user(tx).await;
    get_or_create_another_user(tx).await;

    create_inactive_rule_for_another_user(tx, "Rule A").await;
    create_inactive_rule_for_another_user(tx, "Rule B").await;
    create_inactive_rule_for_another_user(tx, "Rule C").await;

    get_or_create_token_pair(tx, Mint::usdc(), Mint::usdt()).await;
}

pub async fn run_test_on_empty_db<'a, T, TFut>(test: T)
where
    T: FnOnce(Tx<'a>) -> TFut + Send + 'static,
    TFut: Future + Send,
{
    let result = panic::AssertUnwindSafe(async move {
        let pool = get_test_pool().await;
        let tx = pool.begin().await.unwrap();
        test(tx).await;
    })
    .catch_unwind()
    .await
    .err();

    // propagate error if test or preparation failed
    if let Some(err) = result {
        panic::resume_unwind(err)
    }
}

pub async fn run_test_with_pool_on_empty_db<T, TFut>(test: T)
where
    T: FnOnce(PgPool) -> TFut + 'static,
    TFut: Future,
{
    let result = panic::AssertUnwindSafe(async move {
        let pool = get_test_pool().await;
        test(pool.clone()).await;
    })
    .catch_unwind()
    .await
    .err();

    // propagate error if test or preparation failed
    if let Some(err) = result {
        panic::resume_unwind(err)
    }
}

pub async fn run_test_with_pool<T, TFut>(test: T)
where
    T: FnOnce(PgPool) -> TFut + 'static,
    TFut: Future,
{
    let result = panic::AssertUnwindSafe(async move {
        let pool = get_test_pool().await;

        let mut tx = pool.begin().await.unwrap();
        initialise_database(&mut tx).await;
        let _ = tx.commit().await;

        test(pool.clone()).await;
    })
    .catch_unwind()
    .await
    .err();

    // propagate error if test or preparation failed
    if let Some(err) = result {
        panic::resume_unwind(err)
    }
}

#[macro_export]
macro_rules! assert_sql {
    ($tx:expr, $query:expr) => {{
        use sqlx::Executor;
        let formatted_query = format!(r#"do $$ begin assert {}; end $$;"#, $query);
        let result = sqlx::query(&formatted_query).execute(&mut **$tx).await;
        assert!(result.is_ok(), "{}", formatted_query);
    }};
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::user::{get_or_create_another_user, get_or_create_test_user};
use ::base::model::TokenMint;
use common::repo::Tx;
use futures_util::FutureExt;
use log::info;
use rand::{thread_rng, Rng};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Executor, PgPool};
use std::future::Future;
use std::panic;
use rule::create_rule_for_another_user;
use token_pair::get_or_create_token_pair;

pub mod address;
pub mod auth;
pub mod invocation;
pub mod notification;
pub mod rule;
pub mod token;
pub mod token_pair;
pub mod user;
pub mod wallet;

fn generate_db_name() -> String {
    let mut rng = thread_rng();
    let charset: Vec<char> = ('a'..='z').collect();
    format!("test_{}", (0..32).map(|_| charset[rng.gen_range(0..charset.len())]).collect::<String>())
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

    let db_name = generate_db_name();

    admin_pool.execute(format!("create database {}", db_name).as_str()).await.unwrap();
    info!("Created test database: {}", db_name);

    drop(admin_pool);

    let pool = PgPoolOptions::new()
        .connect_with(
            PgConnectOptions::new()
                .host("localhost")
                .port(2345)
                .username("user")
                .password("pass")
                .database(db_name.as_str()),
        )
        .await
        .unwrap();

    sqlx::migrate!("../../migrations").run(&pool).await.unwrap();

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

        get_or_create_test_user(&mut tx).await;
        get_or_create_another_user(&mut tx).await;

        create_rule_for_another_user(&mut tx, "Rule A").await;
        create_rule_for_another_user(&mut tx, "Rule B").await;
        create_rule_for_another_user(&mut tx, "Rule C").await;

        get_or_create_token_pair(&mut tx, TokenMint::usdc(), TokenMint::usdt()).await;

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

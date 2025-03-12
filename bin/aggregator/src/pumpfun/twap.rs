// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::model::Partition;
use solana::pumpfun::repo::TwapRepo;
use sqlx::PgPool;
use std::time::Duration;
use tokio::task::JoinHandle;

pub struct RefreshTwaps {
    pool: PgPool,
    repo: TwapRepo,
}

impl RefreshTwaps {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            repo: TwapRepo::new(),
        }
    }

    pub async fn refresh(&self) -> Vec<JoinHandle<()>> {
        let mut result = Vec::new();
        let repo = self.repo.clone();
        let pool = self.pool.clone();
        for partition in Partition::enumerate() {
            let repo = repo.clone();
            let pool = pool.clone();
            result.push(tokio::spawn(async move {
                loop {
                    let mut tx = pool.begin().await.unwrap();

                    repo.calculate_1m(&mut tx, partition).await.unwrap();
                    repo.calculate_5m(&mut tx, partition).await.unwrap();
                    repo.calculate_15m(&mut tx, partition).await.unwrap();
                    repo.calculate_1h(&mut tx, partition).await.unwrap();
                    repo.calculate_6h(&mut tx, partition).await.unwrap();
                    repo.calculate_1d(&mut tx, partition).await.unwrap();

                    let _ = tx.commit().await;
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }))
        }
        result
    }
}

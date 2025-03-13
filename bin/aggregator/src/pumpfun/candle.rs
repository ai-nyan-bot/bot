// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::model::Partition;
use solana::pumpfun::repo::CandleRepo;
use sqlx::PgPool;
use std::time::Duration;
use tokio::task::JoinHandle;

pub struct RefreshCandles {
    pool: PgPool,
    repo: CandleRepo,
}

impl RefreshCandles {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            repo: CandleRepo::new(),
        }
    }

    pub async fn refresh(&self) -> Vec<JoinHandle<()>> {
        let mut result = Vec::new();
        let repo = self.repo.clone();
        let pool = self.pool.clone();
        for partition in Partition::enumerate() {
            let repo_1s = repo.clone();
            let pool_1s = pool.clone();
            result.push(tokio::spawn(async move {
                loop {
                    let mut tx = pool_1s.begin().await.unwrap();
                    repo_1s.calculate_1s(&mut tx, partition).await.unwrap();
                    repo_1s
                        .calculate_progress_1s(&mut tx, partition)
                        .await
                        .unwrap();
                        
                    let _ = tx.commit().await;
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            }));

            let repo = repo.clone();
            let pool = pool.clone();
            result.push(tokio::spawn(async move {
                loop {
                    let mut tx = pool.begin().await.unwrap();

                    repo.calculate_1m(&mut tx, partition).await.unwrap();
                    repo.calculate_progress_1m(&mut tx, partition)
                        .await
                        .unwrap();
                    repo.calculate_mcap_1m(&mut tx, partition).await.unwrap();
                    repo.calculate_usd_1m(&mut tx, partition).await.unwrap();

                    repo.calculate_5m(&mut tx, partition).await.unwrap();
                    repo.calculate_progress_5m(&mut tx, partition)
                        .await
                        .unwrap();
                    repo.calculate_mcap_5m(&mut tx, partition).await.unwrap();
                    repo.calculate_usd_5m(&mut tx, partition).await.unwrap();

                    repo.calculate_15m(&mut tx, partition).await.unwrap();
                    repo.calculate_progress_15m(&mut tx, partition)
                        .await
                        .unwrap();
                    repo.calculate_mcap_15m(&mut tx, partition).await.unwrap();
                    repo.calculate_usd_15m(&mut tx, partition).await.unwrap();

                    repo.calculate_1h(&mut tx, partition).await.unwrap();
                    repo.calculate_progress_1h(&mut tx, partition)
                        .await
                        .unwrap();
                    repo.calculate_mcap_1h(&mut tx, partition).await.unwrap();
                    repo.calculate_usd_1h(&mut tx, partition).await.unwrap();

                    repo.calculate_6h(&mut tx, partition).await.unwrap();
                    repo.calculate_progress_6h(&mut tx, partition)
                        .await
                        .unwrap();
                    repo.calculate_mcap_6h(&mut tx, partition).await.unwrap();
                    repo.calculate_usd_6h(&mut tx, partition).await.unwrap();

                    repo.calculate_1d(&mut tx, partition).await.unwrap();
                    repo.calculate_progress_1d(&mut tx, partition)
                        .await
                        .unwrap();
                    repo.calculate_mcap_1d(&mut tx, partition).await.unwrap();
                    repo.calculate_usd_1d(&mut tx, partition).await.unwrap();

                    let _ = tx.commit().await;
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }))
        }
        result
    }
}

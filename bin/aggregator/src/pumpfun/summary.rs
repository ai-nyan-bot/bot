// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::model::Partition;
use solana::pumpfun::repo::SummaryRepo;
use sqlx::PgPool;
use std::time::Duration;
use tokio::task::JoinHandle;

pub struct RefreshSummaries {
    pool: PgPool,
    repo: SummaryRepo,
}

impl RefreshSummaries {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            repo: SummaryRepo::new(),
        }
    }

    pub fn m1(&self) -> JoinHandle<()> {
        let repo = self.repo.clone();
        let pool = self.pool.clone();
        tokio::spawn(async move {
            loop {
                for partition in Partition::enumerate() {
                    let repo = repo.clone();
                    let pool = pool.clone();
                    tokio::spawn(async move {
                        let mut tx = pool.begin().await.unwrap();

                        repo.calculate_1m(&mut tx, partition).await.unwrap();

                        let _ = tx.commit().await;
                        tokio::time::sleep(Duration::from_millis(400)).await;
                    })
                    .await
                    .unwrap();
                }
            }
        })
    }

    pub fn m5(&self) -> JoinHandle<()> {
        let repo = self.repo.clone();
        let pool = self.pool.clone();
        tokio::spawn(async move {
            loop {
                for partition in Partition::enumerate() {
                    let repo = repo.clone();
                    let pool = pool.clone();
                    tokio::spawn(async move {
                        let mut tx = pool.begin().await.unwrap();

                        repo.calculate_5m(&mut tx, partition).await.unwrap();

                        let _ = tx.commit().await;
                        tokio::time::sleep(Duration::from_millis(400)).await;
                    })
                    .await
                    .unwrap();
                }
            }
        })
    }

    pub fn m15(&self) -> JoinHandle<()> {
        let repo = self.repo.clone();
        let pool = self.pool.clone();
        tokio::spawn(async move {
            loop {
                for partition in Partition::enumerate() {
                    let repo = repo.clone();
                    let pool = pool.clone();
                    tokio::spawn(async move {
                        let mut tx = pool.begin().await.unwrap();

                        repo.calculate_15m(&mut tx, partition).await.unwrap();

                        let _ = tx.commit().await;
                        tokio::time::sleep(Duration::from_millis(400)).await;
                    })
                    .await
                    .unwrap();
                }
            }
        })
    }

    pub fn h1(&self) -> JoinHandle<()> {
        let repo = self.repo.clone();
        let pool = self.pool.clone();
        tokio::spawn(async move {
            loop {
                for partition in Partition::enumerate() {
                    let repo = repo.clone();
                    let pool = pool.clone();
                    tokio::spawn(async move {
                        let mut tx = pool.begin().await.unwrap();

                        repo.calculate_1h(&mut tx, partition).await.unwrap();

                        let _ = tx.commit().await;
                        tokio::time::sleep(Duration::from_millis(400)).await;
                    })
                    .await
                    .unwrap();
                }
            }
        })
    }

    pub fn h6(&self) -> JoinHandle<()> {
        let repo = self.repo.clone();
        let pool = self.pool.clone();
        tokio::spawn(async move {
            loop {
                for partition in Partition::enumerate() {
                    let repo = repo.clone();
                    let pool = pool.clone();
                    tokio::spawn(async move {
                        let mut tx = pool.begin().await.unwrap();

                        repo.calculate_6h(&mut tx, partition).await.unwrap();

                        let _ = tx.commit().await;
                        tokio::time::sleep(Duration::from_millis(400)).await;
                    })
                    .await
                    .unwrap();
                }
            }
        })
    }

    pub fn d1(&self) -> JoinHandle<()> {
        let repo = self.repo.clone();
        let pool = self.pool.clone();
        tokio::spawn(async move {
            loop {
                for partition in Partition::enumerate() {
                    let repo = repo.clone();
                    let pool = pool.clone();
                    tokio::spawn(async move {
                        let mut tx = pool.begin().await.unwrap();

                        repo.calculate_1d(&mut tx, partition).await.unwrap();

                        let _ = tx.commit().await;
                        tokio::time::sleep(Duration::from_millis(400)).await;
                    })
                    .await
                    .unwrap();
                }
            }
        })
    }
}

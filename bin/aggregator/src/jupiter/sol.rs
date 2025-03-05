// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use solana::jupiter::repo::SolRepo;
use sqlx::PgPool;
use std::time::Duration;
use tokio::task::JoinHandle;

pub struct RefreshSol {
    pool: PgPool,
    repo: SolRepo,
}

impl RefreshSol {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            repo: SolRepo::new(),
        }
    }

    pub fn m1(&self) -> JoinHandle<()> {
        let repo = self.repo.clone();
        let pool = self.pool.clone();
        tokio::spawn(async move {
            loop {
                let repo = repo.clone();
                let pool = pool.clone();
                tokio::spawn(async move {
                    let mut tx = pool.begin().await.unwrap();

                    repo.calculate_1m(&mut tx).await.unwrap();

                    let _ = tx.commit().await;
                    tokio::time::sleep(Duration::from_millis(10)).await;
                })
                .await
                .unwrap();
            }
        })
    }

    pub fn m5(&self) -> JoinHandle<()> {
        let repo = self.repo.clone();
        let pool = self.pool.clone();
        tokio::spawn(async move {
            loop {
                let repo = repo.clone();
                let pool = pool.clone();
                tokio::spawn(async move {
                    let mut tx = pool.begin().await.unwrap();

                    repo.calculate_5m(&mut tx).await.unwrap();

                    let _ = tx.commit().await;
                    tokio::time::sleep(Duration::from_millis(10)).await;
                })
                .await
                .unwrap();
            }
        })
    }

    pub fn m15(&self) -> JoinHandle<()> {
        let repo = self.repo.clone();
        let pool = self.pool.clone();
        tokio::spawn(async move {
            loop {
                let repo = repo.clone();
                let pool = pool.clone();
                tokio::spawn(async move {
                    let mut tx = pool.begin().await.unwrap();

                    repo.calculate_15m(&mut tx).await.unwrap();

                    let _ = tx.commit().await;
                    tokio::time::sleep(Duration::from_millis(10)).await;
                })
                .await
                .unwrap();
            }
        })
    }

    pub fn h1(&self) -> JoinHandle<()> {
        let repo = self.repo.clone();
        let pool = self.pool.clone();
        tokio::spawn(async move {
            loop {
                let repo = repo.clone();
                let pool = pool.clone();
                tokio::spawn(async move {
                    let mut tx = pool.begin().await.unwrap();

                    repo.calculate_1h(&mut tx).await.unwrap();

                    let _ = tx.commit().await;
                    tokio::time::sleep(Duration::from_millis(10)).await;
                })
                .await
                .unwrap();
            }
        })
    }

    pub fn h6(&self) -> JoinHandle<()> {
        let repo = self.repo.clone();
        let pool = self.pool.clone();
        tokio::spawn(async move {
            loop {
                let repo = repo.clone();
                let pool = pool.clone();
                tokio::spawn(async move {
                    let mut tx = pool.begin().await.unwrap();

                    repo.calculate_6h(&mut tx).await.unwrap();

                    let _ = tx.commit().await;
                    tokio::time::sleep(Duration::from_millis(10)).await;
                })
                .await
                .unwrap();
            }
        })
    }

    pub fn d1(&self) -> JoinHandle<()> {
        let repo = self.repo.clone();
        let pool = self.pool.clone();
        tokio::spawn(async move {
            loop {
                let repo = repo.clone();
                let pool = pool.clone();
                tokio::spawn(async move {
                    let mut tx = pool.begin().await.unwrap();

                    repo.calculate_1d(&mut tx).await.unwrap();

                    let _ = tx.commit().await;
                    tokio::time::sleep(Duration::from_millis(10)).await;
                })
                .await
                .unwrap();
            }
        })
    }
}

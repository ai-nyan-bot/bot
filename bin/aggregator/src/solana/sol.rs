// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use solana::repo::SolRepo;
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

    pub async fn refresh(&self) -> Vec<JoinHandle<()>> {
        let mut result = Vec::new();
        let repo = self.repo.clone();
        let pool = self.pool.clone();
        result.push(tokio::spawn(async move {
            loop {
                let mut tx = pool.begin().await.unwrap();

                repo.calculate_1m(&mut tx).await.unwrap();
                repo.calculate_5m(&mut tx).await.unwrap();
                repo.calculate_15m(&mut tx).await.unwrap();
                repo.calculate_1h(&mut tx).await.unwrap();
                repo.calculate_6h(&mut tx).await.unwrap();
                repo.calculate_1d(&mut tx).await.unwrap();

                let _ = tx.commit().await;
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }));
        result
    }
}

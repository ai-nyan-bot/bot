// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::log_ms;
use crate::time::sleep_ms;
use log::trace;
use solana::repo::SolRepo;
use sqlx::PgPool;
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
                if let Ok(Some(mut tx)) = pool.try_begin().await {
                    log_ms!("calculate_1m", async {
                        repo.calculate_1m(&mut tx).await.unwrap()
                    });

                    log_ms!("calculate_5m", async {
                        repo.calculate_5m(&mut tx).await.unwrap()
                    });

                    log_ms!("calculate_15m", async {
                        repo.calculate_15m(&mut tx).await.unwrap()
                    });

                    log_ms!("calculate_1h", async {
                        repo.calculate_1h(&mut tx).await.unwrap()
                    });

                    log_ms!("calculate_6h", async {
                        repo.calculate_6h(&mut tx).await.unwrap()
                    });

                    log_ms!("calculate_1d", async {
                        repo.calculate_1d(&mut tx).await.unwrap()
                    });

                    let _ = tx.commit().await;
                    sleep_ms(500, 1000).await;
                } else {
                    trace!("Failed to acquire transaction");
                    sleep_ms(1, 100).await;
                }
            }
        }));
        result
    }
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::log_ms;
use crate::time::sleep_ms;
use common::model::Partition;
use log::trace;
use solana::jupiter::repo::TwapRepo;
use sqlx::PgPool;
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
                    if let Ok(Some(mut tx)) = pool.try_begin().await {
                        log_ms!("calculate_1m", partition, async {
                            repo.calculate_1m(&mut tx, partition).await.unwrap()
                        });

                        log_ms!("calculate_5m", partition, async {
                            repo.calculate_5m(&mut tx, partition).await.unwrap()
                        });

                        log_ms!("calculate_15m", partition, async {
                            repo.calculate_15m(&mut tx, partition).await.unwrap()
                        });

                        log_ms!("calculate_1h", partition, async {
                            repo.calculate_1h(&mut tx, partition).await.unwrap()
                        });

                        log_ms!("calculate_6h", partition, async {
                            repo.calculate_6h(&mut tx, partition).await.unwrap()
                        });

                        log_ms!("calculate_1d", partition, async {
                            repo.calculate_1d(&mut tx, partition).await.unwrap()
                        });

                        let _ = tx.commit().await;
                        sleep_ms(500, 1000).await;
                    } else {
                        trace!("Failed to acquire transaction");
                        sleep_ms(1, 100).await;
                    }
                }
            }))
        }
        result
    }
}

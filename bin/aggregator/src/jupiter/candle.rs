// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::log_ms;
use crate::time::sleep_ms;
use common::model::Partition;
use log::trace;
use solana::jupiter::repo::CandleRepo;
use sqlx::PgPool;
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
                    if let Ok(Some(mut tx)) = pool_1s.try_begin().await {
                        log_ms!("calculate_1s", partition, async {
                            repo_1s.calculate_1s(&mut tx, partition).await.unwrap();
                        });

                        let _ = tx.commit().await;
                        sleep_ms(100, 500).await;
                    } else {
                        trace!("Failed to acquire transaction");
                        sleep_ms(1, 100).await;
                    }
                }
            }));

            let repo_candle = repo.clone();
            let pool_candle = pool.clone();
            result.push(tokio::spawn(async move {
                loop {
                    if let Ok(Some(mut tx)) = pool_candle.try_begin().await {
                        log_ms!("calculate_1m", partition, async {
                            repo_candle.calculate_1m(&mut tx, partition).await.unwrap();
                        });

                        log_ms!("calculate_5m", partition, async {
                            repo_candle.calculate_5m(&mut tx, partition).await.unwrap();
                        });

                        log_ms!("calculate_15m", partition, async {
                            repo_candle.calculate_15m(&mut tx, partition).await.unwrap();
                        });

                        log_ms!("calculate_1h", partition, async {
                            repo_candle.calculate_1h(&mut tx, partition).await.unwrap();
                        });

                        log_ms!("calculate_6h", partition, async {
                            repo_candle.calculate_6h(&mut tx, partition).await.unwrap();
                        });

                        log_ms!("calculate_1d", partition, async {
                            repo_candle.calculate_1d(&mut tx, partition).await.unwrap();
                        });

                        let _ = tx.commit().await;
                        sleep_ms(500, 1000).await;
                    } else {
                        trace!("Failed to acquire transaction");
                        sleep_ms(1, 100).await;
                    }
                }
            }));

            let repo_mcap = repo.clone();
            let pool_mcap = pool.clone();
            result.push(tokio::spawn(async move {
                loop {
                    if let Ok(Some(mut tx)) = pool_mcap.try_begin().await {
                        log_ms!("calculate_mcap_1m", partition, async {
                            repo_mcap
                                .calculate_mcap_1m(&mut tx, partition)
                                .await
                                .unwrap();
                        });

                        log_ms!("calculate_mcap_5m", partition, async {
                            repo_mcap
                                .calculate_mcap_5m(&mut tx, partition)
                                .await
                                .unwrap();
                        });

                        log_ms!("calculate_mcap_15m", partition, async {
                            repo_mcap
                                .calculate_mcap_15m(&mut tx, partition)
                                .await
                                .unwrap();
                        });

                        log_ms!("calculate_mcap_1h", partition, async {
                            repo_mcap
                                .calculate_mcap_1h(&mut tx, partition)
                                .await
                                .unwrap();
                        });

                        log_ms!("calculate_mcap_6h", partition, async {
                            repo_mcap
                                .calculate_mcap_6h(&mut tx, partition)
                                .await
                                .unwrap();
                        });

                        log_ms!("calculate_mcap_1d", partition, async {
                            repo_mcap
                                .calculate_mcap_1d(&mut tx, partition)
                                .await
                                .unwrap();
                        });

                        let _ = tx.commit().await;
                        sleep_ms(500, 1000).await;
                    } else {
                        trace!("Failed to acquire transaction");
                        sleep_ms(1, 100).await;
                    }
                }
            }));

            let repo_usd = repo.clone();
            let pool_usd = pool.clone();
            result.push(tokio::spawn(async move {
                loop {
                    if let Ok(Some(mut tx)) = pool_usd.try_begin().await {
                        log_ms!("calculate_usd_1m", partition, async {
                            repo_usd.calculate_usd_1m(&mut tx, partition).await.unwrap();
                        });

                        log_ms!("calculate_usd_5m", partition, async {
                            repo_usd.calculate_usd_5m(&mut tx, partition).await.unwrap();
                        });

                        log_ms!("calculate_usd_15m", partition, async {
                            repo_usd
                                .calculate_usd_15m(&mut tx, partition)
                                .await
                                .unwrap();
                        });

                        log_ms!("calculate_usd_1h", partition, async {
                            repo_usd.calculate_usd_1h(&mut tx, partition).await.unwrap();
                        });

                        log_ms!("calculate_usd_6h", partition, async {
                            repo_usd.calculate_usd_6h(&mut tx, partition).await.unwrap();
                        });

                        log_ms!("calculate_usd_1d", partition, async {
                            repo_usd.calculate_usd_1d(&mut tx, partition).await.unwrap();
                        });

                        let _ = tx.commit().await;
                        sleep_ms(500, 1000).await;
                    } else {
                        trace!("Failed to acquire transaction");
                        sleep_ms(1, 100).await;
                    }
                }
            }));
        }
        result
    }
}

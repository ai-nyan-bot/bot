// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::pumpfun::model::summary::PumpfunSummary;
use crate::pumpfun::service::token::TokenService;
use base::model::TokenPairId;
use common::model::Timeframe;
use common::repo::error::RepoError;
use common::service::{ServiceError, ServiceResult};

impl TokenService {
    // FIXME accept timeframe
    pub async fn summarise(
        &self,
        token_pair: impl Into<TokenPairId> + Send,
    ) -> ServiceResult<PumpfunSummary> {
        let mut tx = self.pool.begin().await?;

        let result = match self.pair.get_by_id(&mut tx, token_pair).await {
            Ok(pair) => {
                let curve = match self.current.get(&mut tx, pair.id).await {
                    Ok(current) => current,
                    Err(err) => {
                        return match err {
                            RepoError::NotFound => {
                                Err(ServiceError::not_found("Current not found"))
                            }
                            _ => Err(err.into()),
                        }
                    }
                };

                let pair_id = pair.id.clone();

                PumpfunSummary {
                    pair: pair,
                    current: curve,
                    m1: self.summary.get(&mut tx, pair_id, Timeframe::M1).await.ok(),
                    m5: self.summary.get(&mut tx, pair_id, Timeframe::M5).await.ok(),
                    m15: self
                        .summary
                        .get(&mut tx, pair_id, Timeframe::M15)
                        .await
                        .ok(),
                    h1: self.summary.get(&mut tx, pair_id, Timeframe::H1).await.ok(),
                    h6: self.summary.get(&mut tx, pair_id, Timeframe::H6).await.ok(),
                    d1: self.summary.get(&mut tx, pair_id, Timeframe::D1).await.ok(),
                }
            }
            Err(err) => {
                return match err {
                    RepoError::NotFound => Err(ServiceError::not_found("TokenPair not found")),
                    _ => Err(err.into()),
                }
            }
        };

        tx.commit().await?;
        Ok(result)
    }
}

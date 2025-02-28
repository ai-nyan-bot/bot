// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Summary;
use crate::pumpfun::model::Curve;
use crate::pumpfun::service::token::TokenService;
use base::model::{TokenPair, TokenPairId};
use common::model::Timeframe;
use common::repo::error::RepoError;
use common::service::{ServiceError, ServiceResult};

#[derive(Debug)]
pub struct TokenSummary {
    pub pair: TokenPair,
    pub curve: Curve,
    pub summary: Summary,
}

impl TokenService {
    pub async fn summarise(
        &self,
        token_pair: impl Into<TokenPairId> + Send,
    ) -> ServiceResult<TokenSummary> {
        let mut tx = self.pool.begin().await?;

        let result = match self.token_pair_repo.get_by_id(&mut tx, token_pair).await {
            Ok(pair) => {
                let curve = match self.curve_repo.get(&mut tx, pair.id).await {
                    Ok(curve) => curve,
                    Err(err) => {
                        return match err {
                            RepoError::NotFound => Err(ServiceError::not_found("Curve not found")),
                            _ => Err(err.into()),
                        }
                    }
                };

                let summary = match self.summary_repo.get(&mut tx, pair.id, Timeframe::D1).await {
                    Ok(summary) => summary,
                    Err(err) => {
                        return match err {
                            RepoError::NotFound => {
                                Err(ServiceError::not_found("Summary not found"))
                            }
                            _ => Err(err.into()),
                        }
                    }
                };

                TokenSummary {
                    pair,
                    curve,
                    summary,
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

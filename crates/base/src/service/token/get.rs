// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{TokenPair, TokenPairId};
use crate::service::token::TokenService;
use common::repo::error::RepoError;
use common::service::{ServiceError, ServiceResult};

impl TokenService {
    pub async fn get_pair(
        &self,
        token_pair: impl Into<TokenPairId> + Send,
    ) -> ServiceResult<TokenPair> {
        let mut tx = self.pool.begin().await?;
        let result = match self.token_pair_repo.get_by_id(&mut tx, token_pair).await {
            Ok(pair) => pair,
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

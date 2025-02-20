// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Rule, RuleId, UserId};
use crate::service::RuleService;
use common::repo::error::RepoError;
use common::service::{ServiceError, ServiceResult};

impl RuleService {
    pub async fn get_by_id(&self, id: impl Into<RuleId> + Send) -> ServiceResult<Rule> {
        let mut tx = self.pool.begin().await?;
        let result = match self.repo.get_by_id(&mut tx, id).await {
            Ok(rule) => rule,
            Err(err) => {
                return match err {
                    RepoError::NotFound => Err(ServiceError::not_found("Rule not found")),
                    _ => Err(err.into()),
                }
            }
        };
        tx.commit().await?;
        Ok(result)
    }

    pub async fn get_by_id_user(
        &self,
        id: impl Into<RuleId> + Send,
        user: impl Into<UserId> + Send,
    ) -> ServiceResult<Rule> {
        let mut tx = self.pool.begin().await?;
        let result = match self.repo.get_by_id(&mut tx, id).await {
            Ok(rule) => {
                if rule.user != user.into() {
                    return Err(ServiceError::not_found("Rule not found"));
                }
                rule
            }
            Err(err) => {
                return match err {
                    RepoError::NotFound => Err(ServiceError::not_found("Rule not found")),
                    _ => Err(err.into()),
                }
            }
        };
        tx.commit().await?;
        Ok(result)
    }
}

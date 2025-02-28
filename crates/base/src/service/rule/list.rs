// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Rule, UserId};
use crate::repo::{RuleQueryAll, RuleQueryUser};
use crate::service::rule::RuleService;
use common::model::Limit;
use common::service::ServiceResult;

impl RuleService {
    pub async fn list_active(&self) -> ServiceResult<Box<[Rule]>> {
        let mut tx = self.pool.begin().await?;
        let result = self
            .repo
            .list_all(
                &mut tx,
                RuleQueryAll {
                    limit: Limit::unlimited(),
                },
            )
            .await?;
        tx.commit().await?;
        Ok(result)
    }

    pub async fn list_user(&self, user: impl Into<UserId>) -> ServiceResult<Vec<Rule>> {
        let mut tx = self.pool.begin().await?;
        let result = self
            .repo
            .list_user(
                &mut tx,
                RuleQueryUser {
                    user: user.into(),
                    limit: Limit::unlimited(),
                },
            )
            .await?;
        tx.commit().await?;
        Ok(result)
    }
}

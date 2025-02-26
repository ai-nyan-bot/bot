// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{AuthenticatedUser, Rule, RuleId, RuleName, Sequence};
use crate::repo;
use crate::service::RuleService;
use common::service::{ServiceError, ServiceResult};

pub struct RuleUpdateCmd {
    pub name: Option<RuleName>,
    pub sequence: Option<Sequence>,
}

impl RuleService {
    pub async fn update(&self, id: impl Into<RuleId>, cmd: RuleUpdateCmd, user: AuthenticatedUser) -> ServiceResult<Rule> {
        let mut tx = self.pool.begin().await?;
        let id = id.into();

        let rule = match self.repo.get_by_id(&mut tx, id).await {
            Ok(rule) => rule,
            Err(_) => return Err(ServiceError::not_found("Rule not found")),
        };

        if rule.user != user.id {
            return Err(ServiceError::not_found("Rule not found"));
        }

        let result = self
            .repo
            .update(
                &mut tx,
                repo::RuleUpdateCmd {
                    id,
                    user: user.id,
                    name: cmd.name.unwrap_or(rule.name),
                    sequence: cmd.sequence.unwrap_or(rule.sequence),
                },
            )
            .await?;

        tx.commit().await?;
        Ok(result)
    }
}

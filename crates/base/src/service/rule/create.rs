// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::AuthenticatedUser;
use crate::model::{Rule, RuleName, Sequence};
use crate::repo;
use crate::service::rule::RuleService;
use common::service::ServiceResult;

pub struct RuleCreateCmd {
    pub name: RuleName,
    pub sequence: Sequence,
}

impl RuleService {
    pub async fn create(&self, cmd: RuleCreateCmd, user: AuthenticatedUser) -> ServiceResult<Rule> {
        let mut tx = self.pool.begin().await?;
        let result = self
            .repo
            .create(
                &mut tx,
                repo::RuleCreateCmd {
                    user: user.id,
                    name: cmd.name,
                    sequence: cmd.sequence,
                },
            )
            .await?;
        tx.commit().await?;
        Ok(result)
    }
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::AuthenticatedUser;
use crate::model::{Action, Condition, Fact, Operator, Sequence, Rule, Value};
use crate::repo::RuleCreateCmd;
use crate::service::rule::RuleService;
use common::service::ServiceResult;

impl RuleService {
    pub async fn create(&self, user: AuthenticatedUser) -> ServiceResult<Rule> {
        let mut tx = self.pool.begin().await?;
        let result = self
            .repo
            .create(
                &mut tx,
                RuleCreateCmd {
                    user: user.id,
                    name: "Some Rule".into(),
                    sequence: Sequence {
                        condition: Condition::Compare {
                            fact: Fact::TokenPriceQuote,
                            operator: Operator::GreaterThan,
                            value: Value::Number(0.00000037974844403108274),
                            timeframe: None,
                        },
                        action: Action::Notify,
                    },
                },
            )
            .await?;
        tx.commit().await?;
        Ok(result)
    }
}

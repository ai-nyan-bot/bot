// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Action, Condition, Operator, Rule, Sequence, Value};
use crate::model::{AuthenticatedUser, Field};
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
                            field: Field::Price,
                            operator: Operator::GreaterThan,
                            value: Value::Percent(0.00000037974844403108274),
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

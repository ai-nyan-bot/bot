// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::AuthenticatedUser;
use crate::model::{Action, Condition, Fact, Operator, Sequence, Strategy, Value};
use crate::repo::StrategyCreateCmd;
use crate::service::strategy::StrategyService;
use common::service::ServiceResult;

impl StrategyService {
    pub async fn create(&self, user: AuthenticatedUser) -> ServiceResult<Strategy> {
        let mut tx = self.pool.begin().await?;
        let result = self
            .repo
            .create(
                &mut tx,
                StrategyCreateCmd {
                    user: user.id,
                    name: "Some Strategy".into(),
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

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::base::user::{get_or_create_another_user, get_or_create_test_user};
use base::model::Action::Notify;
use base::model::Condition::Compare;
use base::model::Fact::TokenCreationDuration;
use base::model::Operator::{Equal, Exists};
use base::model::{Fact, Sequence, Strategy, StrategyName, Value};
use base::repo::{StrategyCreateCmd, StrategyQueryAll, StrategyRepo};
use common::model::{Count, Limit};
use common::repo::{RepoResult, Tx};

pub async fn create_strategy_for_test_user<'a>(tx: &mut Tx<'a>, name: impl Into<StrategyName>) -> RepoResult<Strategy> {
    let test_user = get_or_create_test_user(tx).await;
    StrategyRepo::new()
        .create(
            tx,
            StrategyCreateCmd {
                user: test_user.id,
                name: name.into(),
                sequence: Sequence {
                    condition: Compare {
                        fact: TokenCreationDuration,
                        operator: Exists,
                        value: Value::Boolean(false),
                        timeframe: None,
                    },
                    action: Notify,
                },
            },
        )
        .await
}

pub async fn create_strategy_for_another_user<'a>(tx: &mut Tx<'a>, name: impl Into<StrategyName>) -> RepoResult<Strategy> {
    let another_user = get_or_create_another_user(tx).await;
    StrategyRepo::new()
        .create(
            tx,
            StrategyCreateCmd {
                user: another_user.id,
                name: name.into(),
                sequence: Sequence {
                    condition: Compare {
                        fact: Fact::TelegramGroupName,
                        operator: Equal,
                        value: Value::String("ANOTHER_TELEGRAM_GROUP_NAME".to_string()),
                        timeframe: None,
                    },
                    action: Notify,
                },
            },
        )
        .await
}

pub async fn count_all<'a>(tx: &mut Tx<'a>) -> Count {
    StrategyRepo::new().count_all(tx, StrategyQueryAll { limit: Limit::max() }).await.unwrap()
}

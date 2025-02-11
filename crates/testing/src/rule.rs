// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::user::{get_or_create_another_user, get_or_create_test_user};
use base::model::Action::Notify;
use base::model::Condition::{Compare, Exists};
use base::model::Fact::TokenCreationDuration;
use base::model::Operator::Equal;
use base::model::{Fact, Sequence, Rule, RuleName, Value};
use base::repo::{RuleCreateCmd, RuleQueryAll, RuleRepo};
use common::model::{Count, Limit};
use common::repo::Tx;

pub async fn create_rule_for_test_user<'a>(tx: &mut Tx<'a>, name: impl Into<RuleName>) -> Rule {
    let test_user = get_or_create_test_user(tx).await;
    RuleRepo::new()
        .create(
            tx,
            RuleCreateCmd {
                user: test_user.id,
                name: name.into(),
                sequence: Sequence {
                    condition: Exists {
                        fact: TokenCreationDuration,
                        timeframe: None,
                    },
                    action: Notify,
                },
            },
        )
        .await
        .unwrap()
}

pub async fn create_rule_for_another_user<'a>(tx: &mut Tx<'a>, name: impl Into<RuleName>) -> Rule {
    let another_user = get_or_create_another_user(tx).await;
    RuleRepo::new()
        .create(
            tx,
            RuleCreateCmd {
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
        .unwrap()
}

pub async fn count_all<'a>(tx: &mut Tx<'a>) -> Count {
    RuleRepo::new().count_all(tx, RuleQueryAll { limit: Limit::max() }).await.unwrap()
}

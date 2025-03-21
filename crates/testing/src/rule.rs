// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::user::{get_or_create_another_user, get_or_create_test_user};
use base::model::Action::NotifyTelegram;
use base::model::Condition::Compare;
use base::model::Operator::MoreThan;
use base::model::{Field, Rule, RuleId, RuleName, RuleStatus, Sequence, Value};
use base::repo::{RuleCreateCmd, RuleRepo, RuleUpdateCmd};
use common::model::{Count, Timeframe};
use common::repo::Tx;

pub async fn create_inactive_rule_for_test_user<'a>(
    tx: &mut Tx<'a>,
    name: impl Into<RuleName>,
) -> Rule {
    let test_user = get_or_create_test_user(tx).await;
    RuleRepo::new()
        .create(
            tx,
            RuleCreateCmd {
                user: test_user.id,
                name: name.into(),
                sequence: Sequence {
                    condition: Compare {
                        field: Field::PriceAvg,
                        operator: MoreThan,
                        value: Value::percent(2.0).into(),
                        timeframe: Some(Timeframe::M15),
                    },
                    action: NotifyTelegram { buttons: vec![] },
                },
            },
        )
        .await
        .unwrap()
}

pub async fn create_active_rule_for_test_user<'a>(
    tx: &mut Tx<'a>,
    name: impl Into<RuleName>,
) -> Rule {
    let created = create_inactive_rule_for_test_user(tx, name).await;
    RuleRepo::new()
        .update(
            tx,
            RuleUpdateCmd {
                id: created.id,
                user: created.user,
                name: created.name,
                sequence: created.sequence,
                status: RuleStatus::Active,
            },
        )
        .await
        .unwrap()
}

pub async fn create_inactive_rule_for_another_user<'a>(tx: &mut Tx<'a>, name: impl Into<RuleName>) -> Rule {
    let another_user = get_or_create_another_user(tx).await;
    RuleRepo::new()
        .create(
            tx,
            RuleCreateCmd {
                user: another_user.id,
                name: name.into(),
                sequence: Sequence {
                    condition: Compare {
                        field: Field::Volume,
                        operator: MoreThan,
                        value: Value::percent(2.0).into(),
                        timeframe: Some(Timeframe::D1),
                    },
                    action: NotifyTelegram { buttons: vec![] },
                },
            },
        )
        .await
        .unwrap()
}

pub async fn get_rule_by_id<'a>(tx: &mut Tx<'a>, id: impl Into<RuleId> + Send) -> Rule {
    RuleRepo::new().get_by_id(tx, id).await.unwrap()
}

pub async fn count_all<'a>(tx: &mut Tx<'a>) -> Count {
    RuleRepo::new().count_all(tx).await.unwrap()
}

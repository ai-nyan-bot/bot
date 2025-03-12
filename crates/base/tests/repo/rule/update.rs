// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::Action::NotifyTelegram;
use base::model::Condition::Compare;
use base::model::Field::{PriceAvg, Volume};
use base::model::Operator::{Equal, MoreThan};
use base::model::{Action, RuleStatus, Sequence, TelegramActionButtonConfig, Value};
use base::repo::{RuleRepo, RuleUpdateCmd};
use common::model::Timeframe::{H1, M15};
use common::repo::error::RepoError;
use testing::rule::{create_inactive_rule_for_test_user, get_rule_by_id};
use testing::run_test;
use testing::user::{get_or_create_another_user, get_or_create_test_user};
use RuleStatus::Inactive;
use TelegramActionButtonConfig::{Buy, Nothing, Sell};

#[test_log::test(sqlx::test)]
async fn test_update() {
    // version increments on update
    // sequence changes
    // name changes
    // user id does not change
    run_test(|mut tx| async move {
        let user = get_or_create_test_user(&mut tx).await;
        let test_instance = RuleRepo::new();

        let test_rule = create_inactive_rule_for_test_user(&mut tx, "A").await;

        let count = testing::rule::count_all(&mut tx).await;
        assert_eq!(count, 4);

        let result = test_instance
            .update(
                &mut tx,
                RuleUpdateCmd {
                    id: test_rule.id,
                    status: Inactive,
                    user: user.id,
                    name: "UPDATED".into(),
                    sequence: Sequence {
                        condition: Compare {
                            field: Volume,
                            operator: Equal,
                            value: Value::quote(42).into(),
                            timeframe: Some(H1),
                        },
                        action: NotifyTelegram {
                            buttons: vec![
                                Nothing,
                                Buy {
                                    value: Value::sol(12),
                                },
                                Sell {
                                    value: Value::percent(3.4),
                                },
                            ],
                        },
                    },
                },
            )
            .await
            .unwrap();

        assert_eq!(result.id, test_rule.id);
        assert_eq!(result.name, "UPDATED");
        assert_eq!(result.status, Inactive);
        assert_eq!(result.version, 2);
        assert_eq!(result.user, 1);
        assert_eq!(
            result.sequence.condition,
            Compare {
                field: Volume,
                operator: Equal,
                value: Value::quote(42).into(),
                timeframe: Some(H1),
            }
        );
        assert_eq!(
            result.sequence.action,
            Action::NotifyTelegram {
                buttons: vec![
                    Nothing,
                    Buy {
                        value: Value::sol(12)
                    },
                    Sell {
                        value: Value::percent(3.4)
                    }
                ]
            }
        );

        let count = testing::rule::count_all(&mut tx).await;
        assert_eq!(count, 4);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_nothing_changed() {
    // if neither name nor sequence changes the version stays as is
    run_test(|mut tx| async move {
        let user = get_or_create_test_user(&mut tx).await;
        let test_instance = RuleRepo::new();

        let test_rule = create_inactive_rule_for_test_user(&mut tx, "A").await;

        let count = testing::rule::count_all(&mut tx).await;
        assert_eq!(count, 4);

        let result = test_instance
            .update(
                &mut tx,
                RuleUpdateCmd {
                    id: test_rule.id,
                    status: RuleStatus::Active,
                    user: user.id,
                    name: "A".into(),
                    sequence: Sequence {
                        condition: Compare {
                            field: PriceAvg,
                            operator: MoreThan,
                            value: Value::percent(2.0).into(),
                            timeframe: Some(M15),
                        },
                        action: NotifyTelegram { buttons: vec![] },
                    },
                },
            )
            .await
            .unwrap();

        assert_eq!(result.id, test_rule.id);
        assert_eq!(result.name, "A");
        assert_eq!(result.user, 1);
        assert_eq!(
            result.sequence.condition,
            Compare {
                field: PriceAvg,
                operator: MoreThan,
                value: Value::percent(2.0).into(),
                timeframe: Some(M15),
            }
        );
        assert_eq!(result.sequence.action, NotifyTelegram { buttons: vec![] });

        let count = testing::rule::count_all(&mut tx).await;
        assert_eq!(count, 4);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_different_user() {
    // user id of the rule must match the user id of the command
    // does not change the row at all
    // returns not found if user id does not match
    run_test(|mut tx| async move {
        let another_user = get_or_create_another_user(&mut tx).await;

        let test_instance = RuleRepo::new();

        let test_rule = create_inactive_rule_for_test_user(&mut tx, "A").await;

        let result = test_instance
            .update(
                &mut tx,
                RuleUpdateCmd {
                    id: test_rule.id,
                    status: RuleStatus::Active,
                    user: another_user.id,
                    name: "UPDATED".into(),
                    sequence: Sequence {
                        condition: Compare {
                            field: Volume,
                            operator: Equal,
                            value: Value::quote(42).into(),
                            timeframe: Some(H1),
                        },
                        action: Action::Sell,
                    },
                },
            )
            .await;
        assert_eq!(result.err(), Some(RepoError::NotFound));

        let rule = get_rule_by_id(&mut tx, test_rule.id).await;
        assert_eq!(rule.id, 4);
        assert_eq!(rule.name, "A");
        assert_eq!(rule.version, 1);
        assert_eq!(
            rule.sequence.condition,
            Compare {
                field: PriceAvg,
                operator: MoreThan,
                value: Value::percent(2.0).into(),
                timeframe: Some(M15),
            }
        );
        assert_eq!(rule.sequence.action, NotifyTelegram { buttons: vec![] });

        let count = testing::rule::count_all(&mut tx).await;
        assert_eq!(count, 4);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_rule_not_found() {
    // returns not found if rule does not exist
    run_test(|mut tx| async move {
        let user = get_or_create_test_user(&mut tx).await;
        let test_instance = RuleRepo::new();

        let _ = create_inactive_rule_for_test_user(&mut tx, "A").await;

        let result = test_instance
            .update(
                &mut tx,
                RuleUpdateCmd {
                    id: 1234.into(),
                    status: RuleStatus::Active,
                    user: user.id,
                    name: "UPDATED".into(),
                    sequence: Sequence {
                        condition: Compare {
                            field: Volume,
                            operator: Equal,
                            value: Value::quote(42).into(),
                            timeframe: Some(H1),
                        },
                        action: NotifyTelegram { buttons: vec![] },
                    },
                },
            )
            .await;
        assert_eq!(result.err(), Some(RepoError::NotFound));

        let count = testing::rule::count_all(&mut tx).await;
        assert_eq!(count, 4);
    })
    .await
}

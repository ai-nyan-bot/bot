// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::UserId;
use base::repo::{RuleQueryUser, RuleRepo};
use common::model::Limit;
use testing::rule::create_inactive_rule_for_test_user;
use testing::{run_test, run_test_on_empty_db};

#[test_log::test(sqlx::test)]
async fn test_ok() {
    run_test(|mut tx| async move {
        let test_instance = RuleRepo::new();

        let _ = create_inactive_rule_for_test_user(&mut tx, "TheMoneyMaker").await;
        let _ = create_inactive_rule_for_test_user(&mut tx, "WinningBig").await;

        let result = test_instance
            .list_user(
                &mut tx,
                RuleQueryUser {
                    user: UserId(1),
                    limit: Limit::default(),
                },
            )
            .await
            .unwrap();

        assert_eq!(result.len(), 2);

        assert_eq!(result[0].id, 5);
        assert_eq!(result[0].user, 1);
        assert_eq!(result[0].name, "WinningBig");

        assert_eq!(result[1].id, 4);
        assert_eq!(result[1].user, 1);
        assert_eq!(result[1].name, "TheMoneyMaker");
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_nothing_found() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = RuleRepo::new();

        let result = test_instance
            .list_user(
                &mut tx,
                RuleQueryUser {
                    user: UserId(1),
                    limit: Limit::default(),
                },
            )
            .await
            .unwrap();

        assert_eq!(result.len(), 0);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_user_does_not_exists() {
    run_test(|mut tx| async move {
        let test_instance = RuleRepo::new();

        let result = test_instance
            .list_user(
                &mut tx,
                RuleQueryUser {
                    user: UserId(123456789),
                    limit: Limit::default(),
                },
            )
            .await
            .unwrap();

        assert_eq!(result.len(), 0);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_limit() {
    run_test(|mut tx| async move {
        let test_instance = RuleRepo::new();

        let _ = create_inactive_rule_for_test_user(&mut tx, "A").await;
        let _ = create_inactive_rule_for_test_user(&mut tx, "B").await;
        let _ = create_inactive_rule_for_test_user(&mut tx, "C").await;

        let result = test_instance
            .list_user(
                &mut tx,
                RuleQueryUser {
                    user: UserId(1),
                    limit: Limit(1),
                },
            )
            .await
            .unwrap();
        assert_eq!(result.len(), 1);

        assert_eq!(result[0].id, 6);
        assert_eq!(result[0].user, 1);
        assert_eq!(result[0].name, "C");

        let result = test_instance
            .list_user(
                &mut tx,
                RuleQueryUser {
                    user: UserId(1),
                    limit: Limit(2),
                },
            )
            .await
            .unwrap();
        assert_eq!(result.len(), 2);

        let result = test_instance
            .list_user(
                &mut tx,
                RuleQueryUser {
                    user: UserId(1),
                    limit: Limit(100),
                },
            )
            .await
            .unwrap();
        assert_eq!(result.len(), 3);

        assert_eq!(result[0].id, 6);
        assert_eq!(result[0].user, 1);
        assert_eq!(result[0].name, "C");

        assert_eq!(result[2].id, 4);
        assert_eq!(result[2].user, 1);
        assert_eq!(result[2].name, "A");
    })
    .await
}

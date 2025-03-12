// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::repo::{RuleQueryAll, RuleRepo};
use common::model::Limit;
use testing::rule::create_rule_for_test_user;
use testing::{run_test, run_test_on_empty_db};

#[test_log::test(sqlx::test)]
async fn test_ok() {
    run_test(|mut tx| async move {
        let test_instance = RuleRepo::new();

        let _ = create_rule_for_test_user(&mut tx, "TheMoneyMaker").await;

        let result = test_instance
            .list_active(
                &mut tx,
                RuleQueryAll {
                    limit: Limit::default(),
                },
            )
            .await
            .unwrap();
        assert_eq!(result.len(), 4);

        assert_eq!(result[0].id, 4);
        assert_eq!(result[0].user, 1);
        assert_eq!(result[0].name, "TheMoneyMaker");

        assert_eq!(result[1].id, 3);
        assert_eq!(result[1].user, 2);
        assert_eq!(result[1].name, "Rule C");

        assert_eq!(result[2].id, 2);
        assert_eq!(result[2].user, 2);
        assert_eq!(result[2].name, "Rule B");

        assert_eq!(result[3].id, 1);
        assert_eq!(result[3].user, 2);
        assert_eq!(result[3].name, "Rule A");
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_nothing_found() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = RuleRepo::new();

        let result = test_instance
            .list_active(
                &mut tx,
                RuleQueryAll {
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

        let _ = create_rule_for_test_user(&mut tx, "TheMoneyMaker").await;

        let result = test_instance
            .list_active(&mut tx, RuleQueryAll { limit: Limit(1) })
            .await
            .unwrap();
        assert_eq!(result.len(), 1);

        assert_eq!(result[0].id, 4);
        assert_eq!(result[0].user, 1);
        assert_eq!(result[0].name, "TheMoneyMaker");

        let result = test_instance
            .list_active(&mut tx, RuleQueryAll { limit: Limit(3) })
            .await
            .unwrap();
        assert_eq!(result.len(), 3);

        let result = test_instance
            .list_active(&mut tx, RuleQueryAll { limit: Limit(100) })
            .await
            .unwrap();
        assert_eq!(result.len(), 4);

        assert_eq!(result[0].id, 4);
        assert_eq!(result[0].user, 1);
        assert_eq!(result[0].name, "TheMoneyMaker");

        assert_eq!(result[3].id, 1);
        assert_eq!(result[3].user, 2);
        assert_eq!(result[3].name, "Rule A");
    })
    .await
}

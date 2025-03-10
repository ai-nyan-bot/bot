// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::Action::NotifyTelegram;
use base::model::Condition::Compare;
use base::model::Operator::MoreThan;
use base::model::{Field, Value};
use base::service::RuleService;
use common::model::Timeframe;
use common::service::ServiceError;
use testing::rule::create_rule_for_test_user;
use testing::run_test_with_pool;

#[test_log::test(sqlx::test)]
async fn test_ok() {
    run_test_with_pool(|pool| async move {
        let mut tx = pool.begin().await.unwrap();
        create_rule_for_test_user(&mut tx, "TheRuleToRuleThemAll").await;
        tx.commit().await.unwrap();

        let test_instance = RuleService::testing(pool.clone());
        let result = test_instance.get_by_id_user(4, 1).await.unwrap();

        assert_eq!(result.id, 4);
        assert_eq!(result.user, 1);
        assert_eq!(result.name, "TheRuleToRuleThemAll");

        assert_eq!(
            result.sequence.condition,
            Compare {
                field: Field::PriceAvg,
                operator: MoreThan,
                value: Value::percent(2.0).into(),
                timeframe: Some(Timeframe::M15),
            },
        );
        assert_eq!(result.sequence.action, NotifyTelegram { buttons: vec![] });
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_not_found() {
    run_test_with_pool(|pool| async move {
        let test_instance = RuleService::testing(pool.clone());
        let result = test_instance.get_by_id_user(12345, 1).await;
        assert_eq!(
            result.err().unwrap(),
            ServiceError::not_found("Rule not found")
        );
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_different_user() {
    run_test_with_pool(|pool| async move {
        let mut tx = pool.begin().await.unwrap();
        create_rule_for_test_user(&mut tx, "TheRuleToRuleThemAll").await;
        tx.commit().await.unwrap();

        let test_instance = RuleService::testing(pool.clone());
        let result = test_instance.get_by_id_user(4, 2).await;
        assert_eq!(
            result.err().unwrap(),
            ServiceError::not_found("Rule not found")
        );
    })
    .await
}

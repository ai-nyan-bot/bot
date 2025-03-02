// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::Action::NotifyTelegram;
use base::model::Condition::Compare;
use base::model::Operator::MoreThan;
use base::model::{Field, Value};
use base::service::RuleService;
use common::model::Timeframe;
use common::service::ServiceError;
use testing::run_test_with_pool;

#[test_log::test(sqlx::test)]
async fn test_ok() {
    run_test_with_pool(|pool| async move {
        let test_instance = RuleService::testing(pool.clone());
        let result = test_instance.get_by_id(3).await.unwrap();

        assert_eq!(result.id, 3);
        assert_eq!(result.user, 2);
        assert_eq!(result.name, "Rule C");

        assert_eq!(
            result.sequence.condition,
            Compare {
                field: Field::Volume,
                operator: MoreThan,
                value: Value::percent(2.0),
                timeframe: Some(Timeframe::D1),
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
        let result = test_instance.get_by_id(123).await;
        assert_eq!(
            result.err().unwrap(),
            ServiceError::not_found("Rule not found")
        );
    })
    .await
}

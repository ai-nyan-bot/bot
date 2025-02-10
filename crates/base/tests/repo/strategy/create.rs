// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::Condition::Compare;
use base::model::Fact::TokenCreationDuration;
use base::model::Operator::Exists;
use base::model::{Action, Sequence, Value};
use base::repo::{StrategyCreateCmd, StrategyRepo};
use common::repo::error::RepoError;
use sqlx::Acquire;
use testing::base::strategy::create_strategy_for_test_user;
use testing::base::user::get_or_create_test_user;
use testing::run_test_on_empty_db;

#[test_log::test(sqlx::test)]
async fn test_create() {
    run_test_on_empty_db(|mut tx| async move {
        let user = get_or_create_test_user(&mut tx).await;
        let test_instance = StrategyRepo::new();
        let result = test_instance
            .create(
                &mut tx,
                StrategyCreateCmd {
                    user: user.id,
                    name: "ChubakaStrat1337".into(),
                    sequence: Sequence {
                        condition: Compare {
                            fact: TokenCreationDuration,
                            operator: Exists,
                            value: Value::Boolean(false),
                            timeframe: None,
                        },
                        action: Action::Buy,
                    },
                },
            )
            .await
            .unwrap();

        assert_eq!(result.id, 1);
        assert_eq!(result.name, "ChubakaStrat1337");
        assert_eq!(result.version, 1);
        assert_eq!(
            result.sequence.condition,
            Compare {
                fact: TokenCreationDuration,
                operator: Exists,
                value: Value::Boolean(false),
                timeframe: None,
            }
        );
        assert_eq!(result.sequence.action, Action::Buy);

        let count = testing::base::strategy::count_all(&mut tx).await;
        assert_eq!(count, 1)
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_strategy_requires_existing_user() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = StrategyRepo::new();
        let result = test_instance
            .create(
                &mut tx.begin().await.unwrap(),
                StrategyCreateCmd {
                    user: 123456789.into(), // does not exist
                    name: "ChubakaStrat1337".into(),
                    sequence: Sequence {
                        condition: Compare {
                            fact: TokenCreationDuration,
                            operator: Exists,
                            value: Value::Boolean(false),
                            timeframe: None,
                        },
                        action: Action::Buy,
                    },
                },
            )
            .await;
        assert_eq!(result.err(), Some(RepoError::ForeignKeyViolation));

        let count = testing::base::strategy::count_all(&mut tx).await;
        assert_eq!(count, 0)
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_strategy_name_is_not_unique() {
    // Rational why wasting resources on a unique index for an edge case - if user does not want same name for the strategy it can be easily updated
    run_test_on_empty_db(|mut tx| async move {
        let first = create_strategy_for_test_user(&mut tx, "A").await;
        let second = create_strategy_for_test_user(&mut tx, "A").await;

        assert_eq!(first.user, 1);
        assert_eq!(second.user, 1);

        let count = testing::base::strategy::count_all(&mut tx).await;
        assert_eq!(count, 2)
    })
    .await
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod get_by_id {
    use base::model::Action::Notify;
    use base::model::Fact::TokenCreationDuration;
    use base::model::Operator::Exists;
    use base::model::{Condition, Value};
    use base::repo::StrategyRepo;
    use common::repo::error::RepoError;
    use testing::base::strategy::create_strategy_for_test_user;
    use testing::run_test;

    #[test_log::test(sqlx::test)]
    async fn test_ok() {
        run_test(|mut tx| async move {
            let test_instance = StrategyRepo::new();

            let strategy = create_strategy_for_test_user(&mut tx, "TheMoneyMaker").await.unwrap();

            let result = test_instance.get_by_id(&mut tx, strategy.id).await.unwrap();
            assert_eq!(result.id, 4);
            assert_eq!(result.user, 1);
            assert_eq!(result.version, 1);
            assert_eq!(result.name, "TheMoneyMaker");
            assert_eq!(
                result.sequence.condition,
                Condition::Compare {
                    fact: TokenCreationDuration,
                    operator: Exists,
                    value: Value::Boolean(false),
                    timeframe: None,
                }
            );
            assert_eq!(result.sequence.action, Notify);
        })
        .await
    }

    #[test_log::test(sqlx::test)]
    async fn test_not_found() {
        run_test(|mut tx| async move {
            let test_instance = StrategyRepo::new();

            let result = test_instance.get_by_id(&mut tx, 44444).await;
            assert_eq!(result.err(), Some(RepoError::NotFound));
        })
        .await
    }
}

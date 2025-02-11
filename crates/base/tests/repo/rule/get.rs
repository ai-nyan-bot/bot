// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod get_by_id {
    use base::model::Action::Notify;
    use base::model::Condition;
    use base::model::Fact::TokenCreationDuration;
    use base::repo::RuleRepo;
    use common::repo::error::RepoError;
    use testing::run_test;
    use testing::rule::create_rule_for_test_user;

    #[test_log::test(sqlx::test)]
    async fn test_ok() {
        run_test(|mut tx| async move {
            let test_instance = RuleRepo::new();

            let rule = create_rule_for_test_user(&mut tx, "TheMoneyMaker").await;

            let result = test_instance.get_by_id(&mut tx, rule.id).await.unwrap();
            assert_eq!(result.id, 4);
            assert_eq!(result.user, 1);
            assert_eq!(result.version, 1);
            assert_eq!(result.name, "TheMoneyMaker");
            assert_eq!(
                result.sequence.condition,
                Condition::Exists {
                    fact: TokenCreationDuration,
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
            let test_instance = RuleRepo::new();

            let result = test_instance.get_by_id(&mut tx, 44444).await;
            assert_eq!(result.err(), Some(RepoError::NotFound));
        })
        .await
    }
}

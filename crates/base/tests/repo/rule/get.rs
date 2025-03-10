// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod get_by_id {
    use base::model::Action::NotifyTelegram;
    use base::model::Field::PriceAvg;
    use base::model::Operator::MoreThan;
    use base::model::{Condition, Value};
    use base::repo::RuleRepo;
    use common::model::Timeframe::M15;
    use common::repo::error::RepoError;
    use testing::rule::create_rule_for_test_user;
    use testing::run_test;

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
                Condition::Compare {
                    field: PriceAvg,
                    operator: MoreThan,
                    value: Value::percent(2.0).into(),
                    timeframe: Some(M15),
                }
            );
            assert_eq!(result.sequence.action, NotifyTelegram { buttons: vec![] });
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

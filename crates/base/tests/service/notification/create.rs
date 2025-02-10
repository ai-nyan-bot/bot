// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod condition_met {
    use base::repo::NotificationRepo;
    use base::service::{NotificationConditionMet, NotificationService};
    use testing::notification::count_all;
    use testing::user::get_or_create_test_user;
    use testing::run_test_with_pool_on_empty_db;

    #[test_log::test(sqlx::test)]
    async fn test_ok() {
        run_test_with_pool_on_empty_db(|pool| async move {
            let mut tx = pool.begin().await.unwrap();
            let test_user = get_or_create_test_user(&mut tx).await;
            let _ = tx.commit().await.unwrap();

            let test_instance = NotificationService::new(pool.clone(), NotificationRepo::new());
            test_instance
                .create_condition_met(NotificationConditionMet {
                    user: test_user.id,
                    token_pair: 234.into(),
                })
                .await
                .unwrap();

            let mut tx = pool.begin().await.unwrap();
            let count = count_all(&mut tx).await;
            assert_eq!(count, 1);
        })
        .await
    }
}

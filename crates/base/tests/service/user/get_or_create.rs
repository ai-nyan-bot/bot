// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::service::UserService;
use testing::run_test_with_pool_on_empty_db;

#[test_log::test(sqlx::test)]
async fn test_creates_user() {
    run_test_with_pool_on_empty_db(|pool| async move {
        let test_instance = UserService::testing(pool);

        let (user, wallet, created) = test_instance
            .get_or_create_telegram_user(123)
            .await
            .unwrap();

        assert!(created);

        assert_eq!(user.id, 1);
        assert_eq!(user.telegram_id.unwrap(), 123);

        assert_eq!(wallet.id, 1);
        assert_eq!(wallet.user_id, 1);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_gets_existing_user() {
    run_test_with_pool_on_empty_db(|pool| async move {
        let test_instance = UserService::testing(pool);

        let _ = test_instance
            .get_or_create_telegram_user(123)
            .await
            .unwrap();

        let (user, wallet, created) = test_instance
            .get_or_create_telegram_user(123)
            .await
            .unwrap();

        assert!(!created);
        assert_eq!(user.id, 1);
        assert_eq!(user.telegram_id.unwrap(), 123);

        assert_eq!(wallet.id, 1);
        assert_eq!(wallet.user_id, 1);
    })
    .await
}

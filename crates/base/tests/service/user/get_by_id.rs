// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::service::UserService;
use common::service::ServiceError;
use testing::run_test_with_pool;

#[test_log::test(sqlx::test)]
async fn test_ok() {
    run_test_with_pool(|pool| async move {
        let test_instance = UserService::testing(pool);

        let result = test_instance.get_by_id(1).await.unwrap();
        assert_eq!(result.id, 1);
        assert_eq!(result.telegram_id.unwrap(), 1234);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_user_not_found() {
    run_test_with_pool(|pool| async move {
        let test_instance = UserService::testing(pool);

        let err = test_instance.get_by_id(404).await.err().unwrap();
        assert_eq!(err, ServiceError::not_found("User not found"));
    })
    .await
}

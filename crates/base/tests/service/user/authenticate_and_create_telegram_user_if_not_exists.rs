// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::service::ServiceError;
use base::service::{AuthenticateUserTelegramCmd, UserService};
use testing::run_test_with_pool_on_empty_db;

#[test_log::test(sqlx::test)]
async fn test_creates_user() {
    run_test_with_pool_on_empty_db(|pool| async move {
        let test_instance = UserService::testing(pool, || "some_token".into());

        let (user, auth, wallet) = test_instance
            .authenticate_and_create_telegram_user_if_not_exists(AuthenticateUserTelegramCmd {
                telegram_id: "123".into(),
            })
            .await
            .unwrap();

        assert_eq!(user.id, 1);
        assert_eq!(user.telegram_id.unwrap(), "123");

        assert_eq!(auth.id, 1);
        assert_eq!(auth.user.id, 1);
        assert_eq!(auth.token, "some_token");

        assert_eq!(wallet.id, 1);
        assert_eq!(wallet.user_id, 1);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_user_already_exists() {
    run_test_with_pool_on_empty_db(|pool| async move {
        let test_instance = UserService::testing(pool, || {
            static mut COUNTER: u8 = 0;
            unsafe {
                COUNTER += 1;
                return if COUNTER == 1 {
                    "token-1".into()
                } else {
                    "token-2".into()
                };
            }
        });

        let (_, auth, _) = test_instance
            .authenticate_and_create_telegram_user_if_not_exists(AuthenticateUserTelegramCmd {
                telegram_id: "123".into(),
            })
            .await
            .unwrap();

        assert_eq!(auth.id, 1);
        assert_eq!(auth.user.id, 1);
        assert_eq!(auth.token, "token-1");

        let (user, auth, wallet) = test_instance
            .authenticate_and_create_telegram_user_if_not_exists(AuthenticateUserTelegramCmd {
                telegram_id: "123".into(),
            })
            .await
            .unwrap();

        assert_eq!(user.id, 1);
        assert_eq!(user.telegram_id.unwrap(), "123");

        assert_eq!(auth.id, 2);
        assert_eq!(auth.user.id, 1);
        assert_eq!(auth.token, "token-2");

        assert_eq!(wallet.id, 1);
        assert_eq!(wallet.user_id, 1);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_token_already_exists() {
    run_test_with_pool_on_empty_db(|pool| async move {
        let test_instance = UserService::testing(pool, || "some_token".into());

        let _ = test_instance
            .authenticate_and_create_telegram_user_if_not_exists(AuthenticateUserTelegramCmd {
                telegram_id: "1".into(),
            })
            .await
            .unwrap();

        let result = test_instance
            .authenticate_and_create_telegram_user_if_not_exists(AuthenticateUserTelegramCmd {
                telegram_id: "2".into(),
            })
            .await;

        assert_eq!(
            result.err(),
            Some(ServiceError::conflict("Auth already exists"))
        )
    })
    .await
}

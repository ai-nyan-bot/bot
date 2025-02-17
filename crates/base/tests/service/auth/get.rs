// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod get_by_token {
	use base::repo::AuthRepo;
	use base::service::AuthService;
	use common::service::ServiceError;
	use testing::auth;
	use testing::user::{get_or_create_another_user, get_or_create_test_user};
	use testing::{run_test_with_pool_on_empty_db, user};

	#[test_log::test(sqlx::test)]
    async fn test_ok() {
        run_test_with_pool_on_empty_db(|pool| async move {
            let test_instance = AuthService::new(pool.clone(), AuthRepo::default());

            let mut tx = pool.begin().await.unwrap();

            get_or_create_test_user(&mut tx).await;
            get_or_create_another_user(&mut tx).await;

            let user = user::create_telegram_user(&mut tx, "ABC").await.unwrap();
            let _ = auth::create_auth(&mut tx, user.id, "token").await.unwrap();

            let _ = tx.commit().await;

            let user = test_instance.get_by_token("token").await.unwrap();
            assert_eq!(user.id, 3);
        })
        .await
    }

    #[test_log::test(sqlx::test)]
    async fn test_does_not_exists() {
        run_test_with_pool_on_empty_db(|pool| async move {
            let test_instance = AuthService::new(pool.clone(), AuthRepo::default());

            let mut tx = pool.begin().await.unwrap();

            get_or_create_test_user(&mut tx).await;
            get_or_create_another_user(&mut tx).await;

            let user = user::create_telegram_user(&mut tx, "ABC").await.unwrap();
            let _ = auth::create_auth(&mut tx, user.id, "token").await.unwrap();

			let _ = tx.commit().await;

            let result = test_instance.get_by_token("does not exist").await;
            assert_eq!(result.err(), Some(ServiceError::not_found("User not found")))
        })
        .await
    }
}

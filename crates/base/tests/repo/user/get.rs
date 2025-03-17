// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod get_by_id {
	use common::repo::error::RepoError;
	use base::repo::UserRepo;
	use testing::user::create_telegram_user;
	use testing::run_test_on_empty_db;

	#[test_log::test(sqlx::test)]
    async fn test_get() {
        run_test_on_empty_db(|mut tx| async move {
            let test_instance = UserRepo::default();

            let _ = create_telegram_user(&mut tx, 1234).await.unwrap();
            let user = test_instance.get_by_id(&mut tx, 1).await.unwrap();
            assert_eq!(user.id, 1);
            assert_eq!(user.telegram_id.unwrap(), 1234)
        })
        .await
    }

    #[test_log::test(sqlx::test)]
    async fn test_does_not_exists() {
        run_test_on_empty_db(|mut tx| async move {
            let test_instance = UserRepo::default();

            let _ = create_telegram_user(&mut tx, 1234).await.unwrap();
            let result = test_instance.get_by_id(&mut tx, 1337).await;
            assert_eq!(result.err(), Some(RepoError::NotFound))
        })
        .await
	}
}

mod get_by_telegram_id {
	use common::repo::error::RepoError;
	use base::repo::UserRepo;
	use testing::user::create_telegram_user;
	use testing::run_test_on_empty_db;

	#[test_log::test(sqlx::test)]
    async fn test_get() {
        run_test_on_empty_db(|mut tx| async move {
            let test_instance = UserRepo::default();

            let _ = create_telegram_user(&mut tx, 1234).await.unwrap();
            let user = test_instance.get_by_telegram_id(&mut tx, 1234).await.unwrap();
            assert_eq!(user.id, 1);
            assert_eq!(user.telegram_id.unwrap(), 1234)
        })
        .await
    }

    #[test_log::test(sqlx::test)]
    async fn test_does_not_exists() {
        run_test_on_empty_db(|mut tx| async move {
            let test_instance = UserRepo::default();

            let _ = create_telegram_user(&mut tx, 1234).await.unwrap();
            let result = test_instance.get_by_telegram_id(&mut tx, 404).await;
            assert_eq!(result.err(), Some(RepoError::NotFound))
        })
        .await
    }
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod get_by_id {
	use base::model::AuthToken;
	use base::repo::{AuthCreateCmd, AuthRepo};
	use common::repo::error::RepoError;
	use testing::base::user;
	use testing::run_test_on_empty_db;

	#[test_log::test(sqlx::test)]
    async fn test_ok() {
        run_test_on_empty_db(|mut tx| async move {
            let test_instance = AuthRepo::default();

            let user = user::create_telegram_user(&mut tx, "ABC").await.unwrap();
            let _ = test_instance
                .create(
                    &mut tx,
                    AuthCreateCmd {
                        user_id: user.id,
                        token: AuthToken::from("token"),
                    },
                )
                .await
                .unwrap();

            let auth = test_instance.get_by_id(&mut tx, 1).await.unwrap();
            assert_eq!(auth.id, 1);
            assert_eq!(auth.user.id, 1);
            assert_eq!(auth.token, "token");
        })
        .await
    }

    #[test_log::test(sqlx::test)]
    async fn test_does_not_exists() {
        run_test_on_empty_db(|mut tx| async move {
            let test_instance = AuthRepo::default();
            let user = user::create_telegram_user(&mut tx, "ABC").await.unwrap();
            let _ = test_instance
                .create(
                    &mut tx,
                    AuthCreateCmd {
                        user_id: user.id,
                        token: AuthToken::from("token"),
                    },
                )
                .await
                .unwrap();

            let result = test_instance.get_by_id(&mut tx, 1337).await;
            assert_eq!(result.err(), Some(RepoError::NotFound))
        })
        .await
    }
}

mod get_by_token {
	use base::model::AuthToken;
	use base::repo::{AuthCreateCmd, AuthRepo};
	use common::repo::error::RepoError;
	use testing::base::user;
	use testing::run_test_on_empty_db;

	#[test_log::test(sqlx::test)]
    async fn test_ok() {
        run_test_on_empty_db(|mut tx| async move {
            let test_instance = AuthRepo::default();

            let user = user::create_telegram_user(&mut tx, "ABC").await.unwrap();
            let _ = test_instance
                .create(
                    &mut tx,
                    AuthCreateCmd {
                        user_id: user.id,
                        token: AuthToken::from("token"),
                    },
                )
                .await
                .unwrap();

            let auth = test_instance.get_by_token(&mut tx, "token").await.unwrap();
            assert_eq!(auth.id, 1);
            assert_eq!(auth.user.id, 1);
            assert_eq!(auth.token, "token");
        })
        .await
    }

    #[test_log::test(sqlx::test)]
    async fn test_does_not_exists() {
        run_test_on_empty_db(|mut tx| async move {
            let test_instance = AuthRepo::default();

            let user = user::create_telegram_user(&mut tx, "ABC").await.unwrap();
            let _ = test_instance
                .create(
                    &mut tx,
                    AuthCreateCmd {
                        user_id: user.id,
                        token: AuthToken::from("token"),
                    },
                )
                .await
                .unwrap();

            let result = test_instance.get_by_token(&mut tx, "does not exist").await;
            assert_eq!(result.err(), Some(RepoError::NotFound))
        })
        .await
    }
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod create_telegram_user {
	use base::model::{AuthToken, UserId};
	use base::repo::{AuthCreateCmd, AuthRepo};
	use common::repo::error::RepoError;
	use sqlx::Acquire;
	use testing::auth::{count_all, list_all};
	use testing::user;
	use testing::run_test_on_empty_db;

	#[test_log::test(sqlx::test)]
    async fn test_create() {
        run_test_on_empty_db(|mut tx| async move {
            let test_instance = AuthRepo::default();

            user::create_telegram_user(&mut tx, 1).await.unwrap();
            let user = user::create_telegram_user(&mut tx, 2).await.unwrap();

            let auth = test_instance
                .create(
                    &mut tx,
                    AuthCreateCmd {
                        user: user.id,
                        token: AuthToken::from("token-1"),
                    },
                )
                .await
                .unwrap();

            assert_eq!(auth.id, 1);
            assert_eq!(auth.user.id, 2);
            assert_eq!(auth.token, "token-1");

            let count = count_all(&mut tx).await;
            assert_eq!(count, 1);
        })
        .await
    }

    #[test_log::test(sqlx::test)]
    async fn test_create_auth_for_not_existing_user() {
        run_test_on_empty_db(|mut tx| async move {
            let test_instance = AuthRepo::default();

            user::create_telegram_user(&mut tx, 1).await.unwrap();
            let _ = user::create_telegram_user(&mut tx, 2).await.unwrap();

            let result = test_instance
                .create(
                    &mut tx.begin().await.unwrap(),
                    AuthCreateCmd {
                        user: UserId::from(1337),
                        token: AuthToken::from("token-1"),
                    },
                )
                .await;
            assert_eq!(result.err().unwrap(), RepoError::ForeignKeyViolation);

            let count = count_all(&mut tx).await;
            assert_eq!(count, 0);
        })
        .await
    }

    #[test_log::test(sqlx::test)]
    async fn test_auth_token_is_unique() {
        run_test_on_empty_db(|mut tx| async move {
            let test_instance = AuthRepo::default();

            let user = user::create_telegram_user(&mut tx, 1).await.unwrap();
            let _ = test_instance
                .create(
                    &mut tx,
                    AuthCreateCmd {
                        user: user.id,
                        token: AuthToken::from("token"),
                    },
                )
                .await
                .unwrap();

            let user = user::create_telegram_user(&mut tx, 2).await.unwrap();

            let result = test_instance
                .create(
                    &mut tx.begin().await.unwrap(),
                    AuthCreateCmd {
                        user: user.id,
                        token: AuthToken::from("token"),
                    },
                )
                .await;
            assert_eq!(result.err(), Some(RepoError::AlreadyExists));

            let count = count_all(&mut tx).await;
            assert_eq!(count, 1);

            let auth = test_instance.get_by_id(&mut tx, 1).await.unwrap();
            assert_eq!(auth.id, 1);
            assert_eq!(auth.user.id, 1);
        })
        .await
    }

    #[test_log::test(sqlx::test)]
    async fn test_user_can_have_multiple_auths() {
        run_test_on_empty_db(|mut tx| async move {
            let test_instance = AuthRepo::default();

            let user = user::create_telegram_user(&mut tx, 1).await.unwrap();

            test_instance
                .create(
                    &mut tx,
                    AuthCreateCmd {
                        user: user.id,
                        token: AuthToken::from("token-1"),
                    },
                )
                .await
                .unwrap();

            test_instance
                .create(
                    &mut tx,
                    AuthCreateCmd {
                        user: user.id,
                        token: AuthToken::from("token-2"),
                    },
                )
                .await
                .unwrap();

            test_instance
                .create(
                    &mut tx,
                    AuthCreateCmd {
                        user: user.id,
                        token: AuthToken::from("token-3"),
                    },
                )
                .await
                .unwrap();

            let count = count_all(&mut tx).await;
            assert_eq!(count, 3);

            let list = list_all(&mut tx).await;
            assert_eq!(list.len(), 3);

            let first = list.first().cloned().unwrap();
            assert_eq!(first.id, 3);
            assert_eq!(first.user.id, 1);
            assert_eq!(first.token, "token-3");

            let last = list.last().cloned().unwrap();
            assert_eq!(last.id, 1);
            assert_eq!(last.user.id, 1);
            assert_eq!(last.token, "token-1");
        })
        .await
    }

    #[test_log::test(sqlx::test)]
    async fn test_multiple_user_with_multiple_auths() {
        run_test_on_empty_db(|mut tx| async move {
            let test_instance = AuthRepo::default();

            let user = user::create_telegram_user(&mut tx, 1).await.unwrap();
            let _ = test_instance
                .create(
                    &mut tx,
                    AuthCreateCmd {
                        user: user.id,
                        token: AuthToken::from("token-1-1"),
                    },
                )
                .await
                .unwrap();

            let _ = test_instance
                .create(
                    &mut tx,
                    AuthCreateCmd {
                        user: user.id,
                        token: AuthToken::from("token-1-2"),
                    },
                )
                .await
                .unwrap();

            let user = user::create_telegram_user(&mut tx, 2).await.unwrap();
            let _ = test_instance
                .create(
                    &mut tx,
                    AuthCreateCmd {
                        user: user.id,
                        token: AuthToken::from("token-2-1"),
                    },
                )
                .await
                .unwrap();

            let _ = test_instance
                .create(
                    &mut tx,
                    AuthCreateCmd {
                        user: user.id,
                        token: AuthToken::from("token-2-2"),
                    },
                )
                .await
                .unwrap();

            let count = count_all(&mut tx).await;
            assert_eq!(count, 4);
        })
        .await
    }
}

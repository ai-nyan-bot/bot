// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod create_telegram_user {
    use base::model::UserId;
    use common::repo::error::RepoError;
    use sqlx::Acquire;
    use testing::base::user::{count_all, create_telegram_user, list_all};
    use testing::run_test_on_empty_db;

    #[test_log::test(sqlx::test)]
    async fn test_create() {
        run_test_on_empty_db(|mut tx| async move {
            let user = create_telegram_user(&mut tx, "ABC").await.unwrap();
            assert_eq!(user.id, UserId(1));
            assert_eq!(user.telegram_id.unwrap(), "ABC");

            let count = count_all(&mut tx).await;
            assert_eq!(count, 1);
        })
        .await
    }

    #[test_log::test(sqlx::test)]
    async fn test_already_exists() {
        run_test_on_empty_db(|mut tx| async move {
            let user = create_telegram_user(&mut tx, "ABC").await.unwrap();
            assert_eq!(user.id, UserId(1));
            assert_eq!(user.telegram_id.unwrap(), "ABC");

            let result = create_telegram_user(&mut tx.begin().await.unwrap(), "ABC").await;
            assert_eq!(result.err(), Some(RepoError::AlreadyExists));

            let count = count_all(&mut tx).await;
            assert_eq!(count, 1);
        })
        .await
    }

    #[test_log::test(sqlx::test)]
    async fn test_create_many() {
        run_test_on_empty_db(|mut tx| async move {
            let _ = create_telegram_user(&mut tx, "1").await.unwrap();
            let _ = create_telegram_user(&mut tx, "2").await.unwrap();
            let _ = create_telegram_user(&mut tx, "3").await.unwrap();
            let _ = create_telegram_user(&mut tx, "4").await.unwrap();

            let count = count_all(&mut tx).await;
            assert_eq!(count, 4);

            let list = list_all(&mut tx).await;
            assert_eq!(list.len(), 4);

            let first = list.first().cloned().unwrap();
            assert_eq!(first.id, 4);
            assert_eq!(first.telegram_id.unwrap(), "4");

            let last = list.last().cloned().unwrap();
            assert_eq!(last.id, 1);
            assert_eq!(last.telegram_id.unwrap(), "1")
        })
        .await
    }
}

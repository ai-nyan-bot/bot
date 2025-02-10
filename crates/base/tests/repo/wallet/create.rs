// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::repo::error::RepoError;
use base::repo::WalletRepo;
use sqlx::Acquire;
use testing::user::create_telegram_user;
use testing::wallet::count_all;
use testing::{run_test_on_empty_db, run_test_with_pool_on_empty_db, user, wallet};

use crate::repo::wallet::{create_wallet, PRIVATE_KEY, PUBLIC_KEY};

#[test_log::test(sqlx::test)]
async fn test_create_wallet() {
    run_test_on_empty_db(|mut tx| async move {
        let _ = create_telegram_user(&mut tx, "1").await.unwrap();

        let user = create_telegram_user(&mut tx, "2").await.unwrap();
        let _ = wallet::create_wallet(&mut tx, user.id).await;

        let count = count_all(&mut tx).await;
        assert_eq!(count, 1);

        let user = create_telegram_user(&mut tx, "3").await.unwrap();
        let wallet = create_wallet(&mut tx, user.id, PUBLIC_KEY.clone(), PRIVATE_KEY.clone()).await.unwrap();

        assert_eq!(wallet.id, 2);
        assert_eq!(wallet.user_id, 3);
        assert_eq!(wallet.solana_public_key, *PUBLIC_KEY);
        assert_eq!(wallet.solana_private_key, *PRIVATE_KEY);

        let count = count_all(&mut tx).await;
        assert_eq!(count, 2);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_tries_to_create_wallet_for_not_existing_user() {
    run_test_on_empty_db(|mut tx| async move {
        let _ = create_telegram_user(&mut tx, "ABC").await.unwrap();

        let result = wallet::create_wallet(&mut tx.begin().await.unwrap(), 3).await;
        assert_eq!(result.err().unwrap(), RepoError::ForeignKeyViolation);

        let count = count_all(&mut tx).await;
        assert_eq!(count, 0);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_one_wallet_per_user() {
    run_test_with_pool_on_empty_db(|pool| async move {
        let test_instance = WalletRepo::default();

        let mut tx = pool.begin().await.unwrap();

        user::create_telegram_user(&mut tx, "1").await.unwrap();
        let user = user::create_telegram_user(&mut tx, "2").await.unwrap();

        let _ = wallet::create_wallet(&mut tx, user.id).await.unwrap();

        tx.commit().await.unwrap();

        let mut tx = pool.begin().await.unwrap();
        let result = wallet::create_wallet(&mut tx, user.id).await;
        assert_eq!(result.err().unwrap(), RepoError::AlreadyExists);

        let mut tx = pool.begin().await.unwrap();
        let count = count_all(&mut tx).await;
        assert_eq!(count, 1);

        let wallet = test_instance.get_by_id(&mut tx, 1).await.unwrap();
        assert_eq!(wallet.id, 1);
        assert_eq!(wallet.user_id, 2)
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_solana_public_key_is_unique() {
    run_test_on_empty_db(|mut tx| async move {
        let user = create_telegram_user(&mut tx, "1").await.unwrap();
        let _ = create_wallet(&mut tx, user.id, PUBLIC_KEY.clone(), PRIVATE_KEY.clone()).await.unwrap();

        let user = create_telegram_user(&mut tx, "2").await.unwrap();
        let result = create_wallet(
            &mut tx.begin().await.unwrap(),
            user.id,
            PUBLIC_KEY.clone(),
            "3GTDJmgjXjJKrAHkgYhkzgRDa1nnZq1nkFbLCX5EtbtfYPMF6mUtHvizhSYkuh6mkuKUjWPsNr37sqRZ6fHbiAX6",
        )
        .await;
        assert_eq!(result.err().unwrap(), RepoError::AlreadyExists);

        let count = count_all(&mut tx).await;
        assert_eq!(count, 1);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_solana_private_key_is_unique() {
    run_test_on_empty_db(|mut tx| async move {
        let user = create_telegram_user(&mut tx, "1").await.unwrap();
        let _ = create_wallet(&mut tx, user.id, PUBLIC_KEY.clone(), PRIVATE_KEY.clone()).await.unwrap();

        let user = create_telegram_user(&mut tx, "2").await.unwrap();
        let result = create_wallet(
            &mut tx.begin().await.unwrap(),
            user.id,
            "Bo9gdG7nRdqWW68MAJNz4vYfNNYFiCdoNnc54RYqmhUy",
            PRIVATE_KEY.clone(),
        )
        .await;

        assert_eq!(result.err().unwrap(), RepoError::AlreadyExists);

        let count = count_all(&mut tx).await;
        assert_eq!(count, 1);
    })
    .await
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::repo::wallet::{create_wallet, PRIVATE_KEY, PUBLIC_KEY};
use base::repo::WalletRepo;
use common::crypt::Nonce;
use common::repo::error::RepoError;
use testing::run_test_on_empty_db;
use testing::user::create_telegram_user;

#[test_log::test(sqlx::test)]
async fn test_get() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = WalletRepo::default();
        let _ = create_telegram_user(&mut tx, 1).await.unwrap();

        let user = create_telegram_user(&mut tx, 2).await.unwrap();
        let _ = testing::wallet::create_wallet(&mut tx, user.id).await;

        let user = create_telegram_user(&mut tx, 3).await.unwrap();
        let _ = create_wallet(
            &mut tx,
            user.id,
            PUBLIC_KEY.clone(),
            PRIVATE_KEY.clone(),
            Nonce::generate(),
        )
        .await
        .unwrap();

        let wallet = test_instance.get_by_id(&mut tx, 2).await.unwrap();
        assert_eq!(wallet.id, 2);
        assert_eq!(wallet.user_id, 3);
        assert_eq!(wallet.public_key, *PUBLIC_KEY);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_does_not_exists() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = WalletRepo::default();
        let user = create_telegram_user(&mut tx, 123).await.unwrap();
        let _ = testing::wallet::create_wallet(&mut tx, user.id).await;

        let result = test_instance.get_by_id(&mut tx, 1337).await;
        assert_eq!(result.err(), Some(RepoError::NotFound))
    })
    .await
}

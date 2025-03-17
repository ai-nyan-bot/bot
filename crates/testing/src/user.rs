// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::User;
use common::model::{Count, Limit, TelegramId};
use common::repo::{RepoResult, Tx};

use crate::auth::create_auth;
use base::repo::{UserCreateTelegramCmd, UserQueryAll, UserRepo};

const USER_REPO: UserRepo = UserRepo {};

pub async fn get_or_create_test_user<'a>(tx: &mut Tx<'a>) -> User {
    // assumes that the user under test always has the id 1
    match USER_REPO.get_by_id(tx, 1).await {
        Ok(user) => user,
        Err(_) => {
            let result = create_telegram_user(tx, 1234).await.unwrap();
            create_auth(tx, 1, "TestUserToken").await.unwrap();
            assert_eq!(result.id, 1);
            result
        }
    }
}

pub async fn get_or_create_another_user<'a>(tx: &mut Tx<'a>) -> User {
    // assumes that a different user  always has the id 2
    match USER_REPO.get_by_id(tx, 2).await {
        Ok(user) => user,
        Err(_) => {
            let result = create_telegram_user(tx, 5432).await.unwrap();
            create_auth(tx, 2, "AnotherUserToken").await.unwrap();
            assert_eq!(result.id, 2);
            result
        }
    }
}

pub async fn create_telegram_user<'a>(
    tx: &mut Tx<'a>,
    telegram_id: impl Into<TelegramId>,
) -> RepoResult<User> {
    USER_REPO
        .create_telegram(
            tx,
            UserCreateTelegramCmd {
                telegram_id: telegram_id.into(),
            },
        )
        .await
}

pub async fn count_all<'a>(tx: &mut Tx<'a>) -> Count {
    USER_REPO.count(tx).await.unwrap()
}

pub async fn list_all<'a>(tx: &mut Tx<'a>) -> Box<[User]> {
    USER_REPO
        .list(
            tx,
            UserQueryAll {
                limit: Limit::max(),
            },
        )
        .await
        .unwrap()
}

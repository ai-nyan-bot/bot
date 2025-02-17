// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{Auth, AuthToken, UserId};
use common::model::{Count, Limit};
use common::repo::{RepoResult, Tx};

use base::repo::{AuthCreateCmd, AuthQueryAll, AuthRepo};

const AUTH_REPO: AuthRepo = AuthRepo {};

pub async fn create_auth<'a>(tx: &mut Tx<'a>, id: impl Into<UserId>, token: impl Into<AuthToken>) -> RepoResult<Auth> {
    AUTH_REPO
        .create(
            tx,
            AuthCreateCmd {
                user: id.into(),
                token: token.into(),
            },
        )
        .await
}

pub async fn count_all<'a>(tx: &mut Tx<'a>) -> Count {
    AUTH_REPO.count(tx, AuthQueryAll { limit: Limit::max() }).await.unwrap()
}

pub async fn list_all<'a>(tx: &mut Tx<'a>) -> Box<[Auth]> {
    AUTH_REPO.list(tx, AuthQueryAll { limit: Limit::max() }).await.unwrap()
}

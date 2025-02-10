// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::Auth;
use common::model::{Count, Limit};
use common::repo::Tx;

use base::repo::{AuthQueryAll, AuthRepo};

const AUTH_REPO: AuthRepo = AuthRepo {};

pub async fn count_all<'a>(tx: &mut Tx<'a>) -> Count {
    AUTH_REPO.count(tx, AuthQueryAll { limit: Limit::max() }).await.unwrap()
}

pub async fn list_all<'a>(tx: &mut Tx<'a>) -> Box<[Auth]> {
    AUTH_REPO.list(tx, AuthQueryAll { limit: Limit::max() }).await.unwrap()
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::repo::TokenRepo;
use common::model::Count;
use common::repo::Tx;

pub async fn count_all<'a>(tx: &mut Tx<'a>) -> Count {
    let repo = TokenRepo::testing_no_token_info();
    repo.count(tx).await.unwrap()
}

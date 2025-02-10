// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::model::{Count, Limit};
use common::repo::Tx;
use solana::repo::solana::{ReadTokenRepo, TokenQuery};

pub async fn count_all<'a>(tx: &mut Tx<'a>) -> Count {
    let repo = ReadTokenRepo::new();
    repo.count(tx, TokenQuery { limit: Limit::max() }).await.unwrap()
}

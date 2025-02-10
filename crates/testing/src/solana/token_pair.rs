// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::model::{Count, Limit};
use common::repo::Tx;
use solana::repo::solana::{ReadTokenPairRepo, ReadTokenRepo, TokenPairQuery};

pub async fn count_all<'a>(tx: &mut Tx<'a>) -> Count {
    let repo = ReadTokenPairRepo::new(ReadTokenRepo::new());
    repo.count(tx, TokenPairQuery { limit: Limit::max() }).await.unwrap()
}

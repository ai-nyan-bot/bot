// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::repo::{ReadTokenPairRepo, ReadTokenRepo, TokenPairQuery, TokenPairRepo, TokenRepo};
use common::model::{Count, Limit, TokenMint, TokenPair};
use common::repo::Tx;
use base::test::SuccessfulTokenInfoLoader;

pub async fn get_or_create_token_pair<'a>(tx: &mut Tx<'a>, base: impl Into<TokenMint> + Send, quote: impl Into<TokenMint> + Send) -> TokenPair {
    TokenPairRepo::new(
        TokenRepo::new(SuccessfulTokenInfoLoader::default(), ReadTokenRepo::new()),
        ReadTokenPairRepo::new(ReadTokenRepo::new()),
    )
    .list_or_populate_by_mints(tx, vec![(base, quote)])
    .await
    .unwrap()
    .pop()
    .unwrap()
}

pub async fn count_all<'a>(tx: &mut Tx<'a>) -> Count {
    let repo = ReadTokenPairRepo::new(ReadTokenRepo::new());
    repo.count(tx, TokenPairQuery { limit: Limit::max() }).await.unwrap()
}

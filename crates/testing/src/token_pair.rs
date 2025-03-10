// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{Mint, TokenPair};
use base::repo::{TokenPairRepo, TokenRepo};
use base::test::{NeverCalledTokenInfoLoader, SuccessfulTokenInfoLoader};
use common::model::Count;
use common::repo::Tx;

pub async fn get_or_create_token_pair<'a>(
    tx: &mut Tx<'a>,
    base: impl Into<Mint> + Send,
    quote: impl Into<Mint> + Send,
) -> TokenPair {
    TokenPairRepo::new(TokenRepo::new(Box::new(
        SuccessfulTokenInfoLoader::default(),
    )))
    .list_or_populate(tx, vec![(base, quote)])
    .await
    .unwrap()
    .pop()
    .unwrap()
}

pub async fn count_all<'a>(tx: &mut Tx<'a>) -> Count {
    let repo = TokenPairRepo::new(TokenRepo::new(Box::new(NeverCalledTokenInfoLoader {})));
    repo.count(tx).await.unwrap()
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::solana::state::State;
use base::repo::TokenToInsert;
use common::repo::Tx;

pub(crate) async fn index_tokens<'a>(tx: &mut Tx<'a>, state: State, mints: Vec<TokenToInsert>) {
    // let inserted = state.token_repo.insert_token(tx, mints).await.unwrap();
    // debug!("Inserted token: {:?}", inserted);
}

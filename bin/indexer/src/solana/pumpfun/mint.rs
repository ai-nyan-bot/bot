// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::solana::state::State;
use base::model::Mint;
use base::repo::TokenToInsert;
use base::LoadTokenInfo;
use common::repo::Tx;
use tracing::debug;

pub(crate) async fn index_tokens<'a, L: LoadTokenInfo<Mint> + Clone>(
    tx: &mut Tx<'a>,
    state: State<L>,
    mints: Vec<TokenToInsert>,
) {
    // let inserted = state.token_repo.insert_token(tx, mints).await.unwrap();
    // debug!("Inserted token: {:?}", inserted);
}

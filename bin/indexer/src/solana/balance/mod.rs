// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::solana::state::State;
use base::repo::TokenBalanceToInsert;
use common::repo::Tx;

pub(crate) async fn index_token_balance<'a>(
    tx: &mut Tx<'a>,
    state: State,
    balances: Vec<TokenBalanceToInsert>,
) {
    state
        .token_balance_repo
        .insert_token_balances(tx, balances)
        .await
        .unwrap();
}

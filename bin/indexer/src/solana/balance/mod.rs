// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::solana::state::State;
use common::repo::Tx;
use solana::repo::{SolBalanceToInsert, TokenBalanceToInsert};

pub(crate) async fn index_token_balance<'a>(
    tx: &mut Tx<'a>,
    state: State,
    balances: Vec<TokenBalanceToInsert>,
) {
    state
        .balance_repo
        .insert_token_balances(tx, balances)
        .await
        .unwrap();
}

pub(crate) async fn index_sol_balance<'a>(
    tx: &mut Tx<'a>,
    state: State,
    balances: Vec<SolBalanceToInsert>,
) {
    state
        .balance_repo
        .insert_sol_balances(tx, balances)
        .await
        .unwrap();
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::solana::state::State;
use common::repo::Tx;
use solana::pumpfun::repo::SlotTrades;

pub(crate) async fn index_trade<'a>(tx: &mut Tx<'a>, state: State, trades: SlotTrades) {
    // FIXME ensure slot is after last slot

    // use returned trades to update curves

    let inserted = state
        .pumpfun_trade_repo
        .insert_trades(tx, trades)
        .await
        .unwrap();

    for trade in inserted {
        state.pumpfun_curve_repo.upsert(tx, trade).await.unwrap();
    }

    // group trades by token_pair_id
    // list or populate bonding curves
    // use trades to update bonding curves

    println!("done")
}

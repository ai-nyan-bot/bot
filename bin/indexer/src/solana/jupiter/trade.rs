// // Copyright (c) nyanbot.com 2025.
// // This file is licensed under the AGPL-3.0-or-later.

use crate::solana::state::State;
use common::repo::Tx;
use solana::jupiter::repo::SlotTrades;

pub(crate) async fn index_trade<'a>(tx: &mut Tx<'a>, state: State, trades: SlotTrades) {
    let inserted = state.jupiter_trade_repo.insert_trades(tx, trades).await.unwrap();

    println!("done")
}

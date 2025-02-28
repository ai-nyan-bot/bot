// // Copyright (c) nyanbot.com 2025.
// // This file is licensed under the AGPL-3.0-or-later.

use crate::solana::state::State;
use base::model::Mint;
use base::LoadTokenInfo;
use common::repo::Tx;
use solana::jupiter::repo::SlotTrades;

pub(crate) async fn index_trade<'a, L: LoadTokenInfo<Mint>>(tx: &mut Tx<'a>, state: State<L>, trades: SlotTrades) {
    let _ = state
        .jupiter_trade_repo
        .insert_trades(tx, trades)
        .await
        .unwrap();
}

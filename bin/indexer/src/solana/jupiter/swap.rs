// // Copyright (c) nyanbot.com 2025.
// // This file is licensed under the AGPL-3.0-or-later.

use crate::solana::state::State;
use common::repo::Tx;
use solana::jupiter::repo::SlotSwaps;

pub(crate) async fn index_swap<'a>(tx: &mut Tx<'a>, state: State, swaps: SlotSwaps) {
    let _ = state
        .jupiter_swap_repo
        .insert_swaps(tx, swaps)
        .await
        .unwrap();
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::test::SuccessfulTokenInfoLoader;
use common::model::Count;
use common::repo::Tx;
use solana::jupiter::model::Swap;
use solana::jupiter::repo::{ReadSwapRepo, SlotSwaps, SwapRepo};
use solana::model::Signature;

pub async fn list_all_swaps<'a>(tx: &mut Tx<'a>) -> Vec<Swap> {
    ReadSwapRepo::new().list(tx).await.unwrap()
}

pub async fn list_with_signature<'a>(
    tx: &mut Tx<'a>,
    signature: impl Into<Signature>,
) -> Vec<Swap> {
    ReadSwapRepo::new()
        .list_with_signature(tx, signature)
        .await
        .unwrap()
}


pub async fn list_micro_with_signature<'a>(
    tx: &mut Tx<'a>,
    signature: impl Into<Signature>,
) -> Vec<Swap> {
    ReadSwapRepo::new()
        .list_micro_with_signature(tx, signature)
        .await
        .unwrap()
}


pub async fn count_swaps<'a>(tx: &mut Tx<'a>) -> Count {
    ReadSwapRepo::new().count_swap(tx).await.unwrap()
}

pub async fn count_micro_swaps<'a>(tx: &mut Tx<'a>) -> Count {
    ReadSwapRepo::new().count_micro_swap(tx).await.unwrap()
}

pub async fn insert_swap<'a>(tx: &mut Tx<'a>, slot_swaps: SlotSwaps) -> Vec<Swap> {
    SwapRepo::testing(Box::new(SuccessfulTokenInfoLoader::default()))
        .insert_swaps(tx, slot_swaps)
        .await
        .unwrap()
}

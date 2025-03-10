// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::test::SuccessfulTokenInfoLoader;
use common::model::Count;
use common::repo::Tx;
use solana::model::Signature;
use solana::pumpfun::model::Swap;
use solana::pumpfun::repo::{CurveRepo, ReadSwapRepo, SlotSwaps, SwapRepo};

pub async fn count_all_curves<'a>(tx: &mut Tx<'a>) -> Count {
    CurveRepo::new().count_all(tx).await.unwrap()
}

pub async fn count_all_swaps<'a>(tx: &mut Tx<'a>) -> Count {
    ReadSwapRepo::new().count_all(tx).await.unwrap()
}

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

pub async fn insert_swap<'a>(tx: &mut Tx<'a>, slot_swaps: SlotSwaps) -> Vec<Swap> {
    SwapRepo::testing(Box::new(SuccessfulTokenInfoLoader::default()))
        .insert_swaps(tx, slot_swaps)
        .await
        .unwrap()
}

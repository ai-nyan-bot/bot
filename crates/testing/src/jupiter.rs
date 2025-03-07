// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::test::SuccessfulTokenInfoLoader;
use common::model::Count;
use common::repo::Tx;
use solana::jupiter::model::Trade;
use solana::jupiter::repo::{ReadTradeRepo, SlotTrades, TradeRepo};
use solana::model::Signature;

pub async fn list_all_trades<'a>(tx: &mut Tx<'a>) -> Vec<Trade> {
    ReadTradeRepo::new().list(tx).await.unwrap()
}

pub async fn list_with_signature<'a>(
    tx: &mut Tx<'a>,
    signature: impl Into<Signature>,
) -> Vec<Trade> {
    ReadTradeRepo::new()
        .list_with_signature(tx, signature)
        .await
        .unwrap()
}

pub async fn count_all_trades<'a>(tx: &mut Tx<'a>) -> Count {
    ReadTradeRepo::new().count_all(tx).await.unwrap()
}

pub async fn insert_trade<'a>(tx: &mut Tx<'a>, slot_trades: SlotTrades) -> Vec<Trade> {
    TradeRepo::testing(SuccessfulTokenInfoLoader::default())
        .insert_trades(tx, slot_trades)
        .await
        .unwrap()
}

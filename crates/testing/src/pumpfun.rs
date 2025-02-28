// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::test::SuccessfulTokenInfoLoader;
use common::model::Count;
use common::repo::Tx;
use solana::model::Signature;
use solana::pumpfun::model::{CurveLog, Trade};
use solana::pumpfun::repo::{CurveRepo, ReadTradeRepo, SlotTrades, TradeRepo};

pub async fn count_all_curves<'a>(tx: &mut Tx<'a>) -> Count {
    CurveRepo::new().count_all(tx).await.unwrap()
}

pub async fn count_all_curve_logs<'a>(tx: &mut Tx<'a>) -> Count {
    CurveRepo::new().count_log_all(tx).await.unwrap()
}

pub async fn list_all_curve_logs<'a>(tx: &mut Tx<'a>) -> Vec<CurveLog> {
    CurveRepo::new().list_log_all(tx).await.unwrap()
}

pub async fn count_all_trades<'a>(tx: &mut Tx<'a>) -> Count {
    ReadTradeRepo::new().count_all(tx).await.unwrap()
}

pub async fn list_all_trades<'a>(tx: &mut Tx<'a>) -> Vec<Trade> {
    ReadTradeRepo::new().list(tx).await.unwrap()
}

pub async fn list_of_tx<'a>(tx: &mut Tx<'a>, signature: impl Into<Signature>) -> Vec<Trade> {
    ReadTradeRepo::new().list_of_tx(tx, signature).await.unwrap()
}

pub async fn insert_trade<'a>(tx: &mut Tx<'a>, slot_trades: SlotTrades) -> Vec<Trade> {
    TradeRepo::testing(SuccessfulTokenInfoLoader::default())
        .insert_trades(tx, slot_trades)
        .await
        .unwrap()
}

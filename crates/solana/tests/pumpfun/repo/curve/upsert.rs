// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::pumpfun::repo::curve::{inserting_slot_trades, updating_slot_trades};
use solana::model::Signature;
use solana::pumpfun::repo::CurveRepo;
use testing::pumpfun::{count_all_curve_logs, count_all_curves, insert_trade};
use testing::run_test_on_empty_db;

#[test_log::test(sqlx::test)]
async fn test_creates_curve() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = CurveRepo::testing();
        let test_trade = insert_trade(&mut tx, inserting_slot_trades())
            .await
            .pop()
            .unwrap();

        let result = test_instance.upsert(&mut tx, test_trade).await.unwrap();
        assert_eq!(result.id, 1000);
        assert_eq!(result.slot, 12345);
        assert_eq!(result.virtual_base_reserves, 512561011366544);
        assert_eq!(result.virtual_quote_reserves, 62802280169);
        assert_eq!(result.progress, 70.66435);
        assert!(!result.complete);

        let count = count_all_curves(&mut tx).await;
        assert_eq!(count, 1);

        let count = count_all_curve_logs(&mut tx).await;
        assert_eq!(count, 1);
    })
    .await;
}

#[test_log::test(sqlx::test)]
async fn test_update_curve() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = CurveRepo::testing();

        let inserting_trade = insert_trade(&mut tx, inserting_slot_trades())
            .await
            .pop()
            .unwrap();

        let updating_trade = insert_trade(&mut tx, updating_slot_trades())
            .await
            .pop()
            .unwrap();

        let _ = test_instance
            .upsert(&mut tx, inserting_trade)
            .await
            .unwrap();

        let result = test_instance.upsert(&mut tx, updating_trade).await.unwrap();
        assert_eq!(result.id, 1000);
        assert_eq!(result.slot, 23456);
        assert_eq!(result.virtual_base_reserves, 0);
        assert_eq!(result.virtual_quote_reserves, 0);
        assert_eq!(result.progress, 100);
        assert!(result.complete);

        let count = count_all_curves(&mut tx).await;
        assert_eq!(count, 1);

        let count = count_all_curve_logs(&mut tx).await;
        assert_eq!(count, 2);
    })
    .await;
}

#[test_log::test(sqlx::test)]
async fn test_curve_updated_at_same_slot() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = CurveRepo::testing();

        let inserting_trade = insert_trade(&mut tx, inserting_slot_trades())
            .await
            .pop()
            .unwrap();

        let mut updating_slot_trades = inserting_slot_trades();
        let trade = updating_slot_trades.trades.get_mut(0).unwrap();
        trade.signature = Signature::from("signaure2");
        trade.virtual_base_reserves = 757214460226289i64.into();
        trade.virtual_quote_reserves = 42511074286i64.into();

        let updating_trade = insert_trade(&mut tx, updating_slot_trades)
            .await
            .pop()
            .unwrap();

        let _ = test_instance
            .upsert(&mut tx, inserting_trade)
            .await
            .unwrap();

        let result = test_instance.upsert(&mut tx, updating_trade).await.unwrap();

        assert_eq!(result.id, 1000);
        assert_eq!(result.slot, 12345);
        assert_eq!(result.virtual_base_reserves, 757214460226289i64);
        assert_eq!(result.virtual_quote_reserves, 42511074286i64);
        assert_eq!(result.progress, 39.816612);
        assert!(!result.complete);
    })
    .await;
}

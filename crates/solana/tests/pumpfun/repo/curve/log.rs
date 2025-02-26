// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::pumpfun::repo::curve::{inserting_slot_trades, updating_slot_trades};
use solana::model::Signature;
use solana::pumpfun::repo::CurveRepo;
use testing::pumpfun::{count_all_curve_logs, insert_trade, list_all_curve_logs};
use testing::run_test_on_empty_db;

#[test_log::test(sqlx::test)]
async fn test_curve_inserted() {
    // Ensures that if a curve gets inserted, a log will be created
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = CurveRepo::testing();

        let inserting_trade = insert_trade(&mut tx, inserting_slot_trades())
            .await
            .pop()
            .unwrap();

        let _ = test_instance
            .upsert(&mut tx, inserting_trade)
            .await
            .unwrap();

        let count = count_all_curve_logs(&mut tx).await;
        assert_eq!(count, 1);

        let mut logs = list_all_curve_logs(&mut tx).await;
        assert_eq!(count, 1);

        let log = logs.pop().unwrap();
        assert_eq!(log.id, 1000);
        assert_eq!(log.slot, 12345);
        assert_eq!(log.virtual_base_reserves, 512561011366544);
        assert_eq!(log.virtual_quote_reserves, 62802280169);
        assert_eq!(log.progress, 70.66435);
        assert!(!log.complete);
    })
    .await;
}

#[test_log::test(sqlx::test)]
async fn test_curve_updated() {
    // Ensures that if a curve gets updated, another log will be created
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

        let _ = test_instance.upsert(&mut tx, updating_trade).await.unwrap();

        let count = count_all_curve_logs(&mut tx).await;
        assert_eq!(count, 2);

        let mut logs = list_all_curve_logs(&mut tx).await;
        assert_eq!(count, 2);

        let second = logs.pop().unwrap();
        assert_eq!(second.id, 1000);
        assert_eq!(second.slot, 12345);
        assert_eq!(second.virtual_base_reserves, 512561011366544);
        assert_eq!(second.virtual_quote_reserves, 62802280169);
        assert_eq!(second.progress, 70.66435);
        assert!(!second.complete);

        let first = logs.pop().unwrap();
        assert_eq!(first.id, 1000);
        assert_eq!(first.slot, 23456);
        assert_eq!(first.virtual_base_reserves, 0);
        assert_eq!(first.virtual_quote_reserves, 0);
        assert_eq!(first.progress, 100);
        assert!(first.complete);
    })
    .await;
}

#[test_log::test(sqlx::test)]
async fn test_curve_updated_at_same_slot() {
    // Ensures that if a curve gets updated at the same slot, the log will be updated too
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

        let _ = test_instance.upsert(&mut tx, updating_trade).await.unwrap();

        let count = count_all_curve_logs(&mut tx).await;
        assert_eq!(count, 1);

        let mut logs = list_all_curve_logs(&mut tx).await;
        assert_eq!(count, 1);

        let first = logs.pop().unwrap();
        assert_eq!(first.id, 1000);
        assert_eq!(first.slot, 12345);
        assert_eq!(first.virtual_base_reserves, 757214460226289i64);
        assert_eq!(first.virtual_quote_reserves, 42511074286i64);
        assert_eq!(first.progress, 39.816612);
        assert!(!first.complete);
    })
    .await;
}

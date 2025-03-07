// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::pumpfun::repo::curve::{inserting_slot_swaps, updating_slot_swaps};
use solana::model::Signature;
use solana::pumpfun::repo::CurveRepo;
use testing::pumpfun::{count_all_curves, insert_swap};
use testing::run_test_on_empty_db;

#[test_log::test(sqlx::test)]
async fn test_creates_curve() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = CurveRepo::testing();
        let test_swap = insert_swap(&mut tx, inserting_slot_swaps())
            .await
            .pop()
            .unwrap();

        let result = test_instance.upsert(&mut tx, test_swap).await.unwrap();
        assert_eq!(result.id, 1000);
        assert_eq!(result.slot, 12345);
        assert_eq!(result.virtual_base_reserves, 512561011366544);
        assert_eq!(result.virtual_quote_reserves, 62802280169);
        assert_eq!(result.progress, 70.66435);
        assert!(!result.complete);

        let count = count_all_curves(&mut tx).await;
        assert_eq!(count, 1);
    })
    .await;
}

#[test_log::test(sqlx::test)]
async fn test_update_curve() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = CurveRepo::testing();

        let inserting_swap = insert_swap(&mut tx, inserting_slot_swaps())
            .await
            .pop()
            .unwrap();

        let updating_swap = insert_swap(&mut tx, updating_slot_swaps())
            .await
            .pop()
            .unwrap();

        let _ = test_instance
            .upsert(&mut tx, inserting_swap)
            .await
            .unwrap();

        let result = test_instance.upsert(&mut tx, updating_swap).await.unwrap();
        assert_eq!(result.id, 1000);
        assert_eq!(result.slot, 23456);
        assert_eq!(result.virtual_base_reserves, 0);
        assert_eq!(result.virtual_quote_reserves, 0);
        assert_eq!(result.progress, 100);
        assert!(result.complete);

        let count = count_all_curves(&mut tx).await;
        assert_eq!(count, 1);
    })
    .await;
}

#[test_log::test(sqlx::test)]
async fn test_curve_updated_at_same_slot() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = CurveRepo::testing();

        let inserting_swap = insert_swap(&mut tx, inserting_slot_swaps())
            .await
            .pop()
            .unwrap();

        let mut updating_slot_swaps = inserting_slot_swaps();
        let swap = updating_slot_swaps.swaps.get_mut(0).unwrap();
        swap.signature = Signature::from("signaure2");
        swap.virtual_base_reserves = 757214460226289i64.into();
        swap.virtual_quote_reserves = 42511074286i64.into();

        let updating_swap = insert_swap(&mut tx, updating_slot_swaps)
            .await
            .pop()
            .unwrap();

        let _ = test_instance
            .upsert(&mut tx, inserting_swap)
            .await
            .unwrap();

        let result = test_instance.upsert(&mut tx, updating_swap).await.unwrap();

        assert_eq!(result.id, 1000);
        assert_eq!(result.slot, 12345);
        assert_eq!(result.virtual_base_reserves, 757214460226289i64);
        assert_eq!(result.virtual_quote_reserves, 42511074286i64);
        assert_eq!(result.progress, 39.816612);
        assert!(!result.complete);
    })
    .await;
}

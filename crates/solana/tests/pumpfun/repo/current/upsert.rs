// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::pumpfun::repo::current::{inserting_slot_swaps, updating_slot_swaps};
use base::model::solana::Signature;
use solana::pumpfun::repo::CurrentRepo;
use sqlx::Executor;
use testing::pumpfun::{count_all_currents, insert_swap};
use testing::run_test_on_empty_db;

#[test_log::test(sqlx::test)]
async fn test_current_without_price_and_market_cap() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = CurrentRepo::testing();

        tx.execute(
            r#"
        insert into solana.token (id, version, mint, name, symbol, decimals, supply) values
            (1000, 0, 'mint1', 'Migrating to McDonalds', 'MTM', 6, null);
        "#,
        )
        .await
        .unwrap();

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
        assert_eq!(result.price, 2);
        assert_eq!(result.price_usd, None);
        assert_eq!(result.market_cap, None);
        assert_eq!(result.market_cap, None);
        assert_eq!(result.market_cap_usd, None);

        let count = count_all_currents(&mut tx).await;
        assert_eq!(count, 1);
    })
    .await;
}

#[test_log::test(sqlx::test)]
async fn test_current_with_price_and_market_cap() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = CurrentRepo::testing();

        tx.execute(
            r#"
        insert into solana.token (id, version, mint, name, symbol, decimals, supply) values
            (1000, 0, 'mint1', 'Migrating to McDonalds', 'MTM', 6, 10000000);

        insert into solana.sol_price_1m (timestamp, usd) values
            (now(), 120);
        "#,
        )
        .await
        .unwrap();

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
        assert_eq!(result.price, 2);
        assert_eq!(result.price_usd.unwrap(), 240);
        assert_eq!(result.market_cap.unwrap(), 20_000_000);
        assert_eq!(result.market_cap_usd.unwrap(), 2_400_000_000);

        let count = count_all_currents(&mut tx).await;
        assert_eq!(count, 1);
    })
    .await;
}

#[test_log::test(sqlx::test)]
async fn test_current_without_price() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = CurrentRepo::testing();

        tx.execute(
            r#"
        insert into solana.token (id, version, mint, name, symbol, decimals, supply) values
            (1000, 0, 'mint1', 'Migrating to McDonalds', 'MTM', 6, 10000000);
        "#,
        )
        .await
        .unwrap();

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
        assert_eq!(result.price, 2);
        assert_eq!(result.price_usd, None);
        assert_eq!(result.market_cap.unwrap(), 20_000_000);
        assert_eq!(result.market_cap_usd, None);

        let count = count_all_currents(&mut tx).await;
        assert_eq!(count, 1);
    })
    .await;
}

#[test_log::test(sqlx::test)]
async fn test_update_current() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = CurrentRepo::testing();

        let inserting_swap = insert_swap(&mut tx, inserting_slot_swaps())
            .await
            .pop()
            .unwrap();

        let updating_swap = insert_swap(&mut tx, updating_slot_swaps())
            .await
            .pop()
            .unwrap();

        let _ = test_instance.upsert(&mut tx, inserting_swap).await.unwrap();

        let result = test_instance.upsert(&mut tx, updating_swap).await.unwrap();
        assert_eq!(result.id, 1000);
        assert_eq!(result.slot, 23456);
        assert_eq!(result.virtual_base_reserves, 0);
        assert_eq!(result.virtual_quote_reserves, 0);
        assert_eq!(result.progress, 100);
        assert!(result.complete);

        let count = count_all_currents(&mut tx).await;
        assert_eq!(count, 1);
    })
    .await;
}

#[test_log::test(sqlx::test)]
async fn test_current_updated_at_same_slot() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = CurrentRepo::testing();

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

        let _ = test_instance.upsert(&mut tx, inserting_swap).await.unwrap();

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

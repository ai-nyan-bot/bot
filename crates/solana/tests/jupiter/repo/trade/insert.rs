// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::Mint;
use base::test::{FailingTokenInfoLoader, SuccessfulTokenInfoLoader};
use common::model::Timestamp;
use common::repo::error::RepoError;
use solana::jupiter::repo::{SlotTrade, SlotTrades, TradeRepo};
use testing::jupiter::count_all_trades;
use testing::run_test_on_empty_db;

fn default_slot_trades() -> SlotTrades {
    SlotTrades {
        slot: 12345.into(),
        timestamp: Timestamp::now(),
        trades: vec![SlotTrade {
            input_mint: Mint::wsol(),
            input_amount: 2_000_000_000.into(),
            output_mint: "mint1".into(),
            output_amount: 1_000_000.into(),
            wallet: Default::default(),
            signature: "signature1".into(),
        }],
    }
}

#[test_log::test(sqlx::test)]
async fn test_ok() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TradeRepo::testing(SuccessfulTokenInfoLoader::default());

        let mut result = test_instance
            .insert_trades(&mut tx, default_slot_trades())
            .await
            .unwrap();
        assert_eq!(result.len(), 1);

        let result = result.pop().unwrap();
        assert_eq!(result.slot, 12345);
        assert_eq!(result.address, 1);
        assert_eq!(result.token_pair, 1000);

        assert_eq!(result.base_amount.0, 1.0);
        assert_eq!(result.quote_amount.0, 2.0);
        assert_eq!(result.price, 2.0);
        assert!(result.is_buy);

        let count = count_all_trades(&mut tx).await;
        assert_eq!(count, 1);
    })
    .await;
}

#[test_log::test(sqlx::test)]
async fn test_multiple() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TradeRepo::testing(SuccessfulTokenInfoLoader::default());

        let slot_trades = SlotTrades {
            slot: 12345.into(),
            timestamp: Timestamp::now(),
            trades: vec![
                SlotTrade {
                    input_mint: Mint::wsol(),
                    input_amount: 2_000_000_000.into(),
                    output_mint: "mint1".into(),
                    output_amount: 1_000_000.into(),
                    wallet: "7PQ3nyAJHXiFQd5c8HgRBMYLF748MQKgq3uYfTuFioHX".into(),
                    signature: "signature1".into(),
                },
                SlotTrade {
                    input_mint: "mint2".into(),
                    input_amount: 5_000_000.into(),
                    output_mint: Mint::wsol(),
                    output_amount: 6_000_000_000i64.into(),
                    wallet: "Bp65Vdx5o5THggj1ZHYsVwaKPhp999mRmAeKyFG9FVnT".into(),
                    signature: "signature2".into(),
                },
            ],
        };

        let mut result = test_instance
            .insert_trades(&mut tx, slot_trades)
            .await
            .unwrap();
        assert_eq!(result.len(), 2);

        let first = result.pop().unwrap();
        assert_eq!(first.slot, 12345);
        assert_eq!(first.address, 2);
        assert_eq!(first.token_pair, 1001);

        assert_eq!(first.base_amount.0, 5.0);
        assert_eq!(first.quote_amount.0, 6.0);
        assert_eq!(first.price, 1.2);
        assert!(!first.is_buy);

        let second = result.pop().unwrap();
        assert_eq!(second.slot, 12345);
        assert_eq!(second.address, 1);
        assert_eq!(second.token_pair, 1000);

        assert_eq!(second.base_amount.0, 1.0);
        assert_eq!(second.quote_amount.0, 2.0);
        assert_eq!(second.price, 2.0);
        assert!(second.is_buy);

        let count = count_all_trades(&mut tx).await;
        assert_eq!(count, 2);
    })
    .await;
}

#[test_log::test(sqlx::test)]
async fn test_no_trades() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TradeRepo::testing(SuccessfulTokenInfoLoader::default());

        let slot_trades = SlotTrades {
            slot: 12345.into(),
            timestamp: Timestamp::now(),
            trades: vec![],
        };

        let result = test_instance
            .insert_trades(&mut tx, slot_trades)
            .await
            .unwrap();

        assert_eq!(result.len(), 0);

        let count = count_all_trades(&mut tx).await;
        assert_eq!(count, 0);
    })
    .await;
}

#[test_log::test(sqlx::test)]
async fn test_duplicate_signature() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TradeRepo::testing(SuccessfulTokenInfoLoader::default());

        let result = test_instance
            .insert_trades(&mut tx, default_slot_trades())
            .await
            .unwrap();

        assert_eq!(result.len(), 1);

        let result = test_instance
            .insert_trades(&mut tx, default_slot_trades())
            .await
            .unwrap();

        assert_eq!(result.len(), 0);

        let count = count_all_trades(&mut tx).await;
        assert_eq!(count, 1);
    })
    .await;
}

#[test_log::test(sqlx::test)]
async fn test_fails_to_load_token_info() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TradeRepo::testing(FailingTokenInfoLoader::default());

        let result = test_instance
            .insert_trades(&mut tx, default_slot_trades())
            .await;
        assert_eq!(result.err().unwrap(), RepoError::NotFound);

        let count = count_all_trades(&mut tx).await;
        assert_eq!(count, 0);
    })
    .await;
}

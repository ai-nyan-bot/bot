// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::Mint;
use base::repo::{TokenPairRepo, TokenRepo};
use testing::run_test_on_empty_db;

#[test_log::test(sqlx::test)]
async fn test_wsol_usdt() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenPairRepo::new(TokenRepo::testing_no_token_info());

        let mut result = test_instance.list_by_ids(&mut tx, vec![1]).await.unwrap();
        assert_eq!(result.len(), 1);

        let result = result.remove(0);
        assert_eq!(result.id, 1);

        assert_eq!(result.base.id, 1);
        assert_eq!(result.base.mint, Mint::wsol());
        assert_eq!(result.base.name.unwrap(), "Wrapped SOL");
        assert_eq!(result.base.symbol.unwrap(), "WSOL");
        assert_eq!(result.base.decimals, 9);

        assert_eq!(result.quote.id, 2);
        assert_eq!(result.quote.mint, Mint::usdt());
        assert_eq!(result.quote.name.unwrap(), "USDT");
        assert_eq!(result.quote.symbol.unwrap(), "USDT");
        assert_eq!(result.quote.decimals, 6);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_wsol_usdc() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenPairRepo::new(TokenRepo::testing_no_token_info());

        let mut result = test_instance.list_by_ids(&mut tx, vec![2]).await.unwrap();
        assert_eq!(result.len(), 1);

        let result = result.remove(0);
        assert_eq!(result.id, 2);

        assert_eq!(result.base.id, 1);
        assert_eq!(result.base.mint, Mint::wsol());
        assert_eq!(result.base.name.unwrap(), "Wrapped SOL");
        assert_eq!(result.base.symbol.unwrap(), "WSOL");
        assert_eq!(result.base.decimals, 9);

        assert_eq!(result.quote.id, 3);
        assert_eq!(result.quote.mint, Mint::usdc());
        assert_eq!(result.quote.name.unwrap(), "USD Coin");
        assert_eq!(result.quote.symbol.unwrap(), "USDC");
        assert_eq!(result.quote.decimals, 6);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_usdc_usdt() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenPairRepo::new(TokenRepo::testing_no_token_info());

        let mut result = test_instance.list_by_ids(&mut tx, vec![3]).await.unwrap();
        assert_eq!(result.len(), 1);

        let result = result.remove(0);
        assert_eq!(result.id, 3);

        assert_eq!(result.base.id, 3);
        assert_eq!(result.base.mint, Mint::usdc());
        assert_eq!(result.base.name.unwrap(), "USD Coin");
        assert_eq!(result.base.symbol.unwrap(), "USDC");
        assert_eq!(result.base.decimals, 6);

        assert_eq!(result.quote.id, 2);
        assert_eq!(result.quote.mint, Mint::usdt());
        assert_eq!(result.quote.name.unwrap(), "USDT");
        assert_eq!(result.quote.symbol.unwrap(), "USDT");
        assert_eq!(result.quote.decimals, 6);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_already_in_cache() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenPairRepo::new(TokenRepo::testing_no_token_info());
        let _ = test_instance.list_by_ids(&mut tx, vec![3]).await.unwrap();

        let mut result = test_instance.list_by_ids(&mut tx, vec![3]).await.unwrap();
        assert_eq!(result.len(), 1);

        let result = result.remove(0);
        assert_eq!(result.id, 3);

        assert_eq!(result.base.id, 3);
        assert_eq!(result.base.mint, Mint::usdc());
        assert_eq!(result.base.name.unwrap(), "USD Coin");
        assert_eq!(result.base.symbol.unwrap(), "USDC");
        assert_eq!(result.base.decimals, 6);

        assert_eq!(result.quote.id, 2);
        assert_eq!(result.quote.mint, Mint::usdt());
        assert_eq!(result.quote.name.unwrap(), "USDT");
        assert_eq!(result.quote.symbol.unwrap(), "USDT");
        assert_eq!(result.quote.decimals, 6);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_not_found() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenPairRepo::new(TokenRepo::testing_no_token_info());

        let result = test_instance.list_by_ids(&mut tx, vec![23]).await.unwrap();

        assert_eq!(result.len(), 0);
    })
    .await
}

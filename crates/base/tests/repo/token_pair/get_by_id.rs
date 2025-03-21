// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::Mint;
use base::repo::{TokenPairRepo, TokenRepo};
use common::repo::error::RepoError;
use testing::run_test_on_empty_db;

#[test_log::test(sqlx::test)]
async fn test_ok() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenPairRepo::new(TokenRepo::testing_no_token_info());

        let result = test_instance.get_by_id(&mut tx, 3).await.unwrap();
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
        let _ = test_instance.get_by_id(&mut tx, 3).await.unwrap();

        let result = test_instance.get_by_id(&mut tx, 3).await.unwrap();
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

        let result = test_instance.get_by_id(&mut tx, 23).await;
        assert_eq!(result.err().unwrap(), RepoError::NotFound);
    })
    .await
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::TokenMint;
use base::service::TokenService;
use common::service::ServiceError;
use testing::run_test_with_pool;

#[test_log::test(sqlx::test)]
async fn test_ok() {
    run_test_with_pool(|pool| async move {
        let test_instance = TokenService::testing(pool.clone());
        let result = test_instance.get_pair(3).await.unwrap();

        assert_eq!(result.id, 3);

        assert_eq!(result.base.id, 3);
        assert_eq!(result.base.mint, TokenMint::usdc());
        assert_eq!(result.base.name, "USD Coin");
        assert_eq!(result.base.symbol, "USDC");
        assert_eq!(result.base.decimals, 6);

        assert_eq!(result.quote.id, 2);
        assert_eq!(result.quote.mint, TokenMint::usdt());
        assert_eq!(result.quote.name, "USDT");
        assert_eq!(result.quote.symbol, "USDT");
        assert_eq!(result.quote.decimals, 6);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_not_found() {
    run_test_with_pool(|pool| async move {
        let test_instance = TokenService::testing(pool.clone());
        let result = test_instance.get_pair(1234567).await;
        assert_eq!(
            result.err().unwrap(),
            ServiceError::not_found("TokenPair not found")
        )
    })
    .await
}

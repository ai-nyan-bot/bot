// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use base::model::Mint;
use base::repo::TokenRepo;
use base::test::{FailingTokenInfoLoader, SuccessfulTokenInfoLoader};
use common::repo::error::RepoError::NotFound;
use testing::run_test_on_empty_db;
use testing::token::count_all;

#[test_log::test(sqlx::test)]
async fn test_wsol() {
    // already exists
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenRepo::testing(SuccessfulTokenInfoLoader::default());
        let result = test_instance.get_or_populate(&mut tx, Mint::wsol()).await.unwrap();

        assert_eq!(result.id, 1);
        assert_eq!(result.mint, Mint::wsol());
        assert_eq!(result.name.unwrap(), "Wrapped SOL");
        assert_eq!(result.symbol.unwrap(), "WSOL");
        assert_eq!(result.decimals, 9);

        let count = count_all(&mut tx).await;
        assert_eq!(count, 7);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_usdt() {
    // already exists
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenRepo::testing(SuccessfulTokenInfoLoader::default());
        let result = test_instance.get_or_populate(&mut tx, Mint::usdt()).await.unwrap();

        assert_eq!(result.id, 2);
        assert_eq!(result.mint, Mint::usdt());
        assert_eq!(result.name.unwrap(), "USDT");
        assert_eq!(result.symbol.unwrap(), "USDT");
        assert_eq!(result.decimals, 6);

        let count = count_all(&mut tx).await;
        assert_eq!(count, 7);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_usdc() {
    // already exists
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenRepo::testing(SuccessfulTokenInfoLoader::default());
        let result = test_instance.get_or_populate(&mut tx, Mint::usdc()).await.unwrap();

        assert_eq!(result.id, 3);
        assert_eq!(result.mint, Mint::usdc());
        assert_eq!(result.name.unwrap(), "USD Coin");
        assert_eq!(result.symbol.unwrap(), "USDC");
        assert_eq!(result.decimals, 6);

        let count = count_all(&mut tx).await;
        assert_eq!(count, 7);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_in_db() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenRepo::testing(SuccessfulTokenInfoLoader::default());
        let _ = test_instance.get_or_populate(&mut tx, bonk_mint()).await.unwrap();

        let test_instance = TokenRepo::testing(SuccessfulTokenInfoLoader::default());
        let result = test_instance.get_or_populate(&mut tx, bonk_mint()).await.unwrap();

        assert_eq!(result.id, 1000);
        assert_eq!(result.mint, bonk_mint());
        assert_eq!(result.name.unwrap(), "1000");
        assert_eq!(result.symbol.unwrap(), "1000");

        let count = count_all(&mut tx).await;
        assert_eq!(count, 8);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_in_cache() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenRepo::testing(SuccessfulTokenInfoLoader::default());
        let _ = test_instance.get_or_populate(&mut tx, bonk_mint()).await.unwrap();

        let result = test_instance.get_or_populate(&mut tx, bonk_mint()).await.unwrap();
        assert_eq!(result.id, 1000);
        assert_eq!(result.mint, bonk_mint());
        assert_eq!(result.name.unwrap(), "1000");
        assert_eq!(result.symbol.unwrap(), "1000");

        let count = count_all(&mut tx).await;
        assert_eq!(count, 8);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_insert() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenRepo::testing(SuccessfulTokenInfoLoader::default());
        let result = test_instance.get_or_populate(&mut tx, bonk_mint()).await.unwrap();

        assert_eq!(result.id, 1000);
        assert_eq!(result.mint, bonk_mint());
        assert_eq!(result.name.unwrap(), "1000");
        assert_eq!(result.symbol.unwrap(), "1000");

        let count = count_all(&mut tx).await;
        assert_eq!(count, 8);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn unable_to_load() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenRepo::testing(FailingTokenInfoLoader {});

        let result = test_instance.get_or_populate(&mut tx, Mint::new("Does_Not_Exists")).await;
        assert_eq!(result.err().unwrap(), NotFound);

        let count = count_all(&mut tx).await;
        assert_eq!(count, 7);
    })
    .await
}

fn bonk_mint() -> Mint {
    "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263".into()
}

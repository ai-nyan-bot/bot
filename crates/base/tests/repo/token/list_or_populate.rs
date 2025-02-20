// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use base::model::TokenMint;
use base::repo::TokenRepo;
use base::test::{FailingTokenInfoLoader, SuccessfulTokenInfoLoader};
use common::repo::error::RepoError;
use std::vec;
use testing::run_test_on_empty_db;
use testing::token::count_all;

#[test_log::test(sqlx::test)]
async fn test_wsol() {
    // already exists
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenRepo::testing(SuccessfulTokenInfoLoader::default());
        let mut result = test_instance.list_or_populate(&mut tx, vec![TokenMint::wsol()]).await.unwrap();
        assert_eq!(result.len(), 1);
        let result = result.pop().unwrap();

        assert_eq!(result.id, 1);
        assert_eq!(result.mint, TokenMint::wsol());
        assert_eq!(result.name, "Wrapped SOL");
        assert_eq!(result.symbol, "WSOL");
        assert_eq!(result.decimals, 9);

        let count = count_all(&mut tx).await;
        assert_eq!(count, 3);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_usdt() {
    // already exists
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenRepo::testing(SuccessfulTokenInfoLoader::default());
        let mut result = test_instance.list_or_populate(&mut tx, vec![TokenMint::usdt()]).await.unwrap();
        assert_eq!(result.len(), 1);
        let result = result.pop().unwrap();

        assert_eq!(result.id, 2);
        assert_eq!(result.mint, TokenMint::usdt());
        assert_eq!(result.name, "USDT");
        assert_eq!(result.symbol, "USDT");
        assert_eq!(result.decimals, 6);

        let count = count_all(&mut tx).await;
        assert_eq!(count, 3);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_usdc() {
    // already exists
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenRepo::testing(SuccessfulTokenInfoLoader::default());
        let mut result = test_instance.list_or_populate(&mut tx, vec![TokenMint::usdc()]).await.unwrap();
        assert_eq!(result.len(), 1);
        let result = result.pop().unwrap();

        assert_eq!(result.id, 3);
        assert_eq!(result.mint, TokenMint::usdc());
        assert_eq!(result.name, "USD Coin");
        assert_eq!(result.symbol, "USDC");
        assert_eq!(result.decimals, 6);

        let count = count_all(&mut tx).await;
        assert_eq!(count, 3);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_already_in_db() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenRepo::testing(SuccessfulTokenInfoLoader::default());
        let result = test_instance.list_or_populate(&mut tx, vec![bonk_mint()]).await.unwrap();
        assert_eq!(result.len(), 1);

        let test_instance = TokenRepo::testing(SuccessfulTokenInfoLoader::default());
        let mut result = test_instance.list_or_populate(&mut tx, vec![bonk_mint()]).await.unwrap();
        assert_eq!(result.len(), 1);

        let result = result.remove(0);
        assert_eq!(result.id, 1000);
        assert_eq!(result.mint, bonk_mint());
        assert_eq!(result.name, "1000");
        assert_eq!(result.symbol, "1000");
        assert_eq!(result.decimals, 1000);

        let count = count_all(&mut tx).await;
        assert_eq!(count, 4);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_already_in_cache() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenRepo::testing(SuccessfulTokenInfoLoader::default());
        let mut result = test_instance.list_or_populate(&mut tx, vec![bonk_mint()]).await.unwrap();
        assert_eq!(result.len(), 1);

        let result = result.remove(0);
        assert_eq!(result.id, 1000);
        assert_eq!(result.mint, bonk_mint());
        assert_eq!(result.name, "1000");
        assert_eq!(result.symbol, "1000");
        assert_eq!(result.decimals, 1000);

        let count = count_all(&mut tx).await;
        assert_eq!(count, 4);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_insert_one() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenRepo::testing(SuccessfulTokenInfoLoader::default());

        let result = test_instance.list_or_populate(&mut tx, vec![lost_lot_of_money_mint()]).await.unwrap();
        assert_eq!(result.len(), 1);

        let result = result.first().unwrap();
        assert_eq!(result.id, 1000);
        assert_eq!(result.mint, lost_lot_of_money_mint());
        assert_eq!(result.name, "1000");
        assert_eq!(result.symbol, "1000");
        assert_eq!(result.decimals, 1000);

        let count = count_all(&mut tx).await;
        assert_eq!(count, 4);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_one_in_cache_one_in_db_one_insert() {
    run_test_on_empty_db(|mut tx| async move {
        let loader = SuccessfulTokenInfoLoader::default();

        let test_instance = TokenRepo::testing(loader.clone());
        let result = test_instance
            .list_or_populate(&mut tx, vec![lost_lot_of_money_mint(), bonk_mint()])
            .await
            .unwrap();
        assert_eq!(result.len(), 2);

        let test_instance = TokenRepo::testing(loader);
        let result = test_instance.list_or_populate(&mut tx, vec![lost_lot_of_money_mint()]).await.unwrap();
        assert_eq!(result.len(), 1);

        let result = test_instance
            .list_or_populate(
                &mut tx,
                vec![
                    TokenMint::new("Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm"),
                    bonk_mint(),
                    lost_lot_of_money_mint(),
                ],
            )
            .await
            .unwrap();
        assert_eq!(result.len(), 3);

        let first = result.first().unwrap();
        assert_eq!(first.id, 1000);
        assert_eq!(first.mint, lost_lot_of_money_mint());
        assert_eq!(first.name, "1000");
        assert_eq!(first.symbol, "1000");
        assert_eq!(first.decimals, 1000);

        let second = result.get(1).unwrap();
        assert_eq!(second.id, 1001);
        assert_eq!(second.mint, bonk_mint());
        assert_eq!(second.name, "1001");
        assert_eq!(second.symbol, "1001");
        assert_eq!(second.decimals, 1001);

        let third = result.last().unwrap();
        assert_eq!(third.id, 1002);
        assert_eq!(third.mint, TokenMint::new("Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm"));
        assert_eq!(third.name, "1002");
        assert_eq!(third.symbol, "1002");
        assert_eq!(third.decimals, 1002);

        let count = count_all(&mut tx).await;
        assert_eq!(count, 6);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_insert_many() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenRepo::testing(SuccessfulTokenInfoLoader::default());

        let result = test_instance
            .list_or_populate(
                &mut tx,
                vec![
                    "9uRJ5aGgeu2i3J98hsC5FDxd2PmRjVy9fQwNAy7fzLG3",
                    "EJRJswH9LyjhAfBWwPBvat1LQtrJYK4sVUzsea889cQt",
                    "53nHsQXkzZUp5MF1BK6Qoa48ud3aXfDFJBbe1oECPucC",
                ],
            )
            .await
            .unwrap();
        assert_eq!(result.len(), 3);

        let first = result.first().unwrap();
        assert_eq!(first.id, 1000);
        assert_eq!(first.mint, "9uRJ5aGgeu2i3J98hsC5FDxd2PmRjVy9fQwNAy7fzLG3");

        let second = result.get(1).unwrap();
        assert_eq!(second.id, 1001);
        assert_eq!(second.mint, "EJRJswH9LyjhAfBWwPBvat1LQtrJYK4sVUzsea889cQt");

        let third = result.last().unwrap();
        assert_eq!(third.id, 1002);
        assert_eq!(third.mint, "53nHsQXkzZUp5MF1BK6Qoa48ud3aXfDFJBbe1oECPucC");

        let count = count_all(&mut tx).await;
        assert_eq!(count, 6);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_unable_to_load() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenRepo::testing(FailingTokenInfoLoader {});

        let result = test_instance.list_or_populate(&mut tx, vec![TokenMint::new("Does_Not_Exists")]).await;
        assert_eq!(result.err().unwrap(), RepoError::NotFound);

        let count = count_all(&mut tx).await;
        assert_eq!(count, 3);
    })
    .await
}

fn bonk_mint() -> TokenMint {
    "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263".into()
}

fn lost_lot_of_money_mint() -> TokenMint {
    "44J6Um1tTiTbtL9nd4hU6MqDyPppeWtGr3rMFQ6ppump".into()
}

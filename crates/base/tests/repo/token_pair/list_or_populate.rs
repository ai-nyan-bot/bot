// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original MIT License Copyright (c) blockworks-foundation 2024.

use base::model::TokenMint;
use base::repo::{TokenPairRepo, TokenRepo};
use base::test::{FailingTokenInfoLoader, SuccessfulTokenInfoLoader};
use common::repo::error::RepoError;
use testing::run_test_on_empty_db;
use testing::solana::{token, token_pair};

#[test_log::test(sqlx::test)]
async fn test_wsol_usdt() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenPairRepo::testing(TokenRepo::testing(SuccessfulTokenInfoLoader::default()));

        let mut result = test_instance
            .list_or_populate_by_mints(&mut tx, vec![(TokenMint::wsol(), TokenMint::usdt())])
            .await
            .unwrap();
        assert_eq!(result.len(), 1);

        let result = result.remove(0);
        assert_eq!(result.id, 1);

        assert_eq!(result.base.id, 1);
        assert_eq!(result.base.mint, TokenMint::wsol());
        assert_eq!(result.base.name, "Wrapped SOL");
        assert_eq!(result.base.symbol, "WSOL");
        assert_eq!(result.base.decimals, 9);

        assert_eq!(result.quote.id, 2);
        assert_eq!(result.quote.mint, TokenMint::usdt());
        assert_eq!(result.quote.name, "USDT");
        assert_eq!(result.quote.symbol, "USDT");
        assert_eq!(result.quote.decimals, 6);

        let count = token_pair::count_all(&mut tx).await;
        assert_eq!(count, 3);

        let count = token::count_all(&mut tx).await;
        assert_eq!(count, 3);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_wsol_usdc() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenPairRepo::testing(TokenRepo::testing(SuccessfulTokenInfoLoader::default()));

        let mut result = test_instance
            .list_or_populate_by_mints(&mut tx, vec![(TokenMint::wsol(), TokenMint::usdc())])
            .await
            .unwrap();
        assert_eq!(result.len(), 1);

        let result = result.remove(0);
        assert_eq!(result.id, 2);

        assert_eq!(result.base.id, 1);
        assert_eq!(result.base.mint, TokenMint::wsol());
        assert_eq!(result.base.name, "Wrapped SOL");
        assert_eq!(result.base.symbol, "WSOL");
        assert_eq!(result.base.decimals, 9);

        assert_eq!(result.quote.id, 3);
        assert_eq!(result.quote.mint, TokenMint::usdc());
        assert_eq!(result.quote.name, "USD Coin");
        assert_eq!(result.quote.symbol, "USDC");
        assert_eq!(result.quote.decimals, 6);

        let count = token_pair::count_all(&mut tx).await;
        assert_eq!(count, 3);

        let count = token::count_all(&mut tx).await;
        assert_eq!(count, 3);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_usdc_usdt() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenPairRepo::testing(TokenRepo::testing(SuccessfulTokenInfoLoader::default()));

        let mut result = test_instance
            .list_or_populate_by_mints(&mut tx, vec![(TokenMint::usdc(), TokenMint::usdt())])
            .await
            .unwrap();
        assert_eq!(result.len(), 1);

        let result = result.remove(0);
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

        let count = token_pair::count_all(&mut tx).await;
        assert_eq!(count, 3);

        let count = token::count_all(&mut tx).await;
        assert_eq!(count, 3);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_already_in_db() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenPairRepo::testing(TokenRepo::testing(SuccessfulTokenInfoLoader::default()));
        let result = test_instance
            .list_or_populate_by_mints(&mut tx, vec![("Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm", TokenMint::wsol())])
            .await
            .unwrap();
        assert_eq!(result.len(), 1);

        let test_instance = TokenPairRepo::testing(TokenRepo::testing(SuccessfulTokenInfoLoader::default()));
        let mut result = test_instance
            .list_or_populate_by_mints(&mut tx, vec![("Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm", TokenMint::wsol())])
            .await
            .unwrap();
        assert_eq!(result.len(), 1);

        let result = result.remove(0);
        assert_eq!(result.id, 1000);

        assert_eq!(result.base.mint, "Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm");
        assert_eq!(result.base.name, "1000");
        assert_eq!(result.base.symbol, "1000");
        assert_eq!(result.base.decimals, 1000);

        assert_eq!(result.quote.mint, "So11111111111111111111111111111111111111112");
        assert_eq!(result.quote.name, "Wrapped SOL");
        assert_eq!(result.quote.symbol, "WSOL");
        assert_eq!(result.quote.decimals, 9);

        let count = token_pair::count_all(&mut tx).await;
        assert_eq!(count, 4);

        let count = token::count_all(&mut tx).await;
        assert_eq!(count, 4);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_already_in_cache() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenPairRepo::testing(TokenRepo::testing(SuccessfulTokenInfoLoader::default()));
        let _ = test_instance
            .list_or_populate_by_mints(&mut tx, vec![("Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm", TokenMint::wsol())])
            .await
            .unwrap();

        let mut result = test_instance
            .list_or_populate_by_mints(&mut tx, vec![("Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm", TokenMint::wsol())])
            .await
            .unwrap();
        assert_eq!(result.len(), 1);

        let result = result.remove(0);
        assert_eq!(result.id, 1000);
        assert_eq!(result.base.id, 1000);
        assert_eq!(result.base.mint, "Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm");
        assert_eq!(result.base.name, "1000");
        assert_eq!(result.base.symbol, "1000");
        assert_eq!(result.base.decimals, 1000);

        assert_eq!(result.quote.id, 1);
        assert_eq!(result.quote.mint, "So11111111111111111111111111111111111111112");
        assert_eq!(result.quote.name, "Wrapped SOL");
        assert_eq!(result.quote.symbol, "WSOL");
        assert_eq!(result.quote.decimals, 9);

        let count = token_pair::count_all(&mut tx).await;
        assert_eq!(count, 4);

        let count = token::count_all(&mut tx).await;
        assert_eq!(count, 4);
    })
    .await
}

#[test_log::test(tokio::test)]
async fn test_insert_one() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenPairRepo::testing(TokenRepo::testing(SuccessfulTokenInfoLoader::default()));

        let mut result = test_instance
            .list_or_populate_by_mints(&mut tx, vec![("Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm", TokenMint::wsol())])
            .await
            .unwrap();

        assert_eq!(result.len(), 1);

        let result = result.remove(0);
        assert_eq!(result.id, 1000);
        assert_eq!(result.base.id, 1000);
        assert_eq!(result.base.mint, "Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm");
        assert_eq!(result.base.name, "1000");
        assert_eq!(result.base.symbol, "1000");
        assert_eq!(result.base.decimals, 1000);

        assert_eq!(result.quote.id, 1);
        assert_eq!(result.quote.mint, "So11111111111111111111111111111111111111112");
        assert_eq!(result.quote.name, "Wrapped SOL");
        assert_eq!(result.quote.symbol, "WSOL");
        assert_eq!(result.quote.decimals, 9);

        let count = token_pair::count_all(&mut tx).await;
        assert_eq!(count, 4);

        let count = token::count_all(&mut tx).await;
        assert_eq!(count, 4);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_one_in_cache_one_in_db_one_insert() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenPairRepo::testing(TokenRepo::testing(SuccessfulTokenInfoLoader::default()));

        let result = test_instance
            .list_or_populate_by_mints(
                &mut tx,
                vec![
                    ("9uRJ5aGgeu2i3J98hsC5FDxd2PmRjVy9fQwNAy7fzLG3", TokenMint::usdc()),
                    ("53nHsQXkzZUp5MF1BK6Qoa48ud3aXfDFJBbe1oECPucC", TokenMint::wsol()),
                ],
            )
            .await
            .unwrap();
        assert_eq!(result.len(), 2);

        let test_instance = TokenPairRepo::testing(TokenRepo::testing(SuccessfulTokenInfoLoader::default()));
        let result = test_instance
            .list_or_populate_by_mints(&mut tx, vec![("9uRJ5aGgeu2i3J98hsC5FDxd2PmRjVy9fQwNAy7fzLG3", TokenMint::usdc())])
            .await
            .unwrap();
        assert_eq!(result.len(), 1);

        let result = test_instance
            .list_or_populate_by_mints(
                &mut tx,
                vec![
                    ("Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm", TokenMint::usdt()),
                    ("9uRJ5aGgeu2i3J98hsC5FDxd2PmRjVy9fQwNAy7fzLG3", TokenMint::usdc()),
                    ("53nHsQXkzZUp5MF1BK6Qoa48ud3aXfDFJBbe1oECPucC", TokenMint::wsol()),
                ],
            )
            .await
            .unwrap();
        assert_eq!(result.len(), 3);

        let first = result.first().unwrap();
        assert_eq!(first.id, 1000);
        assert_eq!(first.base.mint, "9uRJ5aGgeu2i3J98hsC5FDxd2PmRjVy9fQwNAy7fzLG3");
        assert_eq!(first.quote.mint, TokenMint::usdc());

        let second = result.get(1).unwrap();
        assert_eq!(second.id, 1001);
        assert_eq!(second.base.mint, "53nHsQXkzZUp5MF1BK6Qoa48ud3aXfDFJBbe1oECPucC");
        assert_eq!(second.quote.mint, TokenMint::wsol());

        let third = result.last().unwrap();
        assert_eq!(third.id, 1002);
        assert_eq!(third.base.mint, "Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm");
        assert_eq!(third.quote.mint, TokenMint::usdt());

        let count = token_pair::count_all(&mut tx).await;
        assert_eq!(count, 6);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_insert_many() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenPairRepo::testing(TokenRepo::testing(SuccessfulTokenInfoLoader::default()));

        let result = test_instance
            .list_or_populate_by_mints(
                &mut tx,
                vec![
                    ("9uRJ5aGgeu2i3J98hsC5FDxd2PmRjVy9fQwNAy7fzLG3", TokenMint::wsol()),
                    ("EJRJswH9LyjhAfBWwPBvat1LQtrJYK4sVUzsea889cQt", TokenMint::usdt()),
                    ("53nHsQXkzZUp5MF1BK6Qoa48ud3aXfDFJBbe1oECPucC", TokenMint::wsol()),
                ],
            )
            .await
            .unwrap();
        assert_eq!(result.len(), 3);

        let first = result.first().unwrap();
        assert_eq!(first.id, 1000);
        assert_eq!(first.base.mint, "9uRJ5aGgeu2i3J98hsC5FDxd2PmRjVy9fQwNAy7fzLG3");
        assert_eq!(first.quote.mint, TokenMint::wsol());

        let second = result.get(1).unwrap();
        assert_eq!(second.id, 1001);
        assert_eq!(second.base.mint, "EJRJswH9LyjhAfBWwPBvat1LQtrJYK4sVUzsea889cQt");
        assert_eq!(second.quote.mint, TokenMint::usdt());

        let third = result.last().unwrap();
        assert_eq!(third.id, 1002);
        assert_eq!(third.base.mint, "53nHsQXkzZUp5MF1BK6Qoa48ud3aXfDFJBbe1oECPucC");
        assert_eq!(third.quote.mint, TokenMint::wsol());

        let count = token_pair::count_all(&mut tx).await;
        assert_eq!(count, 6);

        let count = token::count_all(&mut tx).await;
        assert_eq!(count, 6);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_unable_to_load() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenPairRepo::testing(TokenRepo::testing(FailingTokenInfoLoader {}));

        let result = test_instance
            .list_or_populate_by_mints(&mut tx, vec![(TokenMint::new("Does_Not_Exists"), TokenMint::wsol())])
            .await;
        assert_eq!(result.err().unwrap(), RepoError::NotFound);

        let count = token_pair::count_all(&mut tx).await;
        assert_eq!(count, 3);

        let count = token::count_all(&mut tx).await;
        assert_eq!(count, 3);
    })
    .await
}

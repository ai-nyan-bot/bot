// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use base::model::Mint;
use base::repo::{TokenPairRepo, TokenRepo};
use base::test::{FailingTokenInfoLoader, SuccessfulTokenInfoLoader};
use common::repo::error::RepoError;
use testing::{run_test_on_empty_db, token, token_pair};
#[test_log::test(sqlx::test)]
async fn test_wsol_usdt() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenPairRepo::testing(TokenRepo::testing(SuccessfulTokenInfoLoader::default()));

        let mut result = test_instance
            .list_or_populate(&mut tx, vec![(Mint::wsol(), Mint::usdt())])
            .await
            .unwrap();
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

        let count = token_pair::count_all(&mut tx).await;
        assert_eq!(count, 3);

        let count = token::count_all(&mut tx).await;
        assert_eq!(count, 7);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_wsol_usdc() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenPairRepo::testing(TokenRepo::testing(SuccessfulTokenInfoLoader::default()));

        let mut result = test_instance
            .list_or_populate(&mut tx, vec![(Mint::wsol(), Mint::usdc())])
            .await
            .unwrap();
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

        let count = token_pair::count_all(&mut tx).await;
        assert_eq!(count, 3);

        let count = token::count_all(&mut tx).await;
        assert_eq!(count, 7);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_usdc_usdt() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenPairRepo::testing(TokenRepo::testing(SuccessfulTokenInfoLoader::default()));

        let mut result = test_instance
            .list_or_populate(&mut tx, vec![(Mint::usdc(), Mint::usdt())])
            .await
            .unwrap();
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

        let count = token_pair::count_all(&mut tx).await;
        assert_eq!(count, 3);

        let count = token::count_all(&mut tx).await;
        assert_eq!(count, 7);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_already_in_db() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenPairRepo::testing(TokenRepo::testing(SuccessfulTokenInfoLoader::default()));
        let result = test_instance
            .list_or_populate(&mut tx, vec![("Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm", Mint::wsol())])
            .await
            .unwrap();
        assert_eq!(result.len(), 1);

        let test_instance = TokenPairRepo::testing(TokenRepo::testing(SuccessfulTokenInfoLoader::default()));
        let mut result = test_instance
            .list_or_populate(&mut tx, vec![("Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm", Mint::wsol())])
            .await
            .unwrap();
        assert_eq!(result.len(), 1);

        let result = result.remove(0);
        assert_eq!(result.id, 1000);

        assert_eq!(result.base.mint, "Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm");
        assert_eq!(result.base.name.unwrap(), "1000");
        assert_eq!(result.base.symbol.unwrap(), "1000");

        assert_eq!(result.quote.mint, "So11111111111111111111111111111111111111112");
        assert_eq!(result.quote.name.unwrap(), "Wrapped SOL");
        assert_eq!(result.quote.symbol.unwrap(), "WSOL");

        let count = token_pair::count_all(&mut tx).await;
        assert_eq!(count, 4);

        let count = token::count_all(&mut tx).await;
        assert_eq!(count, 8);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_already_in_cache() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenPairRepo::testing(TokenRepo::testing(SuccessfulTokenInfoLoader::default()));
        let _ = test_instance
            .list_or_populate(&mut tx, vec![("Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm", Mint::wsol())])
            .await
            .unwrap();

        let mut result = test_instance
            .list_or_populate(&mut tx, vec![("Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm", Mint::wsol())])
            .await
            .unwrap();
        assert_eq!(result.len(), 1);

        let result = result.remove(0);
        assert_eq!(result.id, 1000);
        assert_eq!(result.base.id, 1000);
        assert_eq!(result.base.mint, "Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm");
        assert_eq!(result.base.name.unwrap(), "1000");
        assert_eq!(result.base.symbol.unwrap(), "1000");

        assert_eq!(result.quote.id, 1);
        assert_eq!(result.quote.mint, "So11111111111111111111111111111111111111112");
        assert_eq!(result.quote.name.unwrap(), "Wrapped SOL");
        assert_eq!(result.quote.symbol.unwrap(), "WSOL");

        let count = token_pair::count_all(&mut tx).await;
        assert_eq!(count, 4);

        let count = token::count_all(&mut tx).await;
        assert_eq!(count, 8);
    })
    .await
}

#[test_log::test(tokio::test)]
async fn test_insert_one() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenPairRepo::testing(TokenRepo::testing(SuccessfulTokenInfoLoader::default()));

        let mut result = test_instance
            .list_or_populate(&mut tx, vec![("Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm", Mint::wsol())])
            .await
            .unwrap();

        assert_eq!(result.len(), 1);

        let result = result.remove(0);
        assert_eq!(result.id, 1000);
        assert_eq!(result.base.id, 1000);
        assert_eq!(result.base.mint, "Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm");
        assert_eq!(result.base.name.unwrap(), "1000");
        assert_eq!(result.base.symbol.unwrap(), "1000");

        assert_eq!(result.quote.id, 1);
        assert_eq!(result.quote.mint, "So11111111111111111111111111111111111111112");
        assert_eq!(result.quote.name.unwrap(), "Wrapped SOL");
        assert_eq!(result.quote.symbol.unwrap(), "WSOL");

        let count = token_pair::count_all(&mut tx).await;
        assert_eq!(count, 4);

        let count = token::count_all(&mut tx).await;
        assert_eq!(count, 8);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_one_in_cache_one_in_db_one_insert() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenPairRepo::testing(TokenRepo::testing(SuccessfulTokenInfoLoader::default()));

        let result = test_instance
            .list_or_populate(
                &mut tx,
                vec![
					("9uRJ5aGgeu2i3J98hsC5FDxd2PmRjVy9fQwNAy7fzLG3", Mint::usdc()),
					("53nHsQXkzZUp5MF1BK6Qoa48ud3aXfDFJBbe1oECPucC", Mint::wsol()),
                ],
            )
            .await
            .unwrap();
        assert_eq!(result.len(), 2);

        let test_instance = TokenPairRepo::testing(TokenRepo::testing(SuccessfulTokenInfoLoader::default()));
        let result = test_instance
            .list_or_populate(&mut tx, vec![("9uRJ5aGgeu2i3J98hsC5FDxd2PmRjVy9fQwNAy7fzLG3", Mint::usdc())])
            .await
            .unwrap();
        assert_eq!(result.len(), 1);

        let result = test_instance
            .list_or_populate(
                &mut tx,
                vec![
					("Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm", Mint::usdt()),
					("9uRJ5aGgeu2i3J98hsC5FDxd2PmRjVy9fQwNAy7fzLG3", Mint::usdc()),
					("53nHsQXkzZUp5MF1BK6Qoa48ud3aXfDFJBbe1oECPucC", Mint::wsol()),
                ],
            )
            .await
            .unwrap();
        assert_eq!(result.len(), 3);

        let first = result.first().unwrap();
        assert_eq!(first.id, 1000);
        assert_eq!(first.base.mint, "9uRJ5aGgeu2i3J98hsC5FDxd2PmRjVy9fQwNAy7fzLG3");
        assert_eq!(first.quote.mint, Mint::usdc());

        let second = result.get(1).unwrap();
        assert_eq!(second.id, 1001);
        assert_eq!(second.base.mint, "53nHsQXkzZUp5MF1BK6Qoa48ud3aXfDFJBbe1oECPucC");
        assert_eq!(second.quote.mint, Mint::wsol());

        let third = result.last().unwrap();
        assert_eq!(third.id, 1002);
        assert_eq!(third.base.mint, "Av6qVigkb7USQyPXJkUvAEm4f599WTRvd75PUWBA9eNm");
        assert_eq!(third.quote.mint, Mint::usdt());

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
            .list_or_populate(
                &mut tx,
                vec![
					("9uRJ5aGgeu2i3J98hsC5FDxd2PmRjVy9fQwNAy7fzLG3", Mint::wsol()),
					("EJRJswH9LyjhAfBWwPBvat1LQtrJYK4sVUzsea889cQt", Mint::usdt()),
					("53nHsQXkzZUp5MF1BK6Qoa48ud3aXfDFJBbe1oECPucC", Mint::wsol()),
                ],
            )
            .await
            .unwrap();
        assert_eq!(result.len(), 3);

        let first = result.first().unwrap();
        assert_eq!(first.id, 1000);
        assert_eq!(first.base.mint, "9uRJ5aGgeu2i3J98hsC5FDxd2PmRjVy9fQwNAy7fzLG3");
        assert_eq!(first.quote.mint, Mint::wsol());

        let second = result.get(1).unwrap();
        assert_eq!(second.id, 1001);
        assert_eq!(second.base.mint, "EJRJswH9LyjhAfBWwPBvat1LQtrJYK4sVUzsea889cQt");
        assert_eq!(second.quote.mint, Mint::usdt());

        let third = result.last().unwrap();
        assert_eq!(third.id, 1002);
        assert_eq!(third.base.mint, "53nHsQXkzZUp5MF1BK6Qoa48ud3aXfDFJBbe1oECPucC");
        assert_eq!(third.quote.mint, Mint::wsol());

        let count = token_pair::count_all(&mut tx).await;
        assert_eq!(count, 6);

        let count = token::count_all(&mut tx).await;
        assert_eq!(count, 10);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_unable_to_load() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = TokenPairRepo::testing(TokenRepo::testing(FailingTokenInfoLoader {}));

        let result = test_instance
            .list_or_populate(&mut tx, vec![(Mint::new("Does_Not_Exists"), Mint::wsol())])
            .await;
        assert_eq!(result.err().unwrap(), RepoError::NotFound);

        let count = token_pair::count_all(&mut tx).await;
        assert_eq!(count, 3);

        let count = token::count_all(&mut tx).await;
        assert_eq!(count, 7);
    })
    .await
}

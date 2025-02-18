// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::Condition::Compare;
use base::model::Field::Price;
use base::model::Operator::MoreThan;
use base::model::{Action, Sequence, TokenMint, Value};
use base::repo::{InvocationCreateCmd, InvocationRepo};
use common::repo::error::RepoError;
use sqlx::Acquire;
use testing::invocation::count_all;
use testing::rule::create_rule_for_test_user;
use testing::run_test;
use testing::token_pair::get_or_create_token_pair;
use testing::user::get_or_create_test_user;

#[test_log::test(sqlx::test)]
async fn test_create() {
    run_test(|mut tx| async move {
        let user = get_or_create_test_user(&mut tx).await;
        let rule = create_rule_for_test_user(&mut tx, "MoneyMaker").await;
        let token_pair = get_or_create_token_pair(&mut tx, TokenMint::usdc(), TokenMint::usdt()).await;

        let test_instance = InvocationRepo::new();
        let result = test_instance
            .create(
                &mut tx,
                InvocationCreateCmd {
                    user: user.id,
                    rule: rule.id,
                    token_pair: token_pair.id,
                    next: Some(Sequence {
                        condition: Compare {
                            field: Price,
                            operator: MoreThan,
                            value: Value::Percent(23.0),
                            timeframe: None,
                        },
                        action: Action::Buy,
                    }),
                },
            )
            .await
            .unwrap();

        assert_eq!(result.id, 1);
        assert_eq!(result.user, 1);
        assert_eq!(result.rule, 4);
        assert_eq!(result.token_pair, 3);

        let next = result.next.unwrap();
        assert_eq!(
            next.condition,
            Compare {
                field: Price,
                operator: MoreThan,
                value: Value::Percent(23.0),
                timeframe: None,
            }
        );
        assert_eq!(next.action, Action::Buy);

        let count = count_all(&mut tx).await;
        assert_eq!(count, 1)
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_next_is_none() {
    run_test(|mut tx| async move {
        let user = get_or_create_test_user(&mut tx).await;
        let rule = create_rule_for_test_user(&mut tx, "MoneyMaker").await;
        let token_pair = get_or_create_token_pair(&mut tx, TokenMint::usdc(), TokenMint::usdt()).await;

        let test_instance = InvocationRepo::new();
        let result = test_instance
            .create(
                &mut tx,
                InvocationCreateCmd {
                    user: user.id,
                    rule: rule.id,
                    token_pair: token_pair.id,
                    next: None,
                },
            )
            .await
            .unwrap();

        assert_eq!(result.id, 1);
        assert_eq!(result.user, 1);
        assert_eq!(result.rule, 4);
        assert_eq!(result.token_pair, 3);
        assert_eq!(result.next, None);

        let count = count_all(&mut tx).await;
        assert_eq!(count, 1)
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_invocation_requires_existing_user() {
    run_test(|mut tx| async move {
        let test_instance = InvocationRepo::new();

        let rule = create_rule_for_test_user(&mut tx, "MoneyMaker").await;
        let token_pair = get_or_create_token_pair(&mut tx, TokenMint::usdc(), TokenMint::usdt()).await;

        let result = test_instance
            .create(
                &mut tx.begin().await.unwrap(),
                InvocationCreateCmd {
                    user: 1234567.into(),
                    rule: rule.id,
                    token_pair: token_pair.id,
                    next: Some(Sequence {
                        condition: Compare {
                            field: Price,
                            operator: MoreThan,
                            value: Value::Percent(23.0),
                            timeframe: None,
                        },
                        action: Action::Buy,
                    }),
                },
            )
            .await;
        assert_eq!(result.err(), Some(RepoError::ForeignKeyViolation));

        let count = count_all(&mut tx).await;
        assert_eq!(count, 0)
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_invocation_requires_existing_rule() {
    run_test(|mut tx| async move {
        let test_instance = InvocationRepo::new();

        let user = get_or_create_test_user(&mut tx).await;
        let token_pair = get_or_create_token_pair(&mut tx, TokenMint::usdc(), TokenMint::usdt()).await;

        let result = test_instance
            .create(
                &mut tx.begin().await.unwrap(),
                InvocationCreateCmd {
                    user: user.id,
                    rule: 12345678.into(),
                    token_pair: token_pair.id,
                    next: Some(Sequence {
                        condition: Compare {
                            field: Price,
                            operator: MoreThan,
                            value: Value::Percent(23.0),
                            timeframe: None,
                        },
                        action: Action::Buy,
                    }),
                },
            )
            .await;
        assert_eq!(result.err(), Some(RepoError::ForeignKeyViolation));

        let count = count_all(&mut tx).await;
        assert_eq!(count, 0)
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_invocation_requires_existing_token_pair() {
    run_test(|mut tx| async move {
        let test_instance = InvocationRepo::new();

        let user = get_or_create_test_user(&mut tx).await;
        let rule = create_rule_for_test_user(&mut tx, "MoneyMaker").await;

        let result = test_instance
            .create(
                &mut tx.begin().await.unwrap(),
                InvocationCreateCmd {
                    user: user.id,
                    rule: rule.id,
                    token_pair: 12345679.into(),
                    next: Some(Sequence {
                        condition: Compare {
                            field: Price,
                            operator: MoreThan,
                            value: Value::Percent(23.0),
                            timeframe: None,
                        },
                        action: Action::Buy,
                    }),
                },
            )
            .await;
        assert_eq!(result.err(), Some(RepoError::ForeignKeyViolation));

        let count = count_all(&mut tx).await;
        assert_eq!(count, 0)
    })
    .await
}

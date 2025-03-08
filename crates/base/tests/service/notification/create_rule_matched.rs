// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{RuleId, TelegramActionButtonConfig, TokenPairId, Value, Venue};
use base::repo::NotificationRepo;
use base::service::{NotificationRuleMatched, NotificationService};
use testing::notification::count_all;
use testing::run_test_with_pool_on_empty_db;
use testing::user::get_or_create_test_user;

#[test_log::test(sqlx::test)]
async fn test_ok() {
    run_test_with_pool_on_empty_db(|pool| async move {
        let mut tx = pool.begin().await.unwrap();
        let test_user = get_or_create_test_user(&mut tx).await;
        tx.commit().await.unwrap();

        let test_instance = NotificationService::new(pool.clone(), NotificationRepo::new());
        test_instance
            .create_rule_matched(NotificationRuleMatched::Telegram {
                user: test_user.id,
                token_pair: 234.into(),
                rule: 456.into(),
                venue: Venue::PumpFun,
                buttons: vec![
					TelegramActionButtonConfig::Nothing,
					TelegramActionButtonConfig::Buy {
                        value: Value::sol(12),
                    },
					TelegramActionButtonConfig::Sell {
                        value: Value::percent(3.4),
                    },
                ],
            })
            .await
            .unwrap();

        let mut tx = pool.begin().await.unwrap();
        let count = count_all(&mut tx).await;
        assert_eq!(count, 1);

        let _ = test_instance
            .pop(1, |notification| async move {
                assert_eq!(notification.user, 1);
                assert_eq!(notification.payload("rule"), Some(RuleId(456)));
                assert_eq!(notification.payload("token_pair"), Some(TokenPairId(234)));
                assert_eq!(notification.payload("venue"), Some(Venue::PumpFun));
                assert_eq!(
                    notification.payload("button_0"),
                    Some(TelegramActionButtonConfig::Nothing)
                );
                assert_eq!(
                    notification.payload("button_1"),
                    Some(TelegramActionButtonConfig::Buy {
                        value: Value::sol(12)
                    })
                );
                assert_eq!(
                    notification.payload("button_2"),
                    Some(TelegramActionButtonConfig::Sell {
                        value: Value::percent(3.4)
                    })
                );
                assert_eq!(
                    notification.payload("button_3"),
                    Some(TelegramActionButtonConfig::Nothing)
                );
                assert_eq!(
                    notification.payload("button_4"),
                    Some(TelegramActionButtonConfig::Nothing)
                );
                assert_eq!(
                    notification.payload("button_5"),
                    Some(TelegramActionButtonConfig::Nothing)
                );
                Ok(())
            })
            .await
            .unwrap();
    })
    .await
}

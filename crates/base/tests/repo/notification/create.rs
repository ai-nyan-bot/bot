// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{NotificationChannel, NotificationType, NotificationPayload};
use base::repo::{NotificationCreateCmd, NotificationRepo};
use common::repo::error::RepoError;
use serde_json::Map;
use sqlx::types::JsonValue;
use sqlx::Acquire;
use testing::user::get_or_create_test_user;
use testing::run_test_on_empty_db;

#[test_log::test(sqlx::test)]
async fn test_create() {
    run_test_on_empty_db(|mut tx| async move {
        let user = get_or_create_test_user(&mut tx).await;
        let test_instance = NotificationRepo::new();
        let result = test_instance
            .create(
                &mut tx,
                NotificationCreateCmd {
                    user: user.id,
                    ty: NotificationType::RuleMatched,
                    channel: NotificationChannel::Telegram,
                    payload: NotificationPayload(JsonValue::Object({
                        let mut map = Map::new();
                        map.insert("answer".to_string(), JsonValue::String("42".to_string()));
                        map
                    })),
                },
            )
            .await
            .unwrap();

        assert_eq!(result.id, 1);
        assert_eq!(result.user, 1);
        assert_eq!(result.channel, NotificationChannel::Telegram);
        assert_eq!(result.ty, NotificationType::RuleMatched);
        assert_eq!(&result.payload.0.to_string(), "{\"answer\":\"42\"}");

        let count = testing::notification::count_all(&mut tx).await;
        assert_eq!(count, 1)
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_notification_requires_existing_user() {
    run_test_on_empty_db(|mut tx| async move {
        let test_instance = NotificationRepo::new();
        let result = test_instance
            .create(
                &mut tx.begin().await.unwrap(),
                NotificationCreateCmd {
                    user: 1.into(),
                    ty: NotificationType::RuleMatched,
                    channel: NotificationChannel::Telegram,
                    payload: NotificationPayload(JsonValue::Object({
                        let mut map = Map::new();
                        map.insert("answer".to_string(), JsonValue::String("42".to_string()));
                        map
                    })),
                },
            )
            .await;

        assert_eq!(result.err(), Some(RepoError::ForeignKeyViolation));

        let count = testing::rule::count_all(&mut tx).await;
        assert_eq!(count, 0)
    })
    .await
}

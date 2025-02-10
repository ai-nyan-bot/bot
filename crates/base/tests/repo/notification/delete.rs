// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{NotificationChannel, NotificationKind, NotificationPayload};
use base::repo::NotificationRepo;
use serde_json::Map;
use sqlx::types::JsonValue;
use testing::base::notification::{count_all, create_notification_for_test_user};
use testing::run_test;

#[test_log::test(sqlx::test)]
async fn test_no_notifications() {
    run_test(|mut tx| async move {
        let test_instance = NotificationRepo::new();

        let result = test_instance.delete(&mut tx, 10).await.unwrap();
        assert_eq!(result.len(), 0);

        let count = count_all(&mut tx).await;
        assert_eq!(count, 0)
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_one() {
    run_test(|mut tx| async move {
        let test_instance = NotificationRepo::new();

        create_notification_for_test_user(
            &mut tx,
            NotificationKind::ConditionMet,
            NotificationPayload(JsonValue::Object({
                let mut map = Map::new();
                map.insert("value".to_string(), JsonValue::String("1".to_string()));
                map
            })),
        )
        .await
        .unwrap();

        create_notification_for_test_user(
            &mut tx,
            NotificationKind::ConditionMet,
            NotificationPayload(JsonValue::Object({
                let mut map = Map::new();
                map.insert("value".to_string(), JsonValue::String("2".to_string()));
                map
            })),
        )
        .await
        .unwrap();

        create_notification_for_test_user(
            &mut tx,
            NotificationKind::ConditionMet,
            NotificationPayload(JsonValue::Object({
                let mut map = Map::new();
                map.insert("value".to_string(), JsonValue::String("3".to_string()));
                map
            })),
        )
        .await
        .unwrap();

        let result = test_instance.delete(&mut tx, 1).await.unwrap();
        assert_eq!(result.len(), 1);

        let notification = &result[0];
        assert_eq!(notification.id, 1);
        assert_eq!(notification.user, 1);
        assert_eq!(notification.channel, NotificationChannel::Telegram);
        assert_eq!(notification.kind, NotificationKind::ConditionMet);
        assert_eq!(&notification.payload.0.to_string(), "{\"value\":\"1\"}");

        let count = count_all(&mut tx).await;
        assert_eq!(count, 2);

        let notification = test_instance.get_by_id(&mut tx, 2).await.unwrap();
        assert_eq!(notification.payload.0.to_string(), "{\"value\":\"2\"}");

        let notification = test_instance.get_by_id(&mut tx, 3).await.unwrap();
        assert_eq!(notification.payload.0.to_string(), "{\"value\":\"3\"}");
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_many() {
    run_test(|mut tx| async move {
        let test_instance = NotificationRepo::new();

        create_notification_for_test_user(
            &mut tx,
            NotificationKind::ConditionMet,
            NotificationPayload(JsonValue::Object({
                let mut map = Map::new();
                map.insert("value".to_string(), JsonValue::String("1".to_string()));
                map
            })),
        )
        .await
        .unwrap();

        create_notification_for_test_user(
            &mut tx,
            NotificationKind::ConditionMet,
            NotificationPayload(JsonValue::Object({
                let mut map = Map::new();
                map.insert("value".to_string(), JsonValue::String("2".to_string()));
                map
            })),
        )
        .await
        .unwrap();

        create_notification_for_test_user(
            &mut tx,
            NotificationKind::ConditionMet,
            NotificationPayload(JsonValue::Object({
                let mut map = Map::new();
                map.insert("value".to_string(), JsonValue::String("3".to_string()));
                map
            })),
        )
        .await
        .unwrap();

        create_notification_for_test_user(
            &mut tx,
            NotificationKind::ConditionMet,
            NotificationPayload(JsonValue::Object({
                let mut map = Map::new();
                map.insert("value".to_string(), JsonValue::String("4".to_string()));
                map
            })),
        )
        .await
        .unwrap();

        let result = test_instance.delete(&mut tx, 3).await.unwrap();
        assert_eq!(result.len(), 3);

        assert_eq!(result[0].payload.0.to_string(), "{\"value\":\"1\"}");
        assert_eq!(result[1].payload.0.to_string(), "{\"value\":\"2\"}");
        assert_eq!(result[2].payload.0.to_string(), "{\"value\":\"3\"}");

        let count = count_all(&mut tx).await;
        assert_eq!(count, 1);

        let notification = test_instance.get_by_id(&mut tx, 4).await.unwrap();
        assert_eq!(notification.payload.0.to_string(), "{\"value\":\"4\"}");
    })
    .await
}

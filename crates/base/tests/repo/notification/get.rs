// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod get_by_id {
    use base::model::{NotificationChannel, NotificationType, NotificationPayload};
    use base::repo::NotificationRepo;
    use common::repo::error::RepoError;
    use serde_json::Map;
    use sqlx::types::JsonValue;
    use testing::notification::create_notification_for_test_user;
    use testing::run_test;

    #[test_log::test(sqlx::test)]
    async fn test_ok() {
        run_test(|mut tx| async move {
            let test_instance = NotificationRepo::new();

            let notification = create_notification_for_test_user(
                &mut tx,
                NotificationType::RuleMatched,
                NotificationPayload(JsonValue::Object({
                    let mut map = Map::new();
                    map.insert("answer".to_string(), JsonValue::String("42".to_string()));
                    map
                })),
            )
            .await
            .unwrap();

            let result = test_instance.get_by_id(&mut tx, notification.id).await.unwrap();
            assert_eq!(result.id, 1);
            assert_eq!(result.user, 1);
            assert_eq!(result.channel, NotificationChannel::Telegram);
            assert_eq!(result.ty, NotificationType::RuleMatched);
            assert_eq!(&result.payload.0.to_string(), "{\"answer\":\"42\"}");
        })
        .await
    }

    #[test_log::test(sqlx::test)]
    async fn test_not_found() {
        run_test(|mut tx| async move {
            let test_instance = NotificationRepo::new();

            let result = test_instance.get_by_id(&mut tx, 44444).await;
            assert_eq!(result.err(), Some(RepoError::NotFound));
        })
        .await
    }
}

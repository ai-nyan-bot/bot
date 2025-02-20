// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{NotificationChannel, NotificationKind, NotificationPayload};
use base::repo::NotificationRepo;
use base::service::{NotificationError, NotificationService};
use common::service::ServiceError;
use serde_json::Map;
use sqlx::types::JsonValue;
use std::fmt::Display;
use testing::notification::{count_all, create_notification_for_another_user, create_notification_for_test_user};
use testing::run_test_with_pool_on_empty_db;
use JsonValue::Object;
use NotificationChannel::Telegram;
use NotificationKind::ConditionMet;

#[test_log::test(sqlx::test)]
async fn test_nothing_to_delete() {
	run_test_with_pool_on_empty_db(|pool| async move {
		let test_instance = NotificationService::new(pool.clone(), NotificationRepo::new());

		let result = test_instance
			.pop(10, |notification| async move { Ok(notification.id) })
			.await
			.unwrap();

		assert_eq!(result.len(), 0);

		let mut tx = pool.begin().await.unwrap();
		let count = count_all(&mut tx).await;
		assert_eq!(count, 0);
	})
		.await
}

#[test_log::test(sqlx::test)]
async fn test_one() {
	run_test_with_pool_on_empty_db(|pool| async move {
		let mut tx = pool.begin().await.unwrap();

		create_notification_for_test_user(
			&mut tx,
			ConditionMet,
			NotificationPayload(Object({
				let mut map = Map::new();
				map.insert("value".to_string(), JsonValue::String("1".to_string()));
				map
			})),
		)
			.await
			.unwrap();

		create_notification_for_another_user(
			&mut tx,
			ConditionMet,
			NotificationPayload(Object({
				let mut map = Map::new();
				map.insert("value".to_string(), JsonValue::String("2".to_string()));
				map
			})),
		)
			.await
			.unwrap();

		let _ = tx.commit().await.unwrap();

		let test_instance = NotificationService::new(pool.clone(), NotificationRepo::new());

		let result = test_instance
			.pop(1, |notification| async move { Ok(notification) })
			.await
			.unwrap();
		assert_eq!(result.len(), 1);

		let notification = &result[0];
		assert_eq!(notification.id, 1);
		assert_eq!(notification.user, 1);
		assert_eq!(notification.channel, Telegram);
		assert_eq!(notification.kind, ConditionMet);
		assert_eq!(&notification.payload.0.to_string(), "{\"value\":\"1\"}");

		let mut tx = pool.begin().await.unwrap();
		let count = count_all(&mut tx).await;
		assert_eq!(count, 1);
	})
		.await
}

#[test_log::test(sqlx::test)]
async fn test_many() {
	run_test_with_pool_on_empty_db(|pool| async move {
		let mut tx = pool.begin().await.unwrap();

		create_notification_for_test_user(
			&mut tx,
			NotificationKind::ConditionMet,
			NotificationPayload(Object({
				let mut map = Map::new();
				map.insert("value".to_string(), JsonValue::String("1".to_string()));
				map
			})),
		)
			.await
			.unwrap();

		create_notification_for_another_user(
			&mut tx,
			NotificationKind::ConditionMet,
			NotificationPayload(Object({
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
			NotificationPayload(Object({
				let mut map = Map::new();
				map.insert("value".to_string(), JsonValue::String("3".to_string()));
				map
			})),
		)
			.await
			.unwrap();

		let _ = tx.commit().await.unwrap();

		let test_instance = NotificationService::new(pool.clone(), NotificationRepo::new());

		let result = test_instance
			.pop(2, |notification| async move { Ok(notification) })
			.await
			.unwrap();

		assert_eq!(result.len(), 2);

		let first = &result[0];
		assert_eq!(first.id, 1);
		assert_eq!(first.user, 1);
		assert_eq!(first.channel, NotificationChannel::Telegram);
		assert_eq!(first.kind, NotificationKind::ConditionMet);
		assert_eq!(&first.payload.0.to_string(), "{\"value\":\"1\"}");

		let second = &result[1];
		assert_eq!(second.id, 2);
		assert_eq!(second.user, 2);
		assert_eq!(second.channel, NotificationChannel::Telegram);
		assert_eq!(second.kind, NotificationKind::ConditionMet);
		assert_eq!(&second.payload.0.to_string(), "{\"value\":\"2\"}");

		let mut tx = pool.begin().await.unwrap();
		let count = count_all(&mut tx).await;
		assert_eq!(count, 1);
	})
		.await
}

#[test_log::test(sqlx::test)]
async fn test_rolls_back_if_any_fails() {
	run_test_with_pool_on_empty_db(|pool| async move {
		let mut tx = pool.begin().await.unwrap();

		create_notification_for_test_user(
			&mut tx,
			ConditionMet,
			NotificationPayload(Object({
				let mut map = Map::new();
				map.insert("value".to_string(), JsonValue::String("1".to_string()));
				map
			})),
		)
			.await
			.unwrap();

		create_notification_for_another_user(
			&mut tx,
			ConditionMet,
			NotificationPayload(Object({
				let mut map = Map::new();
				map.insert("value".to_string(), JsonValue::String("2".to_string()));
				map
			})),
		)
			.await
			.unwrap();

		create_notification_for_test_user(
			&mut tx,
			ConditionMet,
			NotificationPayload(Object({
				let mut map = Map::new();
				map.insert("value".to_string(), JsonValue::String("3".to_string()));
				map
			})),
		)
			.await
			.unwrap();

		let _ = tx.commit().await.unwrap();

		let test_instance = NotificationService::new(pool.clone(), NotificationRepo::new());

		let result = test_instance
			.pop(10, |notification| async move {
				if notification.id == 2 {
					return Err(NotificationError("some error".to_string()));
				}
				return Ok(notification);
			})
			.await;

		assert_eq!(result.err().unwrap(), ServiceError::internal("notification error: some error"));

		let mut tx = pool.begin().await.unwrap();
		let count = count_all(&mut tx).await;
		assert_eq!(count, 3);
	})
		.await
}

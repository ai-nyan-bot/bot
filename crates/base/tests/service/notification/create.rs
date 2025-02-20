// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod condition_matched {
	use base::model::{TelegramButtonConfig, Value};
	use base::repo::NotificationRepo;
	use base::service::{NotificationConditionMatched, NotificationService};
	use sqlx::types::JsonValue;
	use testing::notification::count_all;
	use testing::run_test_with_pool_on_empty_db;
	use testing::user::get_or_create_test_user;


	#[test_log::test(sqlx::test)]
	async fn test_ok() {
		run_test_with_pool_on_empty_db(|pool| async move {
			let mut tx = pool.begin().await.unwrap();
			let test_user = get_or_create_test_user(&mut tx).await;
			let _ = tx.commit().await.unwrap();

			let test_instance = NotificationService::new(pool.clone(), NotificationRepo::new());
			test_instance
				.create_condition_matched(NotificationConditionMatched::Telegram {
					user: test_user.id,
					token_pair: 234.into(),
					buttons: vec![
						TelegramButtonConfig::None,
						TelegramButtonConfig::Buy {
							value: Value::Sol(1.2)
						},
						TelegramButtonConfig::Sell {
							value: Value::Percent(3.4)
						}
					],
				})
				.await
				.unwrap();

			let mut tx = pool.begin().await.unwrap();
			let count = count_all(&mut tx).await;
			assert_eq!(count, 1);

			let _ = test_instance.pop(1, |notification| async move {
				assert_eq!(*notification.payload.get("token_pair").unwrap(), JsonValue::String("234".to_string()));
				Ok(())
			}).await.unwrap();
		})
			.await
	}
}

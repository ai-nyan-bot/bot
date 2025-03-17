// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::env;
use dotenv::dotenv;
use sqlx::types::JsonValue;
use base::model::{Notification, NotificationChannel, NotificationPayload, NotificationType, TokenPairId};
use common::ConfigValue;
use common::model::CreatedAt;
use common::repo::pool::PostgresConfig;
use telegram::{send_notification, AppState, Config, TelegramConfig, WalletConfig};

#[tokio::main]
pub async fn main() {
	dotenv().ok();

	let token_pair_id = TokenPairId::from(1109);

	send_notification(
		AppState::setup(Config {
			telegram: TelegramConfig {
				token: ConfigValue::Value(
					env::var("TEST_TELEGRAM_TOKEN").expect("TEST_TELEGRAM_TOKEN must be set"),
				),
				webapp_url: ConfigValue::Value("http://telegram.nyan.bot".to_string()),
			},
			postgres: PostgresConfig {
				connection_string: ConfigValue::Value(
					"postgres://root:tor@localhost:5432/dev?sslmode=disable".to_string(),
				),
				pool_min: Default::default(),
				pool_max: Default::default(),
				timeout_acquire_ms: Default::default(),
			},
			wallet: WalletConfig { secret: Default::default() },
		})
			.await,
		Notification {
			id: 1.into(),
			user: 1.into(),
			channel: NotificationChannel::Telegram,
			ty: NotificationType::RuleMatched,
			payload: NotificationPayload(JsonValue::Object({
				let mut map = serde_json::Map::new();
				map.insert("venue".to_string(), JsonValue::String("PumpFun".to_string()));
				map.insert("token_pair".to_string(), JsonValue::Number(token_pair_id.0.into()));
				map
			})),
			created_at: CreatedAt::now(),
		},
	)
		.await
		.unwrap();
}

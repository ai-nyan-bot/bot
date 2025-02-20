// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{NotificationChannel, NotificationKind, NotificationPayload, UserId};
use crate::model::{TelegramButtonConfig, TokenPairId};
use crate::repo::NotificationCreateCmd;
use crate::service::notification::NotificationService;
use common::repo::Tx;
use common::service::ServiceResult;
use serde_json::Map;
use sqlx::types::JsonValue;

pub enum NotificationConditionMatched {
	Telegram {
		user: UserId,
		token_pair: TokenPairId,
		buttons: Vec<TelegramButtonConfig>,
	}
}

impl NotificationService {
	pub async fn create_condition_matched(&self, notification: NotificationConditionMatched) -> ServiceResult<()> {
		let mut tx = self.pool.begin().await?;
		self.create_condition_matched_tx(&mut tx, notification).await?;
		tx.commit().await?;
		Ok(())
	}

	pub async fn create_condition_matched_tx<'a>(&self, tx: &mut Tx<'a>, notification: NotificationConditionMatched) -> ServiceResult<()> {
		match notification {
			NotificationConditionMatched::Telegram { user, token_pair, buttons } => {
				self.repo.create(
					tx,
					NotificationCreateCmd {
						user,
						kind: NotificationKind::ConditionMet,
						channel: NotificationChannel::Telegram,
						payload: NotificationPayload(JsonValue::Object({
							let mut map = Map::new();
							map.insert("token_pair".to_string(), JsonValue::String(token_pair.to_string()));
							map.insert("button_0".to_string(), serde_json::to_value(buttons.get(0).unwrap_or(&TelegramButtonConfig::None)).unwrap());
							map.insert("button_1".to_string(), serde_json::to_value(buttons.get(1).unwrap_or(&TelegramButtonConfig::None)).unwrap());
							map.insert("button_2".to_string(), serde_json::to_value(buttons.get(2).unwrap_or(&TelegramButtonConfig::None)).unwrap());
							map.insert("button_3".to_string(), serde_json::to_value(buttons.get(3).unwrap_or(&TelegramButtonConfig::None)).unwrap());
							map.insert("button_4".to_string(), serde_json::to_value(buttons.get(4).unwrap_or(&TelegramButtonConfig::None)).unwrap());
							map.insert("button_5".to_string(), serde_json::to_value(buttons.get(5).unwrap_or(&TelegramButtonConfig::None)).unwrap());
							map
						})),
					},
				).await?;
			}
		}
		Ok(())
	}
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{NotificationChannel, NotificationType, NotificationPayload, RuleId, UserId};
use crate::model::{TelegramActionButtonConfig, TokenPairId};
use crate::repo::NotificationCreateCmd;
use crate::service::notification::NotificationService;
use common::repo::Tx;
use common::service::ServiceResult;
use serde_json::Map;
use sqlx::types::JsonValue;

pub enum NotificationRuleMatched {
	Telegram {
		user: UserId,
		rule: RuleId,
		token_pair: TokenPairId,
		buttons: Vec<TelegramActionButtonConfig>,
	}
}

impl NotificationService {
	pub async fn create_rule_matched(&self, notification: NotificationRuleMatched) -> ServiceResult<()> {
		let mut tx = self.pool.begin().await?;
		self.create_rule_matched_tx(&mut tx, notification).await?;
		tx.commit().await?;
		Ok(())
	}

	pub async fn create_rule_matched_tx<'a>(&self, tx: &mut Tx<'a>, notification: NotificationRuleMatched) -> ServiceResult<()> {
		match notification {
			NotificationRuleMatched::Telegram { user, rule, token_pair, buttons } => {
				self.repo.create(
					tx,
					NotificationCreateCmd {
						user,
						ty: NotificationType::RuleMatched,
						channel: NotificationChannel::Telegram,
						payload: NotificationPayload(JsonValue::Object({
							let mut map = Map::new();
							map.insert("rule".to_string(), serde_json::to_value(rule)?);
							map.insert("token_pair".to_string(), serde_json::to_value(token_pair)?);
							map.insert("button_0".to_string(), serde_json::to_value(buttons.first().unwrap_or(&TelegramActionButtonConfig::None)).unwrap());
							map.insert("button_1".to_string(), serde_json::to_value(buttons.get(1).unwrap_or(&TelegramActionButtonConfig::None)).unwrap());
							map.insert("button_2".to_string(), serde_json::to_value(buttons.get(2).unwrap_or(&TelegramActionButtonConfig::None)).unwrap());
							map.insert("button_3".to_string(), serde_json::to_value(buttons.get(3).unwrap_or(&TelegramActionButtonConfig::None)).unwrap());
							map.insert("button_4".to_string(), serde_json::to_value(buttons.get(4).unwrap_or(&TelegramActionButtonConfig::None)).unwrap());
							map.insert("button_5".to_string(), serde_json::to_value(buttons.get(5).unwrap_or(&TelegramActionButtonConfig::None)).unwrap());
							map
						})),
					},
				).await?;
			}
		}
		Ok(())
	}
}

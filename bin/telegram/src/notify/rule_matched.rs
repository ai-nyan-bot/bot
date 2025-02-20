// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::AppState;
use base::model::{Notification, NotificationType, TokenPairId};
use base::service::NotificationResult;
use teloxide::payloads::SendMessageSetters;
use teloxide::requests::Requester;
use teloxide::types::{ChatId, InlineKeyboardButton, InlineKeyboardMarkup, Recipient, WebAppInfo};
use url::Url;

pub(crate) async fn rule_matched(state: AppState, notification: Notification) -> NotificationResult<()> {
	assert_eq!(notification.ty, NotificationType::RuleMatched);

	let user = state.user_service().get_by_id(notification.user).await?;

	if let Some(telegram_id) = user.telegram_id {
		let buttons = InlineKeyboardMarkup::new(vec![
			vec![InlineKeyboardButton::web_app("Buy 0.01", WebAppInfo { url: Url::parse("https://telegram.nyan.bot/rules").unwrap(), })],
			vec![InlineKeyboardButton::web_app("Sell 100%", WebAppInfo { url: Url::parse("https://telegram.nyan.bot/rules").unwrap(), })]
		]);

		let token_pair_id: TokenPairId = notification.payload("token_pair").unwrap();

		let x = state.bot.send_message(
			Recipient::Id(ChatId(telegram_id.0.parse::<i64>().unwrap())),
			format!("Condition met {token_pair_id}"),
		).reply_markup(buttons).await.unwrap();
	}

	dbg!(&notification);

	panic!()
}
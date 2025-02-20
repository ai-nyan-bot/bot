// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod rule_matched;

use crate::AppState;
use common::Signal;
use log::info;
use std::time::Duration;
use tokio::select;
use tokio::task::JoinHandle;
use base::model::NotificationType;
use crate::notify::rule_matched::rule_matched;

pub fn notify(state: AppState, mut signal: Signal) -> JoinHandle<()> {
	tokio::spawn(async move {
		let mut interval = tokio::time::interval(Duration::from_secs(1));
		loop {
			select! {
				_ = signal.recv() => {
					info!("Signal received");
					break;
				}
				_ = interval.tick() => {
					next_notifications(state.clone()).await
				}
			}
		}
	})
}

async fn next_notifications(state: AppState) {
	let _ = state.notification_service().pop(1, {
		let state = state.clone();
		move |notification| {
			let state = state.clone();
			async move {
				match notification.ty {
					NotificationType::RuleMatched => rule_matched(state, notification).await?,
				}
				Ok(())
			}
		}
	}).await;
}

// async fn next_notifications(state: AppState) {
// 	let _ = state.notification_service().pop(1, |notification| async move {
// 		match notification.ty {
// 			NotificationType::RuleMatched => rule_matched(state.clone(), notification).await
// 			// NotificationType::RuleMatched => {}
// 		}
// 		// let user = state.user_service().get_by_id(notification.user).await.unwrap();
// 		// if let Some(telegram_id) = user.telegram_id {
// 		// 	let token_pair_id = notification.payload.0.get("token_pair_id").unwrap().as_str().unwrap();
// 		//
// 		// 	let buttons = InlineKeyboardMarkup::new(vec![
// 		// 		vec![InlineKeyboardButton::web_app("Buy 0.01", WebAppInfo { url: Url::parse("https://telegram.nyan.bot/rules").unwrap(), })],
// 		// 		vec![InlineKeyboardButton::web_app("Sell 100%", WebAppInfo { url: Url::parse("https://telegram.nyan.bot/rules").unwrap(), })]
// 		// 	]);
// 		//
// 		// 	let x = notifier.send_message(
// 		// 		Recipient::Id(ChatId(telegram_id.0.parse::<i64>().unwrap())),
// 		// 		format!("Condition met {token_pair_id}"),
// 		// 	).reply_markup(buttons).await.unwrap();
// 
// 		Ok(())
// 	}).await;
// }
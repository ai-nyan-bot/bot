// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{Notification, TokenPairId};
use log::{info, trace};
use std::fmt::{Display, Formatter};
use std::time::Duration;
use telegram::{schema, AppState, Config, HandlerResult, MessageState, MyDialog};
use teloxide::types::{Recipient, WebAppInfo};
use teloxide::{
	dispatching::dialogue::InMemStorage,
	prelude::*,
	types::{InlineKeyboardButton, InlineKeyboardMarkup},
	utils::command::BotCommands,
};
use tokio::runtime::Builder;
use tokio::signal;
use tokio::signal::unix::{signal, SignalKind};
use tokio::time::sleep;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use url::Url;

#[derive(Debug)]
struct NotificationError(pub String);

impl Display for NotificationError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("notification error: {}", self.0))
	}
}

impl std::error::Error for NotificationError {}

fn main() {
	let config = Config::load();

	tracing_subscriber::registry().with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| format!("{}=trace", env!("CARGO_CRATE_NAME")).into())).with(tracing_subscriber::fmt::layer()).init();

	let runtime = Builder::new_current_thread().worker_threads(2).enable_all().build().unwrap();

	runtime.block_on(async {
		let bot = Bot::new(config.telegram.token.resolve());

		let notifier = bot.clone();

		let state = AppState::setup(config).await;

		let mut dispatcher = Dispatcher::builder(bot, schema()).dependencies(dptree::deps![InMemStorage::<MessageState>::new(), state.clone()]).build();

		let notification_state = state.clone();
		tokio::spawn(async move {
			loop {
				let result = notification_state.service.notification.pop(1, |notification| async move { Ok::<Notification, NotificationError>(notification) }).await.unwrap();

				for notification in result {
					trace!("Sent {notification:#?}");
					let user = notification_state.service.user.get_by_id(notification.user).await.unwrap();

					if let Some(telegram_id) = user.telegram_id {
						let token_pair_id = notification.payload.0.get("token_pair_id").unwrap().as_str().unwrap();

						let buttons = InlineKeyboardMarkup::new(vec![
							vec![InlineKeyboardButton::web_app("Buy 0.01", WebAppInfo { url: Url::parse("https://telegram.nyan.bot/rules").unwrap(), })],
							vec![InlineKeyboardButton::web_app("Sell 100%", WebAppInfo { url: Url::parse("https://telegram.nyan.bot/rules").unwrap(), })]
						]);

						let x = notifier.send_message(
							Recipient::Id(ChatId(telegram_id.0.parse::<i64>().unwrap())),
							format!("Condition met {token_pair_id}"),
						).reply_markup(buttons).await.unwrap();
						panic!("");
					}
				}
				sleep(Duration::from_millis(1_000)).await;
			}
		});

		let _ = tokio::spawn(async move {
			let mut sigterm = signal(SignalKind::terminate()).unwrap();

			let shutdown_signal = async {
				signal::ctrl_c().await.expect("Failed to listen for shutdown signal");
				info!("Shutdown signal received");
			};

			tokio::select! {
                _ = shutdown_signal => {
                    info!("Shutting down the bot...");
                    let _ = dispatcher.shutdown_token().shutdown().unwrap();
                }
                _ = sigterm.recv() => {
                    info!("Received SIGTERM. Cleaning up resources...");
                    let _ = dispatcher.shutdown_token().shutdown();
                }
                _ = dispatcher.dispatch() => {
                    info!("Dispatcher has finished");
                }
            }
		}).await;
	})
}

async fn start_dialog(bot: Bot, dialog: MyDialog, msg: Message) -> HandlerResult {
	bot.send_message(msg.chat.id, "Let's start! What's your full name?").await?;
	dialog.update(MessageState::ReceiveFullName).await?;
	Ok(())
}

async fn receive_full_name(bot: Bot, dialog: MyDialog, msg: Message) -> HandlerResult {
	match msg.text().map(ToOwned::to_owned) {
		Some(full_name) => {
			let products = ["Apple", "Banana", "Orange", "Potato"].map(|product| InlineKeyboardButton::callback(product, product));

			bot.send_message(msg.chat.id, "Select a product:").reply_markup(InlineKeyboardMarkup::new([products])).await?;

			dialog.update(MessageState::ReceiveProductChoice { full_name }).await?;
		}
		None => {
			bot.send_message(msg.chat.id, "Please, send me your full name.").await?;
		}
	}

	Ok(())
}

async fn receive_product_selection(
	bot: Bot,
	dialog: MyDialog,
	// full_name: String, // Available from `State::ReceiveProductChoice`.
	state: MessageState,
	q: CallbackQuery,
) -> HandlerResult {
	let MessageState::ReceiveProductChoice { full_name } = state else {
		panic!("")
	};

	if let Some(product) = &q.data {
		bot.send_message(dialog.chat_id(), format!("{full_name}, product '{product}' has been purchased successfully!")).await?;
		dialog.exit().await?;
	}

	Ok(())
}

// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

#![cfg_attr(not(debug_assertions), deny(warnings))]

use common::Signal;
use log::info;
use telegram::{run, AppState, Config};
use tokio::runtime::Builder;
use tokio::signal::unix::{signal, SignalKind};
use tokio::{select, signal};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

fn main() {
    let config = Config::load();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let runtime = Builder::new_current_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        //
        // let notifier = bot.clone();
        //
        let run_signal = Signal::new();

        let state = AppState::setup(config).await;

        let app_signal = run_signal.clone();
        let _ = tokio::spawn(async move {
            let mut sigterm = signal(SignalKind::terminate()).unwrap();

            let tokio_signal = async {
                signal::ctrl_c()
                    .await
                    .expect("Failed to listen for shutdown signal");
                info!("Shutdown signal received");
            };

            select! {
                _ = tokio_signal => {
                    info!("Shutting down the bot...");
                    app_signal.shutdown();
                    // let _ = dispatcher.shutdown_token().shutdown().unwrap();
                }
                _ = sigterm.recv() => {
                    info!("Received SIGTERM. Cleaning up resources...");
                    app_signal.terminate("SIGTERM");
                    // let _ = dispatcher.shutdown_token().shutdown();
                }
                // _ = dispatcher.dispatch() => {
                //     info!("Dispatcher has finished");
                // }
            }
        });

        run(state, run_signal).await;

        // let mut dispatcher = Dispatcher::builder(bot, schema()).dependencies(dptree::deps![InMemStorage::<MessageState>::new(), state.clone()]).build();

        // let notification_state = state.clone();
        // tokio::spawn(async move {
        // 	loop {
        // 		let result = notification_state.service.notification.pop(1, |notification| async move { Ok(notification) }).await.unwrap();
        //
        // 		for notification in result {
        // 			trace!("Sent {notification:#?}");
        // 			let user = notification_state.service.user.get_by_id(notification.user).await.unwrap();
        //
        // 			if let Some(telegram_id) = user.telegram_id {
        // 				let token_pair_id = notification.payload.0.get("token_pair_id").unwrap().as_str().unwrap();
        //
        // 				let buttons = InlineKeyboardMarkup::new(vec![
        // 					vec![InlineKeyboardButton::web_app("Buy 0.01", WebAppInfo { url: Url::parse("https://telegram.nyan.bot/rules").unwrap(), })],
        // 					vec![InlineKeyboardButton::web_app("Sell 100%", WebAppInfo { url: Url::parse("https://telegram.nyan.bot/rules").unwrap(), })]
        // 				]);
        //
        // 				let x = notifier.send_message(
        // 					Recipient::Id(ChatId(telegram_id.0.parse::<i64>().unwrap())),
        // 					format!("Condition met {token_pair_id}"),
        // 				).reply_markup(buttons).await.unwrap();
        // 				panic!("");
        // 			}
        // 		}
        // 		sleep(Duration::from_millis(1_000)).await;
        // 	}
        // });

        // let _ = tokio::spawn(async move {
        // 	let mut sigterm = signal(SignalKind::terminate()).unwrap();
        //
        // 	let shutdown_signal = async {
        // 		signal::ctrl_c().await.expect("Failed to listen for shutdown signal");
        // 		info!("Shutdown signal received");
        // 	};
        //
        // 	tokio::select! {
        //         _ = shutdown_signal => {
        //             info!("Shutting down the bot...");
        //             let _ = dispatcher.shutdown_token().shutdown().unwrap();
        //         }
        //         _ = sigterm.recv() => {
        //             info!("Received SIGTERM. Cleaning up resources...");
        //             let _ = dispatcher.shutdown_token().shutdown();
        //         }
        //         _ = dispatcher.dispatch() => {
        //             info!("Dispatcher has finished");
        //         }
        //     }
        // })
    })
}

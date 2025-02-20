// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use log::info;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::Dispatcher;
use teloxide::dptree;
use tokio::select;
use crate::{schema, AppState, MessageState};
use common::Signal;
use tokio::task::JoinHandle;

pub fn dispatch(state: AppState, mut signal: Signal) -> JoinHandle<()> {
	tokio::spawn(async move {
		let mut dispatcher = Dispatcher::builder(state.bot.clone(), schema())
			.dependencies(dptree::deps![InMemStorage::<MessageState>::new(), state.clone()])
			.build();
			
		select! {
			_ = signal.recv() => {
				info!("Signal received");
				let _ = dispatcher.shutdown_token().shutdown();
			}
			_ = dispatcher.dispatch() =>{
				info!("Dispatcher finished");
				signal.terminate("Dispatcher finished");
			}
		}	
	})
}
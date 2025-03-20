// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

pub use crate::config::*;
use crate::dispatch::dispatch;
pub use crate::notify::{notify, send_notification};
pub use crate::schema::schema;
pub use crate::state::*;
use common::Signal;
use log::info;
use tokio::try_join;

mod callback;
mod command;
mod config;
mod dispatch;
mod format;
mod i18n;
mod message;
mod notify;
mod schema;
mod state;

#[derive(Clone, Default)]
pub enum MessageState {
    #[default]
    Main,
}

pub async fn run(state: AppState, signal: Signal) {
    let _ = try_join!(
        async { notify(state.clone(), signal.clone()).await },
        async { dispatch(state.clone(), signal.clone()).await }
    );
    info!("all tasks have been stopped, exiting...");
}

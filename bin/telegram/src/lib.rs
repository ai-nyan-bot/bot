// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

pub use crate::config::*;
pub use crate::schema::schema;
pub use crate::state::*;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::Dialogue;

mod callback;
mod command;
mod config;
mod i18n;
mod message;
mod schema;
mod state;

pub type MyDialog = Dialogue<MessageState, InMemStorage<MessageState>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum MessageState {
    #[default]
    Main,

    ReceiveFullName,
    ReceiveProductChoice {
        full_name: String,
    },
}

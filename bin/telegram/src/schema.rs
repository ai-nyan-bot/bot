// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::callback::callback;
use crate::{command, MessageState};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{dialogue, UpdateFilterExt, UpdateHandler};
use teloxide::dptree::case;
use teloxide::macros::BotCommands;
use teloxide::prelude::Update;
use teloxide::{dptree, filter_command};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    Balance,
    Cancel,
    Help,
    Rules,
    Start,
    Token,
    Wallet,
}

pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = filter_command::<Command, _>()
        .branch(
            case![MessageState::Main]
                .branch(case![Command::Help].endpoint(command::help))
                .branch(case![Command::Rules].endpoint(command::rules))
                .branch(case![Command::Start].endpoint(command::start))
                .branch(case![Command::Balance].endpoint(command::balance))
                .branch(case![Command::Wallet].endpoint(command::wallet))
                .branch(case![Command::Token].endpoint(command::token)),
        );

    let message_handler = Update::filter_message().branch(command_handler);

    let callback_query_handler = Update::filter_callback_query().endpoint(callback);

    dialogue::enter::<Update, InMemStorage<MessageState>, MessageState, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}

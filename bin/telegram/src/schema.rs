// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::{callback, command, message, MessageState};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{dialogue, UpdateFilterExt, UpdateHandler};
use teloxide::dptree::case;
use teloxide::macros::BotCommands;
use teloxide::prelude::Update;
use teloxide::{dptree, filter_command};
use crate::callback::callback;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    Balance,
    Cancel,
    Help,
    Start,
    Wallet,
}

pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = filter_command::<Command, _>()
        .branch(
            case![MessageState::Main]
                .branch(case![Command::Help].endpoint(command::help))
                .branch(case![Command::Start].endpoint(command::start))
                .branch(case![Command::Balance].endpoint(command::balance))
                .branch(case![Command::Wallet].endpoint(command::wallet)),
        )
        .branch(case![Command::Cancel].endpoint(command::cancel));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        // .branch(case![MessageState::ReceiveFullName].endpoint(receive_full_name))
        .branch(dptree::endpoint(message::invalid));

    let callback_query_handler = Update::filter_callback_query().endpoint(callback);
    // .branch(case![MessageState::ReceiveProductChoice { full_name }]
    //         .endpoint(receive_product_selection),
    // );

    dialogue::enter::<Update, InMemStorage<MessageState>, MessageState, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}
